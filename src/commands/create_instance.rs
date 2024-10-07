use crate::{AllocationCallbacks, Instance, InstanceCreateInfo, RawInstanceCreateInfo, VkResult};
use super::Result;
use bumpalo::Bump;
use std::ptr::null;

impl Instance {
    pub unsafe fn create(create_info: &InstanceCreateInfo) -> Result<Instance> {
        let bump = Bump::new();
        let mut instance = Instance::null();
        let create_info = create_info.into_raw(&bump);
        match vk_create_instance(
            &create_info,
            null(),
            &mut instance
        ) {
            VkResult::Success => Ok(instance),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateInstance"]
    fn vk_create_instance(
        p_create_info: *const RawInstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *mut Instance
    ) -> VkResult;
}