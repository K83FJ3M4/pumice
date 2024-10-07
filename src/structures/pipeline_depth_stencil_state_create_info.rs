use crate::{Bool32, CompareOp, PipelineDepthStencilStateCreateFlags, StructureType, FALSE, TRUE};
use super::StencilOpState;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub depth_test_enable: bool,
    pub depth_write_enable: bool,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: bool,
    pub stencil_test_enable: bool,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32
}

#[repr(C)]
pub(crate) struct RawPipelineDepthStencilStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineDepthStencilStateCreateFlags,
    pub(crate) depth_test_enable: Bool32,
    pub(crate) depth_write_enable: Bool32,
    pub(crate) depth_compare_op: CompareOp,
    pub(crate) depth_bounds_test_enable: Bool32,
    pub(crate) stencil_test_enable: Bool32,
    pub(crate) front: StencilOpState,
    pub(crate) back: StencilOpState,
    pub(crate) min_depth_bounds: f32,
    pub(crate) max_depth_bounds: f32
}

impl PipelineDepthStencilStateCreateInfo {
    pub(crate) fn into_raw(&self) -> RawPipelineDepthStencilStateCreateInfo {
        RawPipelineDepthStencilStateCreateInfo {
            s_type: StructureType::PipelineDepthStencilStateCreateInfo,
            p_next: null(),
            flags: PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: if self.depth_test_enable { TRUE } else { FALSE },
            depth_write_enable: if self.depth_write_enable { TRUE } else { FALSE },
            depth_compare_op: self.depth_compare_op,
            depth_bounds_test_enable: if self.depth_bounds_test_enable { TRUE } else { FALSE },
            stencil_test_enable: if self.stencil_test_enable { TRUE } else { FALSE },
            front: self.front,
            back: self.back,
            min_depth_bounds: self.min_depth_bounds,
            max_depth_bounds: self.max_depth_bounds
        }
    }
}