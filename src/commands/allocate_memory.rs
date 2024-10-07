use crate::{AllocationCallbacks, Device, DeviceMemory, MemoryAllocateInfo, RawMemoryAllocateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn allocate_memory(self, allocate_info: &MemoryAllocateInfo) -> Result<DeviceMemory> {
        let mut memory = DeviceMemory::null();
        let allocate_info = allocate_info.into_raw();
        match vk_allocate_memory(
            self,
            &allocate_info,
            null(),
            &mut memory
        ) {
            VkResult::Success => Ok(memory),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkAllocateMemory"]
    fn vk_allocate_memory(
        device: Device,
        p_allocate_info: *const RawMemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory
    ) -> VkResult;
}