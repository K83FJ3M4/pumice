use crate::{CommandBuffer, VkResult};
use super::Result;

impl CommandBuffer {
    pub unsafe fn end(self) -> Result {
        match vk_end_command_buffer(self) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkEndCommandBuffer"]
    pub fn vk_end_command_buffer(command_buffer: CommandBuffer) -> VkResult;
}