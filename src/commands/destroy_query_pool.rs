use crate::{AllocationCallbacks, Device, QueryPool};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_query_pool(self, query_pool: QueryPool) {
        vk_destroy_query_pool(self, query_pool, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyQueryPool"]
    pub fn vk_destroy_query_pool(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    );
}