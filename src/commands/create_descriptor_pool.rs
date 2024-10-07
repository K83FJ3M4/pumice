use crate::{AllocationCallbacks, DescriptorPool, DescriptorPoolCreateInfo, Device, RawDescriptorPoolCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_descriptor_pool(self, create_info: &DescriptorPoolCreateInfo) -> Result<DescriptorPool> {
        let mut descriptor_pool = DescriptorPool::null();
        let create_info = create_info.into_raw();
        match vk_create_descriptor_pool(
            self,
            &create_info,
            null(),
            &mut descriptor_pool
        ) {
            VkResult::Success => Ok(descriptor_pool),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateDescriptorPool"]
    fn vk_create_descriptor_pool(
        device: Device,
        p_create_info: *const RawDescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *mut DescriptorPool
    ) -> VkResult;
}