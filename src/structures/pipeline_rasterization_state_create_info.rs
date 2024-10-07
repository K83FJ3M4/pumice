use crate::{Bool32, CullModeFlags, FrontFace, PipelineRasterizationStateCreateFlags, PolygonMode, StructureType, FALSE, TRUE};
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct PipelineRasterizationStateCreateInfo {
    pub depth_clamp_enable: bool,
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32
}

#[repr(C)]
pub(crate) struct RawPipelineRasterizationStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineRasterizationStateCreateFlags,
    pub(crate) depth_clamp_enable: Bool32,
    pub(crate) rasterizer_discard_enable: Bool32,
    pub(crate) polygon_mode: PolygonMode,
    pub(crate) cull_mode: CullModeFlags,
    pub(crate) front_face: FrontFace,
    pub(crate) depth_bias_enable: Bool32,
    pub(crate) depth_bias_constant_factor: f32,
    pub(crate) depth_bias_clamp: f32,
    pub(crate) depth_bias_slope_factor: f32,
    pub(crate) line_width: f32
}

impl PipelineRasterizationStateCreateInfo {
    pub(crate) fn into_raw(&self) -> RawPipelineRasterizationStateCreateInfo {
        RawPipelineRasterizationStateCreateInfo {
            s_type: StructureType::PipelineRasterizationStateCreateInfo,
            p_next: std::ptr::null(),
            flags: PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: if self.depth_clamp_enable { TRUE } else { FALSE },
            rasterizer_discard_enable: if self.rasterizer_discard_enable { TRUE } else { FALSE },
            polygon_mode: self.polygon_mode,
            cull_mode: self.cull_mode,
            front_face: self.front_face,
            depth_bias_enable: if self.depth_bias_enable { TRUE } else { FALSE },
            depth_bias_constant_factor: self.depth_bias_constant_factor,
            depth_bias_clamp: self.depth_bias_clamp,
            depth_bias_slope_factor: self.depth_bias_slope_factor,
            line_width: self.line_width
        }
    }
}