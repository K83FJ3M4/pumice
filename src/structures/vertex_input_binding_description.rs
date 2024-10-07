use crate::VertexInputRate;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate
}