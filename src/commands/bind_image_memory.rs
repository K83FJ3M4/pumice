use crate::{Device, DeviceMemory, DeviceSize, Image, VkResult};
use super::Result;

impl Device {
    pub unsafe fn bind_image_memory(self, image: Image, memory: DeviceMemory, memory_offset: DeviceSize) -> Result {
        match vk_bind_image_memory(
            self,
            image,
            memory,
            memory_offset
        ) {
            VkResult::Success => Ok(()),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkBindImageMemory"]
    fn vk_bind_image_memory(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize
    ) -> VkResult;
}