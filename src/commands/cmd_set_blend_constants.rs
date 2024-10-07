use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_set_blend_constants(
        self,
        blend_constants: [f32; 4]
    ) {
        vk_cmd_set_blend_constants(
            self,
            &blend_constants
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetBlendConstants"]
    pub fn vk_cmd_set_blend_constants(
        command_buffer: CommandBuffer,
        blend_constants: *const [f32; 4]
    );
}