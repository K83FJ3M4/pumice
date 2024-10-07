use crate::{Device, Event, VkResult};
use super::Result;

impl Device {
    pub unsafe fn reset_event(self, event: Event) -> Result {
        match vk_reset_event(self, event) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkResetEvent"]
    pub fn vk_reset_event(
        device: Device,
        event: Event,
    ) -> VkResult;
}