use crate::{Buffer, BufferImageCopy, CommandBuffer, Image, ImageLayout};

impl CommandBuffer {
    pub unsafe fn cmd_copy_image_to_buffer(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy]
    ) {
        vk_cmd_copy_image_to_buffer(
            self,
            src_image,
            src_image_layout,
            dst_buffer,
            regions.len().try_into().unwrap_or(u32::MAX),
            regions.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdCopyImageToBuffer"]
    fn vk_cmd_copy_image_to_buffer(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy
    );
}