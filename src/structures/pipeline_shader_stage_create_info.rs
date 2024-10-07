use crate::{PipelineShaderStageCreateFlags, ShaderModule, ShaderStageFlags, StructureType};
use super::{str_into_raw, RawSpecializationInfo, SpecializationInfo};
use std::ffi::{c_char, c_void};
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineShaderStageCreateInfo<'a> {
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub name: &'a str,
    pub specialization_info: Option<SpecializationInfo<'a>>
}

#[repr(C)]
pub(crate) struct RawPipelineShaderStageCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineShaderStageCreateFlags,
    pub(crate) stage: ShaderStageFlags,
    pub(crate) module: ShaderModule,
    pub(crate) p_name: *const c_char,
    pub(crate) p_specialization_info: *const RawSpecializationInfo
}

impl<'a> PipelineShaderStageCreateInfo<'a> {
    pub(crate) fn into_raw(self, bump: &Bump) -> RawPipelineShaderStageCreateInfo {
        let p_specialization_info = match self.specialization_info {
            Some(specialization_info) => bump.alloc(specialization_info.into_raw()),
            None => null()
        };
        
        RawPipelineShaderStageCreateInfo {
            s_type: StructureType::PipelineShaderStageCreateInfo,
            p_next: null(),
            flags: PipelineShaderStageCreateFlags::empty(),
            stage: self.stage,
            module: self.module,
            p_name: str_into_raw(self.name, bump),
            p_specialization_info
        }
    }
}