use crate::{DynamicState, PipelineDynamicStateCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineDynamicStateCreateInfo<'a> {
    pub dynamic_states: &'a [DynamicState]
}

#[repr(C)]
pub(crate) struct RawPipelineDynamicStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineDynamicStateCreateFlags,
    pub(crate) dynamic_state_count: u32,
    pub(crate) p_dynamic_states: *const DynamicState
}

impl<'a> PipelineDynamicStateCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineDynamicStateCreateInfo {
        RawPipelineDynamicStateCreateInfo {
            s_type: StructureType::PipelineDynamicStateCreateInfo,
            p_next: null(),
            flags: PipelineDynamicStateCreateFlags::empty(),
            dynamic_state_count: self.dynamic_states.len() as u32,
            p_dynamic_states: self.dynamic_states.as_ptr()
        }
    }
}