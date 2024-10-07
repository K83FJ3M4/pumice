use crate::{DeviceSize, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct MemoryAllocateInfo {
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32
}

#[repr(C)]
pub(crate) struct RawMemoryAllocateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) allocation_size: DeviceSize,
    pub(crate) memory_type_index: u32
}

impl MemoryAllocateInfo {
    pub(crate) fn into_raw(&self) -> RawMemoryAllocateInfo {
        RawMemoryAllocateInfo {
            s_type: StructureType::MemoryAllocateInfo,
            p_next: null(),
            allocation_size: self.allocation_size,
            memory_type_index: self.memory_type_index
        }
    }
}