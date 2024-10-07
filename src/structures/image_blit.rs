use super::{ImageSubresourceLayers, Offset3D};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2]
}