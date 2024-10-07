use crate::{CommandBuffer, Filter, Image, ImageBlit, ImageLayout};

impl CommandBuffer {
    pub unsafe fn cmd_blit_image(
        self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter
    ) {
        vk_cmd_blit_image(
            self,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len().try_into().unwrap_or(u32::MAX),
            regions.as_ptr(),
            filter
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBlitImage"]
    fn vk_cmd_blit_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter
    );
}