use crate::{AllocationCallbacks, Device, Fence};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_fence(self, fence: Fence) {
        vk_destroy_fence(self, fence, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyFence"]
    pub fn vk_destroy_fence(
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks,
    );
}