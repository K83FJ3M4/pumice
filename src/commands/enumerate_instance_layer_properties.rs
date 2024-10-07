use crate::{Instance, LayerProperties, VkResult};
use super::Result;

impl Instance {
    pub unsafe fn enumerate_layer_properties(self) -> Result<Vec<LayerProperties>> {
        let mut property_count = 0;
        let mut properties = Vec::new();

        match vk_enumerate_instance_layer_properties(
            &mut property_count,
            std::ptr::null_mut()
        ) {
            VkResult::Success => (),
            result => return Err(result),
        }

        properties.reserve(property_count as usize);
        properties.set_len(property_count as usize);

        match vk_enumerate_instance_layer_properties(
            &mut property_count,
            properties.as_mut_ptr()
        ) {
            VkResult::Success => Ok(properties),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkEnumerateInstanceLayerProperties"]
    pub fn vk_enumerate_instance_layer_properties(
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> VkResult;
}