use crate::{CommandBuffer, PipelineLayout, ShaderStageFlags};
use std::ffi::c_void;

impl CommandBuffer {
    pub unsafe fn cmd_push_constants(
        self,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[u8]
    ) {
        vk_cmd_push_constants(
            self,
            layout,
            stage_flags,
            offset,
            values.len().try_into().unwrap_or(u32::MAX),
            values.as_ptr() as *const c_void
        );
    }
}

extern "C" {
    #[link_name = "vkCmdPushConstants"]
    fn vk_cmd_push_constants(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void
    );
}