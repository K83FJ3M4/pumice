use crate::{CommandBuffer, StencilFaceFlags};

impl CommandBuffer {
    pub unsafe fn cmd_set_stencil_compare_mask(
        self,
        face_mask: StencilFaceFlags,
        compare_mask: u32
    ) {
        vk_cmd_set_stencil_compare_mask(
            self,
            face_mask,
            compare_mask
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetStencilCompareMask"]
    pub fn vk_cmd_set_stencil_compare_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32
    );
}