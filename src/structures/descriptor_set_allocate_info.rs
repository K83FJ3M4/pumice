use crate::{DescriptorPool, DescriptorSetLayout, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct DescriptorSetAllocateInfo<'a> {
    pub descriptor_pool: DescriptorPool,
    pub set_layouts: &'a [DescriptorSetLayout]
}

#[repr(C)]
pub(crate) struct RawDescriptorSetAllocateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) descriptor_pool: DescriptorPool,
    pub(crate) descriptor_set_count: u32,
    pub(crate) p_set_layouts: *const DescriptorSetLayout
}

impl<'a> DescriptorSetAllocateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawDescriptorSetAllocateInfo {
        RawDescriptorSetAllocateInfo {
            s_type: StructureType::DescriptorSetAllocateInfo,
            p_next: null(),
            descriptor_pool: self.descriptor_pool,
            descriptor_set_count: self.set_layouts.len() as u32,
            p_set_layouts: self.set_layouts.as_ptr()
        }
    }
}