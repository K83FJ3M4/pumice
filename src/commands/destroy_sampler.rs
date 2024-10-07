use crate::{AllocationCallbacks, Device, Sampler};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_sampler(self, sampler: Sampler) {
        vk_destroy_sampler(self, sampler, null());
    }
}

extern "C" {
    #[link_name = "vkDestroySampler"]
    pub fn vk_destroy_sampler(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    );
}