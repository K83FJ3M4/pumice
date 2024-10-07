use crate::{CommandBuffer, CommandBufferAllocateInfo, Device, RawCommandBufferAllocateInfo, Result, VkResult};
use std::usize;

impl Device {
    pub unsafe fn allocate_command_buffers(self, allocate_info: &CommandBufferAllocateInfo) -> Result<Vec<CommandBuffer>> {
        let count = allocate_info.command_buffer_count.try_into().unwrap_or(usize::MAX);
        let allocate_info = allocate_info.into_raw();
        let mut command_buffers = vec![CommandBuffer::null(); count];
        match vk_allocate_command_buffers(
            self, &allocate_info,
            command_buffers.as_mut_ptr()
        ) {
            VkResult::Success => Ok(command_buffers),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkAllocateCommandBuffers"]
    fn vk_allocate_command_buffers(
        device: Device,
        p_allocate_info: *const RawCommandBufferAllocateInfo,
        p_command_buffers: *mut CommandBuffer
    ) -> VkResult;
}