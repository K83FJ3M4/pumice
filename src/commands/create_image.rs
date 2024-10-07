use crate::{AllocationCallbacks, Device, Image, ImageCreateInfo, RawImageCreateInfo, VkResult};
use std::ptr::null;
use super::Result;

impl Device {
    pub unsafe fn create_image(
        self,
        create_info: &ImageCreateInfo
    ) -> Result<Image> {
        let mut image = Image::null();
        let create_info = create_info.into_raw();
        match vk_create_image(
            self,
            &create_info,
            null(),
            &mut image
        ) {
            VkResult::Success => Ok(image),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateImage"]
    fn vk_create_image(
        device: Device,
        create_info: *const RawImageCreateInfo,
        allocator: *const AllocationCallbacks,
        image: *mut Image
    ) -> VkResult;
}