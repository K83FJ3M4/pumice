use crate::{Device, Fence, VkResult};
use super::Result;

impl Device {
    pub unsafe fn reset_fences(
        self,
        fences: &[Fence]
    ) -> Result {
        match vk_reset_fences(self, fences.len() as u32, fences.as_ptr()) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkResetFences"]
    fn vk_reset_fences(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence
    ) -> VkResult;
}