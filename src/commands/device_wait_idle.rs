use crate::{Device, VkResult};
use super::Result;

impl Device {
    pub unsafe fn wait_idle(self) -> Result {
        match vk_device_wait_idle(self) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkDeviceWaitIdle"]
    pub fn vk_device_wait_idle(device: Device) -> VkResult;
}