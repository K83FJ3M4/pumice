#[repr(C)]
#[derive(Clone, Copy)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}