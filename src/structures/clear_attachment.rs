use crate::ImageAspectFlags;
use super::ClearValue;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue
}