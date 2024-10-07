use crate::{PipelineBindPoint, SubpassDescriptionFlags};
use super::AttachmentReference;
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct SubpassDescription<'a> {
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachments: &'a [AttachmentReference],
    pub color_attachments: &'a [AttachmentReference],
    pub resolve_attachments: &'a [AttachmentReference],
    pub depth_stencil_attachment: Option<AttachmentReference>,
    pub preserve_attachments: &'a [u32]
}

#[repr(C)]
pub(crate) struct RawSubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32
}

impl<'a> SubpassDescription<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawSubpassDescription {
        let p_depth_stencil_attachment = match self.depth_stencil_attachment {
            Some(ref attachment) => bump.alloc(*attachment),
            None => null()
        };

        RawSubpassDescription {
            flags: SubpassDescriptionFlags::empty(),
            pipeline_bind_point: self.pipeline_bind_point,
            input_attachment_count: self.input_attachments.len() as u32,
            p_input_attachments: self.input_attachments.as_ptr(),
            color_attachment_count: self.color_attachments.len() as u32,
            p_color_attachments: self.color_attachments.as_ptr(),
            p_resolve_attachments: self.resolve_attachments.as_ptr(),
            p_depth_stencil_attachment,
            preserve_attachment_count: self.preserve_attachments.len() as u32,
            p_preserve_attachments: self.preserve_attachments.as_ptr()
        }
    }
}