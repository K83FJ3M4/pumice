use crate::ImageAspectFlags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32
}