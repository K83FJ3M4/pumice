use crate::{FramebufferCreateFlags, ImageView, RenderPass, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct FramebufferCreateInfo<'a> {
    pub render_pass: RenderPass,
    pub attachments: &'a [ImageView],
    pub width: u32,
    pub height: u32,
    pub layers: u32
}

#[repr(C)]
pub(crate) struct RawFramebufferCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: FramebufferCreateFlags,
    pub(crate) render_pass: RenderPass,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const ImageView,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) layers: u32
}

impl<'a> FramebufferCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawFramebufferCreateInfo {
        RawFramebufferCreateInfo {
            s_type: StructureType::FramebufferCreateInfo,
            p_next: null(),
            flags: FramebufferCreateFlags::empty(),
            render_pass: self.render_pass,
            attachment_count: self.attachments.len() as u32,
            p_attachments: self.attachments.as_ptr(),
            width: self.width,
            height: self.height,
            layers: self.layers
        }
    }
}