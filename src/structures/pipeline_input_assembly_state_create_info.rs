use crate::{Bool32, PipelineInputAssemblyStateCreateFlags, PrimitiveTopology, StructureType, FALSE, TRUE};
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: bool
}

#[repr(C)]
pub(crate) struct RawPipelineInputAssemblyStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineInputAssemblyStateCreateFlags,
    pub(crate) topology: PrimitiveTopology,
    pub(crate) primitive_restart_enable: Bool32
}

impl PipelineInputAssemblyStateCreateInfo {
    pub(crate) fn into_raw(&self) -> RawPipelineInputAssemblyStateCreateInfo {
        RawPipelineInputAssemblyStateCreateInfo {
            s_type: StructureType::PipelineInputAssemblyStateCreateInfo,
            p_next: std::ptr::null(),
            flags: PipelineInputAssemblyStateCreateFlags::empty(),
            topology: self.topology,
            primitive_restart_enable: if self.primitive_restart_enable { TRUE } else { FALSE }
        }
    }
}