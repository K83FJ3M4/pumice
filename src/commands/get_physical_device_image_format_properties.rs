use crate::{Format, ImageCreateFlags, ImageFormatProperties, ImageTiling, ImageType, ImageUsageFlags, PhysicalDevice, VkResult};
use std::mem::MaybeUninit;

impl PhysicalDevice {
    pub unsafe fn get_image_format_properties(
        self,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> ImageFormatProperties {
        let mut properties = MaybeUninit::uninit();
        vk_get_physical_device_image_format_properties(
            self,
            format,
            type_,
            tiling,
            usage,
            flags,
            properties.as_mut_ptr(),
        );
        properties.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceImageFormatProperties"]
    pub fn vk_get_physical_device_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        type_: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> VkResult;
}