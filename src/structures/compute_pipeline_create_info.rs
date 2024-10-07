use crate::{Pipeline, PipelineCreateFlags, PipelineLayout, StructureType};
use super::{PipelineShaderStageCreateInfo, RawPipelineShaderStageCreateInfo};
use std::ffi::c_void;
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct ComputePipelineCreateInfo<'a> {
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo<'a>,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32
}

#[repr(C)]
pub(crate) struct RawComputePipelineCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineCreateFlags,
    pub(crate) stage: RawPipelineShaderStageCreateInfo,
    pub(crate) layout: PipelineLayout,
    pub(crate) base_pipeline_handle: Pipeline,
    pub(crate) base_pipeline_index: i32
}

impl<'a> ComputePipelineCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawComputePipelineCreateInfo {
        RawComputePipelineCreateInfo {
            s_type: StructureType::ComputePipelineCreateInfo,
            p_next: null(),
            flags: self.flags,
            stage: self.stage.into_raw(bump),
            layout: self.layout,
            base_pipeline_handle: self.base_pipeline_handle,
            base_pipeline_index: self.base_pipeline_index
        }
    }
}