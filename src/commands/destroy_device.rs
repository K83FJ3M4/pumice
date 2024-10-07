use crate::{AllocationCallbacks, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy(self) {
        vk_destroy_device(self, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyDevice"]
    pub fn vk_destroy_device(
        device: Device,
        p_allocator: *const AllocationCallbacks,
    );
}