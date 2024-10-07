use crate::{Bool32, PipelineMultisampleStateCreateFlags, SampleCountFlags, SampleMask, StructureType, FALSE, TRUE};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineMultisampleStateCreateInfo<'a> {
    pub rasterization_samples: SampleCountFlags,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: &'a [SampleMask],
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool
}

#[repr(C)]
pub(crate) struct RawPipelineMultisampleStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineMultisampleStateCreateFlags,
    pub(crate) rasterization_samples: SampleCountFlags,
    pub(crate) sample_shading_enable: Bool32,
    pub(crate) min_sample_shading: f32,
    pub(crate) sample_mask: *const SampleMask,
    pub(crate) alpha_to_coverage_enable: Bool32,
    pub(crate) alpha_to_one_enable: Bool32
}

impl<'a> PipelineMultisampleStateCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineMultisampleStateCreateInfo {
        RawPipelineMultisampleStateCreateInfo {
            s_type: StructureType::PipelineMultisampleStateCreateInfo,
            p_next: null(),
            flags: PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: self.rasterization_samples,
            sample_shading_enable: if self.sample_shading_enable { TRUE } else { FALSE },
            min_sample_shading: self.min_sample_shading,
            sample_mask: self.sample_mask.as_ptr(),
            alpha_to_coverage_enable: if self.alpha_to_coverage_enable { TRUE } else { FALSE },
            alpha_to_one_enable: if self.alpha_to_one_enable { TRUE } else { FALSE }
        }
    }
}