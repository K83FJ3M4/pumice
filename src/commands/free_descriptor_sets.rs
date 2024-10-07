use crate::{DescriptorPool, DescriptorSet, Device, VkResult};
use super::Result;

impl Device {
    pub unsafe fn free_descriptor_sets(
        self,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> Result {
        match vk_free_descriptor_sets(
            self,
            descriptor_pool,
            descriptor_sets.len() as u32,
            descriptor_sets.as_ptr(),
        ) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkFreeDescriptorSets"]
    fn vk_free_descriptor_sets(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> VkResult;
}