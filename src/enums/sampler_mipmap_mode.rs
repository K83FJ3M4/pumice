#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1,
}