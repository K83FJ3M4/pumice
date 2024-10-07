use crate::UUID_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; UUID_SIZE]
}