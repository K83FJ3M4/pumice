use crate::{AllocationCallbacks, Device, QueryPool, QueryPoolCreateInfo, RawQueryPoolCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_query_pool(self, create_info: &QueryPoolCreateInfo) -> Result<QueryPool> {
        let mut query_pool = QueryPool::null();
        let create_info = create_info.into_raw();
        match vk_create_query_pool(
            self,
            &create_info,
            null(),
            &mut query_pool
        ) {
            VkResult::Success => Ok(query_pool),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateQueryPool"]
    fn vk_create_query_pool(
        device: Device,
        p_create_info: *const RawQueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *mut QueryPool
    ) -> VkResult;
}