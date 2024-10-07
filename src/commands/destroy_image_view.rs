use crate::{AllocationCallbacks, Device, ImageView};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_image_view(self, image_view: ImageView) {
        vk_destroy_image_view(self, image_view, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyImageView"]
    pub fn vk_destroy_image_view(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    );
}