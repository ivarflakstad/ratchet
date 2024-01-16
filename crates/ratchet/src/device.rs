use crate::gpu::{AllocatorError, PoolError, WgpuDevice};

#[derive(Clone, Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Failed to acquire device with error: {0:?}")]
    DeviceAcquisitionFailed(#[from] wgpu::RequestDeviceError),
    #[error("Failed to get adapter.")]
    AdapterRequestFailed,
    #[error("Failed to create storage with error: {0:?}")]
    StorageCreationFailed(#[from] PoolError), //shouldn't be PoolError
    #[error("Device mismatch, requested device: {0:?}, actual device: {1:?}")]
    DeviceMismatch(String, String),
    #[error("Failed to allocate buffer with error: {0:?}")]
    BufferAllocationFailed(#[from] AllocatorError),
}

pub enum DeviceRequest {
    CPU,
    GPU,
}

#[derive(Clone, Default, PartialEq)]
pub enum Device {
    #[default]
    CPU,
    GPU(WgpuDevice),
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::CPU => write!(f, "CPU"),
            Device::GPU(gpu) => write!(f, "GPU:{}", gpu.ordinal()),
        }
    }
}

impl Device {
    pub fn request_device(request: DeviceRequest) -> Result<Self, DeviceError> {
        match request {
            DeviceRequest::CPU => Ok(Device::CPU),
            DeviceRequest::GPU => {
                let gpu = pollster::block_on(WgpuDevice::new())?;
                Ok(Device::GPU(gpu))
            }
        }
    }

    pub fn label(&self) -> String {
        format!("{:?}", self)
    }

    pub fn try_gpu(&self) -> Result<&WgpuDevice, DeviceError> {
        match self {
            Device::GPU(gpu) => Ok(gpu),
            Device::CPU => Err(DeviceError::DeviceMismatch(
                "CPU".to_string(),
                "GPU".to_string(),
            )),
        }
    }
}
