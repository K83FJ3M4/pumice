use crate::{Format, ImageCreateFlags, ImageLayout, ImageTiling, ImageType, ImageUsageFlags, SampleCountFlags, SharingMode, StructureType};
use super::Extent3D;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct ImageCreateInfo<'a> {
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: &'a [u32],
    pub initial_layout: ImageLayout
}

#[repr(C)]
pub(crate) struct RawImageCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: ImageCreateFlags,
    pub(crate) image_type: ImageType,
    pub(crate) format: Format,
    pub(crate) extent: Extent3D,
    pub(crate) mip_levels: u32,
    pub(crate) array_layers: u32,
    pub(crate) samples: SampleCountFlags,
    pub(crate) tiling: ImageTiling,
    pub(crate) usage: ImageUsageFlags,
    pub(crate) sharing_mode: SharingMode,
    pub(crate) queue_family_index_count: u32,
    pub(crate) p_queue_family_indices: *const u32,
    pub(crate) initial_layout: ImageLayout
}

impl<'a> ImageCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawImageCreateInfo {
        RawImageCreateInfo {
            s_type: StructureType::ImageCreateInfo,
            p_next: null(),
            flags: self.flags,
            image_type: self.image_type,
            format: self.format,
            extent: self.extent,
            mip_levels: self.mip_levels,
            array_layers: self.array_layers,
            samples: self.samples,
            tiling: self.tiling,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            queue_family_index_count: self.queue_family_indices.len() as u32,
            p_queue_family_indices: self.queue_family_indices.as_ptr(),
            initial_layout: self.initial_layout
        }
    }
}