use crate::{Format, Image, ImageViewCreateFlags, ImageViewType, StructureType};
use super::{ComponentMapping, ImageSubresourceRange};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct ImageViewCreateInfo {
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange
}

#[repr(C)]
pub(crate) struct RawImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange
}

impl ImageViewCreateInfo {
    pub(crate) fn into_raw(&self) -> RawImageViewCreateInfo {
        RawImageViewCreateInfo {
            s_type: StructureType::ImageViewCreateInfo,
            p_next: null(),
            flags: ImageViewCreateFlags::empty(),
            image: self.image,
            view_type: self.view_type,
            format: self.format,
            components: self.components,
            subresource_range: self.subresource_range
        }
    }
}