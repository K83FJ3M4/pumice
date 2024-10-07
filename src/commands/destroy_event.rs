use crate::{AllocationCallbacks, Device, Event};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_event(self, event: Event) {
        vk_destroy_event(self, event, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyEvent"]
    pub fn vk_destroy_event(
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks,
    );
}