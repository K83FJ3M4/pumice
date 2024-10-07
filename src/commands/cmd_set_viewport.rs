use crate::{CommandBuffer, Viewport};

impl CommandBuffer {
    pub unsafe fn cmd_set_viewport(
        self,
        first_viewport: u32,
        viewports: &[Viewport]
    ) {
        vk_cmd_set_viewport(
            self,
            first_viewport,
            viewports.len().try_into().unwrap_or(u32::MAX),
            viewports.as_ptr()
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetViewport"]
    pub fn vk_cmd_set_viewport(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        viewports: *const Viewport
    );
}