use crate::{ExtensionProperties, Instance, VkResult};
use std::ffi::c_char;
use super::Result;

impl Instance {
    pub unsafe fn enumerate_extension_properties(self, layer_name: Option<&str>) -> Result<Vec<ExtensionProperties>> {
        let mut property_count = 0;
        let mut properties = Vec::new();
        let layer_name = match layer_name {
            Some(layer_name) => layer_name.as_ptr() as *const c_char,
            None => std::ptr::null(),
        };

        match vk_enumerate_instance_extension_properties(
            layer_name,
            &mut property_count,
            std::ptr::null_mut()
        ) {
            VkResult::Success => (),
            result => return Err(result),
        }

        properties.reserve(property_count as usize);
        properties.set_len(property_count as usize);

        match vk_enumerate_instance_extension_properties(
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
    #[link_name = "vkEnumerateInstanceExtensionProperties"]
    fn vk_enumerate_instance_extension_properties(
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties
    ) -> VkResult;
}