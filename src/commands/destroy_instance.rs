use crate::{AllocationCallbacks, Instance};
use std::ptr::null;

impl Instance {
    pub unsafe fn destroy(self) {
        vk_destroy_instance(self, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyInstance"]
    pub fn vk_destroy_instance(
        instance: Instance,
        p_allocator: *const AllocationCallbacks,
    );
}