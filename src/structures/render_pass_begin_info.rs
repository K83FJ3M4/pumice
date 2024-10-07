use crate::{Framebuffer, RenderPass, StructureType};
use super::{ClearValue, Rect2D};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct RenderPassBeginInfo<'a> {
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_values: &'a [ClearValue]
}

#[repr(C)]
pub(crate) struct RawRenderPassBeginInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) render_pass: RenderPass,
    pub(crate) framebuffer: Framebuffer,
    pub(crate) render_area: Rect2D,
    pub(crate) clear_value_count: u32,
    pub(crate) p_clear_values: *const ClearValue
}

impl<'a> RenderPassBeginInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawRenderPassBeginInfo {
        RawRenderPassBeginInfo {
            s_type: StructureType::RenderPassBeginInfo,
            p_next: null(),
            render_pass: self.render_pass,
            framebuffer: self.framebuffer,
            render_area: self.render_area,
            clear_value_count: self.clear_values.len() as u32,
            p_clear_values: self.clear_values.as_ptr()
        }
    }
}