use crate::{CommandBuffer, StencilFaceFlags};

impl CommandBuffer {
    pub unsafe fn cmd_set_stencil_write_mask(
        self,
        face_mask: StencilFaceFlags,
        write_mask: u32
    ) {
        vk_cmd_set_stencil_write_mask(
            self,
            face_mask,
            write_mask
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetStencilWriteMask"]
    pub fn vk_cmd_set_stencil_write_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32
    );
}