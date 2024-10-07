use crate::{AccessFlags, Buffer, DeviceSize, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct BufferMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize
}

#[repr(C)]
pub(crate) struct RawBufferMemoryBarrier {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) src_access_mask: AccessFlags,
    pub(crate) dst_access_mask: AccessFlags,
    pub(crate) src_queue_family_index: u32,
    pub(crate) dst_queue_family_index: u32,
    pub(crate) buffer: Buffer,
    pub(crate) offset: DeviceSize,
    pub(crate) size: DeviceSize
}

impl BufferMemoryBarrier {
    pub(crate) fn into_raw(&self) -> RawBufferMemoryBarrier {
        RawBufferMemoryBarrier {
            s_type: StructureType::BufferMemoryBarrier,
            p_next: null(),
            src_access_mask: self.src_access_mask,
            dst_access_mask: self.dst_access_mask,
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index: self.dst_queue_family_index,
            buffer: self.buffer,
            offset: self.offset,
            size: self.size
        }
    }
}