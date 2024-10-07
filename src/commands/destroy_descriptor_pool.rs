use crate::{AllocationCallbacks, DescriptorPool, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_descriptor_pool(
        self,
        descriptor_pool: DescriptorPool
    ) {
        vk_destroy_descriptor_pool(self, descriptor_pool, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyDescriptorPool"]
    pub fn vk_destroy_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    );
}