use super::Extent3D;
use crate::{DeviceSize, SampleCountFlags};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize
}