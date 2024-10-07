use super::{Extent3D, ImageSubresourceLayers, Offset3D};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D
}