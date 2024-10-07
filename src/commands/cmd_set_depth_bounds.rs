use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_set_depth_bounds(
        self,
        min_depth_bounds: f32,
        max_depth_bounds: f32
    ) {
        vk_cmd_set_depth_bounds(
            self,
            min_depth_bounds,
            max_depth_bounds
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetDepthBounds"]
    pub fn vk_cmd_set_depth_bounds(
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32
    );
}