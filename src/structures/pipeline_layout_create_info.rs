use crate::{DescriptorSetLayout, PipelineLayoutCreateFlags, StructureType};
use super::PushConstantRange;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineLayoutCreateInfo<'a> {
    pub set_layouts: &'a [DescriptorSetLayout],
    pub push_constant_ranges: &'a [PushConstantRange]
}

#[repr(C)]
pub(crate) struct RawPipelineLayoutCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineLayoutCreateFlags,
    pub(crate) set_layout_count: u32,
    pub(crate) p_set_layouts: *const DescriptorSetLayout,
    pub(crate) push_constant_range_count: u32,
    pub(crate) p_push_constant_ranges: *const PushConstantRange
}

impl<'a> PipelineLayoutCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineLayoutCreateInfo {
        RawPipelineLayoutCreateInfo {
            s_type: StructureType::PipelineLayoutCreateInfo,
            p_next: null(),
            flags: PipelineLayoutCreateFlags::empty(),
            set_layout_count: self.set_layouts.len() as u32,
            p_set_layouts: self.set_layouts.as_ptr(),
            push_constant_range_count: self.push_constant_ranges.len() as u32,
            p_push_constant_ranges: self.push_constant_ranges.as_ptr()
        }
    }
}