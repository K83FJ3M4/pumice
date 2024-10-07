use crate::DeviceSize;
use super::{ImageSubresourceLayers, Offset3D};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Offset3D
}