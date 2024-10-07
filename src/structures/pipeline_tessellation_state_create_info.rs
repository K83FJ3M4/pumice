use crate::{PipelineTessellationStateCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineTessellationStateCreateInfo {
    pub patch_control_points: u32
}

#[repr(C)]
pub(crate) struct RawPipelineTessellationStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineTessellationStateCreateFlags,
    pub(crate) patch_control_points: u32
}

impl PipelineTessellationStateCreateInfo {
    pub(crate) fn into_raw(&self) -> RawPipelineTessellationStateCreateInfo {
        RawPipelineTessellationStateCreateInfo {
            s_type: StructureType::PipelineTessellationStateCreateInfo,
            p_next: null(),
            flags: PipelineTessellationStateCreateFlags::empty(),
            patch_control_points: self.patch_control_points
        }
    }
}