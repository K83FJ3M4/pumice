use crate::{AllocationCallbacks, DescriptorSetLayout, DescriptorSetLayoutCreateInfo, Device, RawDescriptorSetLayoutCreateInfo, VkResult};
use super::Result;
use std::ptr::null;
use bumpalo::Bump;

impl Device {
    pub unsafe fn create_descriptor_set_layout(self, create_info: &DescriptorSetLayoutCreateInfo) -> Result<DescriptorSetLayout> {
        let bump = Bump::new();
        let mut set_layout = DescriptorSetLayout::null();
        let create_info = create_info.into_raw(&bump);

        match vk_create_descriptor_set_layout(
            self,
            &create_info,
            null(),
            &mut set_layout
        ) {
            VkResult::Success => Ok(set_layout),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateDescriptorSetLayout"]
    fn vk_create_descriptor_set_layout(
        device: Device,
        p_create_info: *const RawDescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *mut DescriptorSetLayout
    ) -> VkResult;
}