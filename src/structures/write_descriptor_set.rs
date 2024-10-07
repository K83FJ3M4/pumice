use crate::{BufferView, DescriptorSet, DescriptorType, StructureType};
use super::{DescriptorBufferInfo, DescriptorImageInfo};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct WriteDescriptorSet<'a> {
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub image_info: &'a [DescriptorImageInfo],
    pub buffer_info: &'a [DescriptorBufferInfo],
    pub texel_buffer_view: &'a [BufferView]
}

#[repr(C)]
pub(crate) struct RawWriteDescriptorSet {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) dst_set: DescriptorSet,
    pub(crate) dst_binding: u32,
    pub(crate) dst_array_element: u32,
    pub(crate) descriptor_count: u32,
    pub(crate) descriptor_type: DescriptorType,
    pub(crate) p_image_info: *const DescriptorImageInfo,
    pub(crate) p_buffer_info: *const DescriptorBufferInfo,
    pub(crate) p_texel_buffer_view: *const BufferView
}

impl WriteDescriptorSet<'_> {
    pub(crate) fn into_raw(&self) -> RawWriteDescriptorSet {
        RawWriteDescriptorSet {
            s_type: StructureType::WriteDescriptorSet,
            p_next: null(),
            dst_set: self.dst_set,
            dst_binding: self.dst_binding,
            dst_array_element: self.dst_array_element,
            descriptor_count: self.descriptor_count,
            descriptor_type: self.descriptor_type,
            p_image_info: self.image_info.as_ptr(),
            p_buffer_info: self.buffer_info.as_ptr(),
            p_texel_buffer_view: self.texel_buffer_view.as_ptr()
        }
    }
}