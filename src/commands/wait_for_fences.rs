use crate::{Bool32, Device, Fence, VkResult};
use super::Result;

impl Device {
    pub unsafe fn wait_for_fences(self, fences: &[Fence], wait_all: bool, timeout: u64) -> Result<bool> {
        match vk_wait_for_fences(
            self,
            fences.len() as u32,
            fences.as_ptr(),
            wait_all.into(),
            timeout
        ) {
            VkResult::Success => Ok(true),
            VkResult::Timeout => Ok(false),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkWaitForFences"]
    fn vk_wait_for_fences(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> VkResult;
}