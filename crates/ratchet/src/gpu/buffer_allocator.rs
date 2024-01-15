use rustc_hash::FxHashMap;
use wgpu::BufferUsages;

use crate::{
    gpu::{BufferDescriptor, BufferPool, GPUBuffer, GpuBufferHandle},
    Tensor, TensorId,
};
use std::cell::{Ref, RefCell, RefMut};

use super::WgpuDevice;

pub struct BufferAllocator {
    //TODO: should this be RefCell
    pool: RefCell<BufferPool>,
}

impl BufferAllocator {
    pub fn new() -> Self {
        Self {
            pool: BufferPool::new().into(),
        }
    }

    pub fn begin_pass(&self, pass_index: u64) {
        self.pool.borrow_mut().begin_pass(pass_index);
    }

    pub fn get(&self, handle: GpuBufferHandle) -> GPUBuffer {
        self.pool.borrow().get(handle).unwrap()
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor, device: &WgpuDevice) -> GPUBuffer {
        self.pool.borrow_mut().allocate(desc, device)
    }

    pub fn pool(&self) -> Ref<BufferPool> {
        self.pool.borrow()
    }

    pub fn pool_mut(&self) -> RefMut<BufferPool> {
        self.pool.borrow_mut()
    }

    pub fn create_buffer_init(
        &self,
        desc: &BufferDescriptor,
        contents: &[u8],
        device: &WgpuDevice,
    ) -> GPUBuffer {
        let buf = self.pool.borrow_mut().allocate(desc, device);
        device.queue().write_buffer(&buf.inner, 0, contents);
        buf
    }

    //Specific allocation method for the graph
    fn graph_allocate(
        &self,
        descriptor: BufferDescriptor,
        free: &mut Vec<GPUBuffer>,
        device: &WgpuDevice,
    ) -> GPUBuffer {
        let mut closest_index = None;
        let mut closest_size_diff: Option<usize> = None;
        for (idx, b) in free.iter().enumerate() {
            let size = b.descriptor.size as usize;
            if size >= descriptor.size as usize {
                match closest_size_diff {
                    None => {
                        closest_index = Some(idx);
                        closest_size_diff = Some(usize::abs_diff(size, descriptor.size as _))
                    }
                    Some(d) if d > usize::abs_diff(size, descriptor.size as _) => {
                        closest_index = Some(idx);
                        closest_size_diff = Some(usize::abs_diff(size, descriptor.size as _))
                    }
                    _ => {}
                }
            }
        }

        match closest_index {
            Some(idx) => {
                if std::env::var("RATCHET_DEBUG").is_ok() {
                    self.create_buffer(&descriptor, device)
                } else {
                    free.remove(idx)
                }
            }
            None => self.create_buffer(&descriptor, device),
        }
    }

    /// # Graph memory allocation
    ///
    /// Simple greedy algorithm for allocating all required buffers to store
    /// activations during an inference pass.
    pub fn allocate_intermediates(
        &self,
        execution_order: &[Tensor],
        device: &WgpuDevice,
    ) -> FxHashMap<TensorId, GPUBuffer> {
        let mut free = Vec::new(); //TODO: switch to BTreeMap
        let mut assignments = FxHashMap::default();

        for t in execution_order {
            if t.resolved() {
                //TODO terrible
                t.storage().try_read().unwrap().raw().map(|b| match b {
                    crate::RawStorage::CPU(_) => todo!(),
                    crate::RawStorage::GPU(g) => assignments.insert(t.id(), g.inner().clone()),
                });

                continue;
            }

            for source in t.srcs() {
                //Here we should trace up once inplace is implemented
                let requested_bytes = source.num_bytes();
                assignments.entry(source.id()).or_insert_with(|| {
                    self.graph_allocate(
                        BufferDescriptor::new(
                            requested_bytes as _,
                            BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                            false,
                        ),
                        &mut free,
                        device,
                    )
                });
            }

            //release my buffer
            if let Some(buf) = assignments.get(&t.id()) {
                free.push(buf.clone());
            }
        }
        for (id, buf) in assignments.iter() {
            println!("{:#?}: {:#?}", id, buf);
        }
        assignments
    }
}