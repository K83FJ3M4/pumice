use crate::{Device, Event, VkResult};
use super::Result;

impl Device {
    pub unsafe fn get_event_status(self, event: Event) -> Result<VkResult> {
        match vk_get_event_status(self, event) {
            VkResult::EventSet => Ok(VkResult::EventSet),
            VkResult::EventReset => Ok(VkResult::EventReset),
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