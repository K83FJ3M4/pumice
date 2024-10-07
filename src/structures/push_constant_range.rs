use crate::ShaderStageFlags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32
}