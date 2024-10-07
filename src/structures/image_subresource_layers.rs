use crate::ImageAspectFlags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32
}