use crate::{AllocationCallbacks, Device, Semaphore};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_semaphore(self, semaphore: Semaphore) {
        vk_destroy_semaphore(self, semaphore, null());
    }
}

extern "C" {
    #[link_name = "vkDestroySemaphore"]
    pub fn vk_destroy_semaphore(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    );
}