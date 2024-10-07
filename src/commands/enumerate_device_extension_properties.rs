use crate::{str_into_raw, ExtensionProperties, PhysicalDevice, VkResult};
use std::ffi::c_char;
use super::Result;
use bumpalo::Bump;
use std::ptr::{null, null_mut};

impl PhysicalDevice {
    pub unsafe fn enumerate_extension_properties(self, layer_name: Option<&str>) -> Result<Vec<ExtensionProperties>> {
        let bump = Bump::new();
        let mut property_count = 0;
        let mut properties = Vec::new();
        let layer_name = match layer_name {
            Some(layer_name) => str_into_raw(layer_name, &bump),
            None => null(),
        };

        match vk_enumerate_device_extension_properties(
            self,
            layer_name,
            &mut property_count,
            null_mut()
        ) {
            VkResult::Success => (),
            result => return Err(result),
        }

        properties.reserve(property_count as usize);
        properties.set_len(property_count as usize);

        match vk_enumerate_device_extension_properties(
            self,
            layer_name,
            &mut property_count,
            properties.as_mut_ptr()
        ) {
            VkResult::Success => Ok(properties),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkEnumerateDeviceExtensionProperties"]
    fn vk_enumerate_device_extension_properties(
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties
    ) -> VkResult;
}