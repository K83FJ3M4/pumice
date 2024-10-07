#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
}