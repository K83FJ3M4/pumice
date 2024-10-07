use crate::{AttachmentDescriptionFlags, AttachmentLoadOp, AttachmentStoreOp, Format, ImageLayout, SampleCountFlags};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout
}