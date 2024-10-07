#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ImageTiling {
    Optimal = 0,
    Linear = 1,
}