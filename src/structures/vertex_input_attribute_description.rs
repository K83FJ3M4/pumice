use crate::Format;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32
}