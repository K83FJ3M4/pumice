use crate::{AllocationCallbacks, DescriptorSetLayout, Device};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_descriptor_set_layout(
        self,
        descriptor_set_layout: DescriptorSetLayout
    ) {
        vk_destroy_descriptor_set_layout(self, descriptor_set_layout, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyDescriptorSetLayout"]
    pub fn vk_destroy_descriptor_set_layout(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    );
}