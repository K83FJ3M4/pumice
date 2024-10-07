use crate::{AllocationCallbacks, BufferView, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_buffer_view(
        self,
        buffer_view: BufferView
    ) {
        vk_destroy_buffer_view(self, buffer_view, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyBufferView"]
    pub fn vk_destroy_buffer_view(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    );
}