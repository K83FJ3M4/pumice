use crate::{Format, FormatProperties, PhysicalDevice};
use std::mem::MaybeUninit;

impl PhysicalDevice {
    pub unsafe fn get_format_properties(self, format: Format) -> FormatProperties {
        let mut properties = MaybeUninit::uninit();
        vk_get_physical_device_format_properties(self, format, properties.as_mut_ptr());
        properties.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceFormatProperties"]
    pub fn vk_get_physical_device_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        p_properties: *mut FormatProperties,
    );
}