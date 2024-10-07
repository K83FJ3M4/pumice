use crate::{ImageAspectFlags, SparseImageFormatFlags};
use super::Extent3D;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags
}