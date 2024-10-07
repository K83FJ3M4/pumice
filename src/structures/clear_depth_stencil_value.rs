#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32
}