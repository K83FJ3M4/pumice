use crate::{Bool32, TRUE};

#[derive(Clone, Copy)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: bool,
    pub residency_standard_2d_multisample_block_shape: bool,
    pub residency_standard_3d_block_shape: bool,
    pub residency_aligned_mip_size: bool,
    pub residency_non_resident_strict: bool
}

#[repr(C)]
pub(crate) struct RawPhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32
}

impl PhysicalDeviceSparseProperties {
    pub(crate) fn from_raw(raw: RawPhysicalDeviceSparseProperties) -> PhysicalDeviceSparseProperties {
        PhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: raw.residency_standard_2d_block_shape == TRUE,
            residency_standard_2d_multisample_block_shape: raw.residency_standard_2d_multisample_block_shape == TRUE,
            residency_standard_3d_block_shape: raw.residency_standard_3d_block_shape == TRUE,
            residency_aligned_mip_size: raw.residency_aligned_mip_size == TRUE,
            residency_non_resident_strict: raw.residency_non_resident_strict == TRUE
        }
    }
}