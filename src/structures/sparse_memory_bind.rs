use crate::{DeviceMemory, DeviceSize, SparseMemoryBindFlags};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags
}