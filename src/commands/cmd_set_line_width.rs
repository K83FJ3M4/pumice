use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_set_line_width(
        self,
        line_width: f32
    ) {
        vk_cmd_set_line_width(
            self,
            line_width
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetLineWidth"]
    pub fn vk_cmd_set_line_width(
        command_buffer: CommandBuffer,
        line_width: f32
    );
}