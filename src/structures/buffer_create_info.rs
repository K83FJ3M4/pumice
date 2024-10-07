use crate::{BufferCreateFlags, BufferUsageFlags, DeviceSize, SharingMode, StructureType};
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct BufferCreateInfo<'a> {
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: &'a [u32]
}

#[repr(C)]
pub(crate) struct RawBufferCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: BufferCreateFlags,
    pub(crate) size: DeviceSize,
    pub(crate) usage: BufferUsageFlags,
    pub(crate) sharing_mode: SharingMode,
    pub(crate) queue_family_index_count: u32,
    pub(crate) p_queue_family_indices: *const u32
}

impl<'a> BufferCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawBufferCreateInfo {
        RawBufferCreateInfo {
            s_type: StructureType::BufferCreateInfo,
            p_next: std::ptr::null(),
            flags: self.flags,
            size: self.size,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            queue_family_index_count: self.queue_family_indices.len() as u32,
            p_queue_family_indices: self.queue_family_indices.as_ptr()
        }
    }
}