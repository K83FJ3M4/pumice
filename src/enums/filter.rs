#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Nearest = 0,
    Linear = 1,
}