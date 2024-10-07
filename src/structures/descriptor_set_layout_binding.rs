use crate::{DescriptorType, Sampler, ShaderStageFlags};

#[derive(Clone, Copy)]
pub struct DescriptorSetLayoutBinding<'a> {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub immutable_samplers: &'a [Sampler]
}

#[repr(C)]
pub(crate) struct RawDescriptorSetLayoutBinding {
    pub(crate) binding: u32,
    pub(crate) descriptor_type: DescriptorType,
    pub(crate) descriptor_count: u32,
    pub(crate) stage_flags: ShaderStageFlags,
    pub(crate) p_immutable_samplers: *const Sampler
}

impl<'a> DescriptorSetLayoutBinding<'a> {
    pub(crate) fn into_raw(&self) -> RawDescriptorSetLayoutBinding {
        RawDescriptorSetLayoutBinding {
            binding: self.binding,
            descriptor_type: self.descriptor_type,
            descriptor_count: self.descriptor_count,
            stage_flags: self.stage_flags,
            p_immutable_samplers: self.immutable_samplers.as_ptr()
        }
    }
}