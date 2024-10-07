use crate::{Buffer, CommandBuffer, DeviceSize};

impl CommandBuffer {
    pub unsafe fn cmd_bind_vertex_buffers(
        self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize]
    ) {
        vk_cmd_bind_vertex_buffers(
            self,
            first_binding,
            buffers.len().min(offsets.len()).try_into().unwrap_or(u32::MAX),
            buffers.as_ptr(),
            offsets.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBindVertexBuffers"]
    fn vk_cmd_bind_vertex_buffers(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize
    );
}