use crate::DeviceSize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32
}