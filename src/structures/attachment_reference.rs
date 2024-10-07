use crate::ImageLayout;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout
}