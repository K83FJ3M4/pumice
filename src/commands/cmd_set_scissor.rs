use crate::{CommandBuffer, Rect2D};

impl CommandBuffer {
    pub unsafe fn cmd_set_scissor(
        self,
        first_scissor: u32,
        scissors: &[Rect2D]
    ) {
        vk_cmd_set_scissor(
            self,
            first_scissor,
            scissors.len().try_into().unwrap_or(u32::MAX),
            scissors.as_ptr()
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetScissor"]
    pub fn vk_cmd_set_scissor(
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        scissors: *const Rect2D
    );
}