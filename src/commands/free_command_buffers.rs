use crate::{CommandBuffer, CommandPool, Device};

impl Device {
    pub unsafe fn free_command_buffers(
        self,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer]
    ) {
        vk_free_command_buffers(
            self,
            command_pool,
            command_buffers.len() as u32,
            command_buffers.as_ptr()
        );
    }
}

extern "C" {
    #[link_name = "vkFreeCommandBuffers"]
    fn vk_free_command_buffers(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer
    );
}