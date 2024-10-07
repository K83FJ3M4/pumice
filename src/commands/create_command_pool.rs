use crate::{AllocationCallbacks, CommandPool, CommandPoolCreateInfo, Device, RawCommandPoolCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_command_pool(self, create_info: &CommandPoolCreateInfo) -> Result<CommandPool> {
        let mut command_pool = CommandPool::null();
        let create_info = create_info.into_raw();

        match vk_create_command_pool(
            self,
            &create_info,
            null(),
            &mut command_pool
        ) {
            VkResult::Success => Ok(command_pool),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateCommandPool"]
    fn vk_create_command_pool(
        device: Device,
        p_create_info: *const RawCommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool
    ) -> VkResult;
}