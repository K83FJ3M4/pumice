use crate::{Buffer, BufferCopy, CommandBuffer};

impl CommandBuffer {
    pub unsafe fn cmd_copy_buffer(
        self,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy]
    ) {
        vk_cmd_copy_buffer(
            self,
            src_buffer,
            dst_buffer,
            regions.len().try_into().unwrap_or(u32::MAX),
            regions.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdCopyBuffer"]
    fn vk_cmd_copy_buffer(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy
    );
}