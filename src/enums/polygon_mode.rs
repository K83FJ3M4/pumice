#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
}