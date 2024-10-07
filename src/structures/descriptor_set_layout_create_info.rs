use crate::{DescriptorSetLayoutCreateFlags, StructureType};
use super::{DescriptorSetLayoutBinding, RawDescriptorSetLayoutBinding};
use std::ffi::c_void;
use bumpalo::Bump;

#[derive(Clone, Copy)]
pub struct DescriptorSetLayoutCreateInfo<'a> {
    pub bindings: &'a [DescriptorSetLayoutBinding<'a>]
}
#[repr(C)]
pub(crate) struct RawDescriptorSetLayoutCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: DescriptorSetLayoutCreateFlags,
    pub(crate) binding_count: u32,
    pub(crate) p_bindings: *const RawDescriptorSetLayoutBinding
}

impl<'a> DescriptorSetLayoutCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawDescriptorSetLayoutCreateInfo {
        let bindings = bump.alloc_slice_fill_iter(
            self.bindings.into_iter().map(DescriptorSetLayoutBinding::into_raw)
        );

        RawDescriptorSetLayoutCreateInfo {
            s_type: StructureType::DescriptorSetLayoutCreateInfo,
            p_next: std::ptr::null(),
            flags: DescriptorSetLayoutCreateFlags::empty(),
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr()
        }
    }
}