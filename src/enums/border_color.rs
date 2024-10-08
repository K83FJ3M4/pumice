#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BorderColor {
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,
}
