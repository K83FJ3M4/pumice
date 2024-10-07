#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ImageViewType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
    Cube = 3,
    Type1DArray = 4,
    Type2DArray = 5,
    CubeArray = 6,
}