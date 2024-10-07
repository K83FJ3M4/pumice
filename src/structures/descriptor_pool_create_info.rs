use crate::{DescriptorPoolCreateFlags, StructureType};
use super::DescriptorPoolSize;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct DescriptorPoolCreateInfo<'a> {
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_sizes: &'a [DescriptorPoolSize]
}

#[repr(C)]
pub(crate) struct RawDescriptorPoolCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: DescriptorPoolCreateFlags,
    pub(crate) max_sets: u32,
    pub(crate) pool_size_count: u32,
    pub(crate) p_pool_sizes: *const DescriptorPoolSize
}

impl<'a> DescriptorPoolCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawDescriptorPoolCreateInfo {
        RawDescriptorPoolCreateInfo {
            s_type: StructureType::DescriptorPoolCreateInfo,
            p_next: null(),
            flags: self.flags,
            max_sets: self.max_sets,
            pool_size_count: self.pool_sizes.len() as u32,
            p_pool_sizes: self.pool_sizes.as_ptr()
        }
    }
}