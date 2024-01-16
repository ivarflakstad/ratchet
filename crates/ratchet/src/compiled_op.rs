use crate::gpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutHandle, ComputePipelineHandle, GPUBuffer,
    GpuBindGroup, WgpuDevice, WorkgroupCount, UNIFORM_ALIGN,
};
use crate::{drvec, rvec, DRVec, RVec, Tensor};
use derive_new::new;
use wgpu::DynamicOffset;

//Compiled op represents a single kernel invocation
//TODO: We need to be more general here, enum with encoder.copy_buffer_to_buffer as a COPY
#[derive(Debug, new)]
pub struct CompiledOp {
    workgroup_count: WorkgroupCount,
    pipeline_handle: ComputePipelineHandle,
    storage_groups: RVec<GpuBindGroup>,
    offset: DynamicOffset, //offset into the metadata uniform buffer
}

impl CompiledOp {
    const MAX_BINDINGS_PER_GROUP: usize = 4;

    //TODO: dsts -> dst
    pub fn create_storage_bind_groups(
        srcs: &[&Tensor],
        dsts: &[&Tensor],
        bind_group_layouts: RVec<BindGroupLayoutHandle>,
        device: &WgpuDevice,
    ) -> RVec<GpuBindGroup> {
        let mut binding_counter: usize = 0;
        let mut bind_group_entries = drvec![];

        for tensor in srcs.iter().chain(dsts.iter()) {
            let buf = tensor.storage().try_read().unwrap();
            let gpu_buf = buf.try_gpu().unwrap();
            bind_group_entries.push(BindGroupEntry {
                handle: gpu_buf.handle,
                offset: 0,
                size: Some(gpu_buf.size().try_into().unwrap()),
            });
            binding_counter += 1;
        }

        let mut storage_groups = rvec![];
        for (group_index, bind_group_layout) in bind_group_layouts.iter().enumerate() {
            let group_range = Self::group_range(group_index, binding_counter);
            let entries = bind_group_entries[group_range].into();
            let layout = *bind_group_layout;

            let bind_group = device
                .get_or_create_bind_group(&BindGroupDescriptor { entries, layout })
                .unwrap();
            storage_groups.push(bind_group);
        }
        storage_groups
    }

    /// Determines which bindings belong to which bind group
    fn group_range(group_index: usize, binding_counter: usize) -> std::ops::Range<usize> {
        let group_end = usize::min(
            (group_index + 1) * Self::MAX_BINDINGS_PER_GROUP,
            binding_counter,
        );
        group_index * Self::MAX_BINDINGS_PER_GROUP..group_end
    }

    //TODO: pool this
    pub fn create_uniform_bind_group(
        device: &WgpuDevice,
        layout: &wgpu::BindGroupLayout,
        buf: &GPUBuffer,
    ) -> wgpu::BindGroup {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buf.inner,
                    offset: 0,
                    size: Some(std::num::NonZeroU64::new(UNIFORM_ALIGN as _).unwrap()),
                }),
            }],
        });
        bind_group
    }

    pub fn workgroup_count(&self) -> &WorkgroupCount {
        &self.workgroup_count
    }

    pub fn offset(&self) -> DynamicOffset {
        self.offset
    }

    pub fn storage_groups(&self) -> &RVec<GpuBindGroup> {
        &self.storage_groups
    }

    pub fn pipeline_handle(&self) -> ComputePipelineHandle {
        self.pipeline_handle
    }
}
