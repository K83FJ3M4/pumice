use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_set_depth_bias(
        self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32
    ) {
        vk_cmd_set_depth_bias(
            self,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetDepthBias"]
    pub fn vk_cmd_set_depth_bias(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32
    );
}