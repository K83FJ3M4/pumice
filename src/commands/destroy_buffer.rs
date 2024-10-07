use crate::{AllocationCallbacks, Buffer, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_buffer(
        self,
        buffer: Buffer
    ) {
        vk_destroy_buffer(self, buffer, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyBuffer"]
    pub fn vk_destroy_buffer(
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks,
    );
}