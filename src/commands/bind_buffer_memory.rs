use crate::{Buffer, Device, DeviceMemory, DeviceSize, VkResult};
use super::Result;

impl Device {
    pub unsafe fn bind_buffer_memory(self, buffer: Buffer, memory: DeviceMemory, memory_offset: DeviceSize) -> Result {
        match vk_bind_buffer_memory(
            self,
            buffer,
            memory,
            memory_offset
        ) {
            VkResult::Success => Ok(()),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkBindBufferMemory"]
    fn vk_bind_buffer_memory(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize
    ) -> VkResult;
}