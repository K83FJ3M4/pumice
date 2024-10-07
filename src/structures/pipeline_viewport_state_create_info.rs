use crate::{PipelineViewportStateCreateFlags, StructureType};
use super::{Rect2D, Viewport};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineViewportStateCreateInfo<'a> {
    pub viewports: &'a [Viewport],
    pub scissors: &'a [Rect2D]
}

#[repr(C)]
pub(crate) struct RawPipelineViewportStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineViewportStateCreateFlags,
    pub(crate) viewport_count: u32,
    pub(crate) p_viewports: *const Viewport,
    pub(crate) scissor_count: u32,
    pub(crate) p_scissors: *const Rect2D
}

impl<'a> PipelineViewportStateCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineViewportStateCreateInfo {
        RawPipelineViewportStateCreateInfo {
            s_type: StructureType::PipelineViewportStateCreateInfo,
            p_next: null(),
            flags: PipelineViewportStateCreateFlags::empty(),
            viewport_count: self.viewports.len() as u32,
            p_viewports: self.viewports.as_ptr(),
            scissor_count: self.scissors.len() as u32,
            p_scissors: self.scissors.as_ptr()
        }
    }
}