use crate::{Device, DeviceSize, QueryPool, QueryResultFlags, VkResult};
use std::ffi::c_void;
use super::Result;

impl Device {
    pub unsafe fn get_query_pool_results(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut [u8],
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result {
        match vk_get_query_pool_results(
            self,
            query_pool,
            first_query,
            query_count,
            data.len(),
            data.as_mut_ptr() as *mut c_void,
            stride,
            flags,
        ) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkGetQueryPoolResults"]
    fn vk_get_query_pool_results(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> VkResult;
}