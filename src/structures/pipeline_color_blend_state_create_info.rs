use crate::{Bool32, LogicOp, PipelineColorBlendStateCreateFlags, StructureType, FALSE, TRUE};
use super::PipelineColorBlendAttachmentState;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineColorBlendStateCreateInfo<'a> {
    pub logic_op_enable: bool,
    pub logic_op: LogicOp,
    pub attachments: &'a [PipelineColorBlendAttachmentState],
    pub blend_constants: [f32; 4]
}

#[repr(C)]
pub(crate) struct RawPipelineColorBlendStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineColorBlendStateCreateFlags,
    pub(crate) logic_op_enable: Bool32,
    pub(crate) logic_op: LogicOp,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const PipelineColorBlendAttachmentState,
    pub(crate) blend_constants: [f32; 4]
}

impl<'a> PipelineColorBlendStateCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineColorBlendStateCreateInfo {
        RawPipelineColorBlendStateCreateInfo {
            s_type: StructureType::PipelineColorBlendStateCreateInfo,
            p_next: null(),
            flags: PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: if self.logic_op_enable { TRUE } else { FALSE },
            logic_op: self.logic_op,
            attachment_count: self.attachments.len() as u32,
            p_attachments: self.attachments.as_ptr(),
            blend_constants: self.blend_constants
        }
    }
}