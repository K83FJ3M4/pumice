use crate::{CommandPool, CommandPoolResetFlags, Device, VkResult};
use super::Result;

impl Device {
    pub unsafe fn reset_command_pool(
        self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result {
        match vk_reset_command_pool(self, command_pool, flags) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkResetCommandPool"]
    pub fn vk_reset_command_pool(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> VkResult;   
}