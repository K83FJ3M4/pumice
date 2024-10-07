use crate::{CommandBuffer, CommandBufferResetFlags, VkResult};
use super::Result;

impl CommandBuffer {
    pub unsafe fn reset(self, flags: CommandBufferResetFlags) -> Result {
        match vk_reset_command_buffer(self, flags) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkResetCommandBuffer"]
    fn vk_reset_command_buffer(
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags
    ) -> VkResult;
}