use crate::{AccessFlags, Image, ImageLayout, StructureType};
use super::ImageSubresourceRange;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct ImageMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange
}

#[repr(C)]
pub(crate) struct RawImageMemoryBarrier {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) src_access_mask: AccessFlags,
    pub(crate) dst_access_mask: AccessFlags,
    pub(crate) old_layout: ImageLayout,
    pub(crate) new_layout: ImageLayout,
    pub(crate) src_queue_family_index: u32,
    pub(crate) dst_queue_family_index: u32,
    pub(crate) image: Image,
    pub(crate) subresource_range: ImageSubresourceRange
}

impl ImageMemoryBarrier {
    pub(crate) fn into_raw(&self) -> RawImageMemoryBarrier {
        RawImageMemoryBarrier {
            s_type: StructureType::ImageMemoryBarrier,
            p_next: null(),
            src_access_mask: self.src_access_mask,
            dst_access_mask: self.dst_access_mask,
            old_layout: self.old_layout,
            new_layout: self.new_layout,
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index: self.dst_queue_family_index,
            image: self.image,
            subresource_range: self.subresource_range
        }
    }
}