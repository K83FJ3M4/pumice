use crate::{Device, Event, VkResult};
use super::Result;

impl Device {
    pub unsafe fn set_event(self, event: Event) -> Result {
        match vk_set_event(self, event) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkSetEvent"]
    fn vk_set_event(
        device: Device,
        event: Event
    ) -> VkResult;
}