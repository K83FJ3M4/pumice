use crate::{Pipeline, PipelineCreateFlags, PipelineLayout, RenderPass, StructureType};
use super::{PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, RawPipelineColorBlendStateCreateInfo, RawPipelineDepthStencilStateCreateInfo, RawPipelineDynamicStateCreateInfo, RawPipelineInputAssemblyStateCreateInfo, RawPipelineMultisampleStateCreateInfo, RawPipelineRasterizationStateCreateInfo, RawPipelineShaderStageCreateInfo, RawPipelineTessellationStateCreateInfo, RawPipelineVertexInputStateCreateInfo, RawPipelineViewportStateCreateInfo};
use std::ffi::c_void;
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct GraphicsPipelineCreateInfo<'a> {
    pub flags: PipelineCreateFlags,
    pub stages: &'a [PipelineShaderStageCreateInfo<'a>],
    pub vertex_input_state: Option<PipelineVertexInputStateCreateInfo<'a>>,
    pub input_assembly_state: Option<PipelineInputAssemblyStateCreateInfo>,
    pub tessellation_state: Option<PipelineTessellationStateCreateInfo>,
    pub viewport_state: Option<PipelineViewportStateCreateInfo<'a>>,
    pub rasterization_state: Option<PipelineRasterizationStateCreateInfo>,
    pub multisample_state: Option<PipelineMultisampleStateCreateInfo<'a>>,
    pub depth_stencil_state: Option<PipelineDepthStencilStateCreateInfo>,
    pub color_blend_state: Option<PipelineColorBlendStateCreateInfo<'a>>,
    pub dynamic_state: Option<PipelineDynamicStateCreateInfo<'a>>,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32
}

#[repr(C)]
pub(crate) struct RawGraphicsPipelineCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineCreateFlags,
    pub(crate) stage_count: u32,
    pub(crate) p_stages: *const RawPipelineShaderStageCreateInfo,
    pub(crate) p_vertex_input_state: *const RawPipelineVertexInputStateCreateInfo,
    pub(crate) p_input_assembly_state: *const RawPipelineInputAssemblyStateCreateInfo,
    pub(crate) p_tessellation_state: *const RawPipelineTessellationStateCreateInfo,
    pub(crate) p_viewport_state: *const RawPipelineViewportStateCreateInfo,
    pub(crate) p_rasterization_state: *const RawPipelineRasterizationStateCreateInfo,
    pub(crate) p_multisample_state: *const RawPipelineMultisampleStateCreateInfo,
    pub(crate) p_depth_stencil_state: *const RawPipelineDepthStencilStateCreateInfo,
    pub(crate) p_color_blend_state: *const RawPipelineColorBlendStateCreateInfo,
    pub(crate) p_dynamic_state: *const RawPipelineDynamicStateCreateInfo,
    pub(crate) layout: PipelineLayout,
    pub(crate) render_pass: RenderPass,
    pub(crate) subpass: u32,
    pub(crate) base_pipeline_handle: Pipeline,
    pub(crate) base_pipeline_index: i32
}

impl<'a> GraphicsPipelineCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawGraphicsPipelineCreateInfo {
        let stages = bump.alloc_slice_fill_iter(
            self.stages.into_iter().map(|stage| stage.into_raw(bump))
        );

        let p_vertex_input_state = match self.vertex_input_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_input_assembly_state = match self.input_assembly_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_tessellation_state = match self.tessellation_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_viewport_state = match self.viewport_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_rasterization_state = match self.rasterization_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_multisample_state = match self.multisample_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_depth_stencil_state = match self.depth_stencil_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_color_blend_state = match self.color_blend_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        let p_dynamic_state = match self.dynamic_state {
            Some(ref state) => bump.alloc(state.into_raw()),
            None => null()
        };

        RawGraphicsPipelineCreateInfo {
            s_type: StructureType::GraphicsPipelineCreateInfo,
            p_next: std::ptr::null(),
            flags: self.flags,
            stage_count: self.stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state,
            p_input_assembly_state,
            p_tessellation_state,
            p_viewport_state,
            p_rasterization_state,
            p_multisample_state,
            p_depth_stencil_state,
            p_color_blend_state,
            p_dynamic_state,
            layout: self.layout,
            render_pass: self.render_pass,
            subpass: self.subpass,
            base_pipeline_handle: self.base_pipeline_handle,
            base_pipeline_index: self.base_pipeline_index
        }
    }
}