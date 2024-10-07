use crate::{Bool32, Framebuffer, QueryControlFlags, QueryPipelineStatisticFlags, RenderPass, StructureType, FALSE, TRUE};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct CommandBufferInheritanceInfo {
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: bool,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags
}

#[repr(C)]
pub(crate) struct RawCommandBufferInheritanceInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) render_pass: RenderPass,
    pub(crate) subpass: u32,
    pub(crate) framebuffer: Framebuffer,
    pub(crate) occlusion_query_enable: Bool32,
    pub(crate) query_flags: QueryControlFlags,
    pub(crate) pipeline_statistics: QueryPipelineStatisticFlags
}

impl CommandBufferInheritanceInfo {
    pub(crate) fn into_raw(&self) -> RawCommandBufferInheritanceInfo {
        RawCommandBufferInheritanceInfo {
            s_type: StructureType::CommandBufferInheritanceInfo,
            p_next: null(),
            render_pass: self.render_pass,
            subpass: self.subpass,
            framebuffer: self.framebuffer,
            occlusion_query_enable: if self.occlusion_query_enable { TRUE } else { FALSE },
            query_flags: self.query_flags,
            pipeline_statistics: self.pipeline_statistics
        }
    }
}