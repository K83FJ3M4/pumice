use crate::{PipelineCacheCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct PipelineCacheCreateInfo<'a> {
    pub initial_data: &'a [u8]
}

#[repr(C)]
pub(crate) struct RawPipelineCacheCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineCacheCreateFlags,
    pub(crate) initial_data_size: usize,
    pub(crate) p_initial_data: *const c_void
}

impl<'a> PipelineCacheCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineCacheCreateInfo {
        RawPipelineCacheCreateInfo {
            s_type: StructureType::PipelineCacheCreateInfo,
            p_next: null(),
            flags: PipelineCacheCreateFlags::empty(),
            initial_data_size: self.initial_data.len(),
            p_initial_data: self.initial_data.as_ptr() as *const c_void
        }
    }
}