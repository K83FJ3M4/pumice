use crate::{AllocationCallbacks, Device, Event, EventCreateFlags, RawEventCreateInfo, StructureType, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_event(self) -> Result<Event> {
        let mut event = Event::null();
        let create_info = RawEventCreateInfo {
            s_type: StructureType::EventCreateInfo,
            p_next: null(),
            flags: EventCreateFlags::empty()
        };

        match vk_create_event(
            self,
            &create_info,
            std::ptr::null(),
            &mut event
        ) {
            VkResult::Success => Ok(event),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateEvent"]
    fn vk_create_event(
        device: Device,
        p_create_info: *const RawEventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *mut Event
    ) -> VkResult;
}