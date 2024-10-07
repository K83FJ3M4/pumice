use crate::{CommandBuffer, Image, ImageLayout, ImageResolve};

impl CommandBuffer {
    pub unsafe fn cmd_resolve_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve]
    ) {
        vk_cmd_resolve_image(
            self,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len().try_into().unwrap_or(u32::MAX),
            regions.as_ptr()
        );
    }
}

extern "C" {
    #[link_name = "vkCmdResolveImage"]
    fn vk_cmd_resolve_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve
    );
}