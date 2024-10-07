use crate::{AttachmentDescription, ClearRect, CommandBuffer};

impl CommandBuffer {
    pub unsafe fn cmd_clear_attachments(
        self,
        attachments: &[AttachmentDescription],
        rects: &[ClearRect]
    ) {
        vk_cmd_clear_attachments(
            self,
            attachments.len().try_into().unwrap_or(u32::MAX),
            attachments.as_ptr(),
            rects.len().try_into().unwrap_or(u32::MAX),
            rects.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdClearAttachments"]
    fn vk_cmd_clear_attachments(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const AttachmentDescription,
        rect_count: u32,
        p_rects: *const ClearRect
    );
}