use crate::{CommandBuffer, SubpassContents};

impl CommandBuffer {
    pub unsafe fn cmd_next_subpass(self, contents: SubpassContents) {
        vk_cmd_next_subpass(
            self,
            contents
        )
    }
}

extern "C" {
    #[link_name = "vkCmdNextSubpass"]
    fn vk_cmd_next_subpass(
        command_buffer: CommandBuffer,
        contents: SubpassContents
    );
}