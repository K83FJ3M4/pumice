use crate::{DescriptorSet, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct CopyDescriptorSet {
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32
}

#[repr(C)]
pub(crate) struct RawCopyDescriptorSet {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) src_set: DescriptorSet,
    pub(crate) src_binding: u32,
    pub(crate) src_array_element: u32,
    pub(crate) dst_set: DescriptorSet,
    pub(crate) dst_binding: u32,
    pub(crate) dst_array_element: u32,
    pub(crate) descriptor_count: u32
}

impl CopyDescriptorSet {
    pub(crate) fn into_raw(&self) -> RawCopyDescriptorSet {
        RawCopyDescriptorSet {
            s_type: StructureType::CopyDescriptorSet,
            p_next: null(),
            src_set: self.src_set,
            src_binding: self.src_binding,
            src_array_element: self.src_array_element,
            dst_set: self.dst_set,
            dst_binding: self.dst_binding,
            dst_array_element: self.dst_array_element,
            descriptor_count: self.descriptor_count
        }
    }
}