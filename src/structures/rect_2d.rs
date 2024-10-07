use super::{Extent2D, Offset2D};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}