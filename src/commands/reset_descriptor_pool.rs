use crate::{DescriptorPool, DescriptorPoolResetFlags, Device, VkResult};
use super::Result;

impl Device {
    pub unsafe fn reset_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool,
    ) -> Result {
        match vk_reset_descriptor_pool(self, descriptor_pool, DescriptorPoolResetFlags::empty()) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkResetDescriptorPool"]
    fn vk_reset_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags
    ) -> VkResult;
}