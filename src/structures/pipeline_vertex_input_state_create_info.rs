use crate::{PipelineVertexInputStateCreateFlags, StructureType};
use super::{VertexInputAttributeDescription, VertexInputBindingDescription};
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct PipelineVertexInputStateCreateInfo<'a> {
    pub vertex_binding_descriptions: &'a [VertexInputBindingDescription],
    pub vertex_attribute_descriptions: &'a [VertexInputAttributeDescription]
}

#[repr(C)]
pub(crate) struct RawPipelineVertexInputStateCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: PipelineVertexInputStateCreateFlags,
    pub(crate) vertex_binding_description_count: u32,
    pub(crate) p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub(crate) vertex_attribute_description_count: u32,
    pub(crate) p_vertex_attribute_descriptions: *const VertexInputAttributeDescription
}

impl<'a> PipelineVertexInputStateCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawPipelineVertexInputStateCreateInfo {
        RawPipelineVertexInputStateCreateInfo {
            s_type: StructureType::PipelineVertexInputStateCreateInfo,
            p_next: std::ptr::null(),
            flags: PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: self.vertex_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: self.vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count: self.vertex_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: self.vertex_attribute_descriptions.as_ptr()
        }
    }
}