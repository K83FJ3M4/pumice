#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
}
