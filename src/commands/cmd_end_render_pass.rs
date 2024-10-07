use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_end_render_pass(self) {
        vk_cmd_end_render_pass(self)
    }
}

extern "C" {
    #[link_name = "vkCmdEndRenderPass"]
    fn vk_cmd_end_render_pass(
        command_buffer: CommandBuffer
    );
}