use crate::{RenderPassCreateFlags, StructureType};
use super::{AttachmentDescription, RawSubpassDescription, SubpassDependency, SubpassDescription};
use std::ffi::c_void;
use bumpalo::Bump;

#[derive(Clone, Copy)]
pub struct RenderPassCreateInfo<'a> {
    pub attachments: &'a [AttachmentDescription],
    pub subpasses: &'a [SubpassDescription<'a>],
    pub dependencies: &'a [SubpassDependency]
}

#[repr(C)]
pub(crate) struct RawRenderPassCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: RenderPassCreateFlags,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const AttachmentDescription,
    pub(crate) subpass_count: u32,
    pub(crate) p_subpasses: *const RawSubpassDescription,
    pub(crate) dependency_count: u32,
    pub(crate) p_dependencies: *const SubpassDependency
}

impl<'a> RenderPassCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawRenderPassCreateInfo {
        let subpasses = bump.alloc_slice_fill_iter(
            self.subpasses.into_iter().map(|subpass| subpass.into_raw(bump))
        );

        RawRenderPassCreateInfo {
            s_type: StructureType::RenderPassCreateInfo,
            p_next: std::ptr::null(),
            flags: RenderPassCreateFlags::empty(),
            attachment_count: self.attachments.len() as u32,
            p_attachments: self.attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: self.dependencies.len() as u32,
            p_dependencies: self.dependencies.as_ptr()
        }
    }
}