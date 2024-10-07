use crate::{AllocationCallbacks, CommandPool, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_command_pool(
        self,
        command_pool: CommandPool
    ) {
        vk_destroy_command_pool(self, command_pool, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyCommandPool"]
    pub fn vk_destroy_command_pool(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    );
}