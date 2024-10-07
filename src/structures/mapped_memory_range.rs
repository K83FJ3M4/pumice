use crate::{DeviceMemory, DeviceSize, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct MappedMemoryRange {
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize
}

#[repr(C)]
pub(crate) struct RawMappedMemoryRange {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) memory: DeviceMemory,
    pub(crate) offset: DeviceSize,
    pub(crate) size: DeviceSize
}

impl MappedMemoryRange {
    pub(crate) fn into_raw(&self) -> RawMappedMemoryRange {
        RawMappedMemoryRange {
            s_type: StructureType::MappedMemoryRange,
            p_next: null(),
            memory: self.memory,
            offset: self.offset,
            size: self.size
        }
    }
}