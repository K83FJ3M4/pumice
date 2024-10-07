use crate::{Buffer, BufferViewCreateFlags, DeviceSize, Format, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct BufferViewCreateInfo {
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize
}

#[repr(C)]
pub(crate) struct RawBufferViewCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: BufferViewCreateFlags,
    pub(crate) buffer: Buffer,
    pub(crate) format: Format,
    pub(crate) offset: DeviceSize,
    pub(crate) range: DeviceSize
}

impl BufferViewCreateInfo {
    pub(crate) fn into_raw(&self) -> RawBufferViewCreateInfo {
        RawBufferViewCreateInfo {
            s_type: StructureType::BufferViewCreateInfo,
            p_next: null(),
            flags: BufferViewCreateFlags::empty(),
            buffer: self.buffer,
            format: self.format,
            offset: self.offset,
            range: self.range
        }
    }
}