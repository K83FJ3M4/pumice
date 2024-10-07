use crate::{PhysicalDeviceType, MAX_PHYSICAL_DEVICE_NAME_SIZE, UUID_SIZE};
use super::{CharArray, PhysicalDeviceLimits, PhysicalDeviceSparseProperties, RawPhysicalDeviceLimits, RawPhysicalDeviceSparseProperties};

#[derive(Clone, Copy)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: CharArray<MAX_PHYSICAL_DEVICE_NAME_SIZE>,
    pub pipeline_cache_uuid: [u8; UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties
}

#[repr(C)]
pub(crate) struct RawPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: CharArray<MAX_PHYSICAL_DEVICE_NAME_SIZE>,
    pub pipeline_cache_uuid: [u8; UUID_SIZE],
    pub limits: RawPhysicalDeviceLimits,
    pub sparse_properties: RawPhysicalDeviceSparseProperties
}

impl PhysicalDeviceProperties {
    pub(crate) fn from_raw(raw: RawPhysicalDeviceProperties) -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            api_version: raw.api_version,
            driver_version: raw.driver_version,
            vendor_id: raw.vendor_id,
            device_id: raw.device_id,
            device_type: raw.device_type,
            device_name: raw.device_name,
            pipeline_cache_uuid: raw.pipeline_cache_uuid,
            limits: PhysicalDeviceLimits::from_raw(raw.limits),
            sparse_properties: PhysicalDeviceSparseProperties::from_raw(raw.sparse_properties)
        }
    }
}
