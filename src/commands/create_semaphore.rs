use crate::{AllocationCallbacks, Device, RawSemaphoreCreateInfo, Semaphore, SemaphoreCreateFlags, StructureType, VkResult};
use std::ptr::null;
use super::Result;

impl Device {
    pub unsafe fn create_semaphore(self) -> Result<Semaphore> {
        let mut semaphore = Semaphore::null();
        let create_info = RawSemaphoreCreateInfo {
            s_type: StructureType::SemaphoreCreateInfo,
            p_next: null(),
            flags: SemaphoreCreateFlags::empty()
        };

        match vk_create_semaphore(
            self,
            &create_info,
            null(),
            &mut semaphore
        ) {
            VkResult::Success => Ok(semaphore),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateSemaphore"]
    fn vk_create_semaphore(
        device: Device,
        p_create_info: *const RawSemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore
    ) -> VkResult;
}