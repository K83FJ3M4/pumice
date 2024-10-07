use super::Rect2D;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}