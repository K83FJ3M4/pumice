use crate::{ClearColorValue, CommandBuffer, Image, ImageLayout, ImageSubresourceRange};

impl CommandBuffer {
    pub unsafe fn cmd_clear_color_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange]
    ) {
        vk_cmd_clear_color_image(
            self,
            image,
            image_layout,
            color,
            ranges.len().try_into().unwrap_or(u32::MAX),
            ranges.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdClearColorImage"]
    fn vk_cmd_clear_color_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange
    );
}