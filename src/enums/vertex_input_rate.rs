#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1,
}