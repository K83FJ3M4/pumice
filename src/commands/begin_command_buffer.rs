use crate::{CommandBuffer, CommandBufferBeginInfo, RawCommandBufferBeginInfo, VkResult};
use super::Result;
use bumpalo::Bump;

impl CommandBuffer {
    pub unsafe fn begin(self, begin_info: &CommandBufferBeginInfo) -> Result {
        let bump = Bump::new();
        let begin_info = begin_info.into_raw(&bump);
        match vk_begin_command_buffer(
            self,
            &begin_info
        ) {
            VkResult::Success => Ok(()),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkBeginCommandBuffer"]
    fn vk_begin_command_buffer(
        command_buffer: CommandBuffer,
        p_begin_info: *const RawCommandBufferBeginInfo
    ) -> VkResult;
}