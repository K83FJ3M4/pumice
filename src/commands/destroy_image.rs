use crate::{AllocationCallbacks, Device, Image};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_image(self, image: Image) {
        vk_destroy_image(self, image, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyImage"]
    pub fn vk_destroy_image(
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks,
    );
}