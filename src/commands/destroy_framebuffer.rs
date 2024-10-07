use crate::{AllocationCallbacks, Device, Framebuffer};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_framebuffer(self, framebuffer: Framebuffer) {
        vk_destroy_framebuffer(self, framebuffer, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyFramebuffer"]
    pub fn vk_destroy_framebuffer(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    );
}