use crate::{ClearDepthStencilValue, CommandBuffer, Image, ImageLayout, ImageSubresourceRange};

impl CommandBuffer {
    pub unsafe fn cmd_clear_depth_stencil_image(
        self,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange]
    ) {
        vk_cmd_clear_depth_stencil_image(
            self,
            image,
            image_layout,
            depth_stencil,
            ranges.len().try_into().unwrap_or(u32::MAX),
            ranges.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdClearDepthStencilImage"]
    fn vk_cmd_clear_depth_stencil_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange
    );
}