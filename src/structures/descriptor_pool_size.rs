use crate::DescriptorType;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DescriptorPoolSize {
    pub r#type: DescriptorType,
    pub descriptor_count: u32
}