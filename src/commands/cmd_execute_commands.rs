use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_execute_commands(
        self,
        command_buffers: &[CommandBuffer]
    ) {
        vk_cmd_execute_commands(
            self,
            command_buffers.len().try_into().unwrap_or(u32::MAX),
            command_buffers.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdExecuteCommands"]
    fn vk_cmd_execute_commands(
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer
    );
}