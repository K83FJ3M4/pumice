use crate::{Device, Event, VkResult};
use super::Result;

impl Device {
    pub unsafe fn get_event_status(self, event: Event) -> Result<bool> {
        match vk_get_event_status(self, event) {
            VkResult::EventSet => Ok(true),
            VkResult::EventReset => Ok(false),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkGetEventStatus"]
    fn vk_get_event_status(
        device: Device,
        event: Event
    ) -> VkResult;
}