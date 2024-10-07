use crate::{AllocationCallbacks, Device, DeviceMemory};
use std::ptr::null;

impl Device {
    pub unsafe fn free_memory(self, memory: DeviceMemory) {
        vk_free_memory(self, memory, null());
    }
}

extern "C" {
    #[link_name = "vkFreeMemory"]
    fn vk_free_memory(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    );
}