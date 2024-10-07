use crate::{AllocationCallbacks, Device, ImageView, ImageViewCreateInfo, RawImageViewCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_image_view(self, create_info: &ImageViewCreateInfo) -> Result<ImageView> {
        let mut image_view = ImageView::null();
        let create_info = create_info.into_raw();
        match vk_create_image_view(
            self,
            &create_info,
            null(),
            &mut image_view
        ) {
            VkResult::Success => Ok(image_view),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateImageView"]
    fn vk_create_image_view(
        device: Device,
        p_create_info: *const RawImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image_view: *mut ImageView
    ) -> VkResult;
}