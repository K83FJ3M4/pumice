use crate::{CommandBuffer, RawRenderPassBeginInfo, RenderPassBeginInfo, SubpassContents};

impl CommandBuffer {
    pub unsafe fn cmd_begin_render_pass(self, render_pass_begin: &RenderPassBeginInfo, contents: SubpassContents) {
        let render_pass_begin = render_pass_begin.into_raw();
        vk_cmd_begin_render_pass(
            self,
            &render_pass_begin,
            contents
        );
    }
}

extern "C" {
    #[link_name = "vkCmdBeginRenderPass"]
    fn vk_cmd_begin_render_pass(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RawRenderPassBeginInfo,
        contents: SubpassContents
    );
}