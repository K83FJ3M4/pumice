use crate::QueueFlags;
use super::Extent3D;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D
}