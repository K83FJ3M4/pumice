use super::ClearDepthStencilValue;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClearValue {
    pub color: [f32; 4],
    pub depth_stencil: ClearDepthStencilValue
}