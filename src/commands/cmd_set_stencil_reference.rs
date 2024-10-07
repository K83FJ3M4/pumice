use crate::{CommandBuffer, StencilFaceFlags};

impl CommandBuffer {
    pub unsafe fn cmd_set_stencil_reference(
        self,
        face_mask: StencilFaceFlags,
        reference: u32
    ) {
        vk_cmd_set_stencil_reference(
            self,
            face_mask,
            reference
        );
    }
}

extern "C" {
    #[link_name = "vkCmdSetStencilReference"]
    pub fn vk_cmd_set_stencil_reference(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32
    );
}