use crate::{Buffer, BufferImageCopy, CommandBuffer, Image, ImageLayout};

impl CommandBuffer {
    pub unsafe fn cmd_copy_buffer_to_image(
        self,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy]
    ) {
        vk_cmd_copy_buffer_to_image(
            self,
            src_buffer,
            dst_image,
            dst_image_layout,
            regions.len().try_into().unwrap_or(u32::MAX),
            regions.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdCopyBufferToImage"]
    fn vk_cmd_copy_buffer_to_image(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy
    );
}