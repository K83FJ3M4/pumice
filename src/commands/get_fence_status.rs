use crate::{Device, Fence, VkResult};
use super::Result;

impl Device {
    pub unsafe fn get_fence_status(self, fence: Fence) -> Result<bool> {
        match vk_get_fence_status(self, fence) {
            VkResult::Success => Ok(true),
            VkResult::NotReady => Ok(false),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkGetFenceStatus"]
    fn vk_get_fence_status(
        device: Device,
        fence: Fence
    ) -> VkResult;
}