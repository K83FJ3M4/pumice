use crate::{AllocationCallbacks, Device, Fence, FenceCreateInfo, RawFenceCreateInfo, VkResult};
use super::Result;

impl Device {
    pub unsafe fn create_fence(self, create_info: &FenceCreateInfo) -> Result<Fence> {
        let mut fence = Fence::null();
        let create_info = create_info.into_raw();
        match vk_create_fence(
            self,
            &create_info,
            std::ptr::null(),
            &mut fence
        ) {
            VkResult::Success => Ok(fence),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateFence"]
    fn vk_create_fence(
        device: Device,
        p_create_info: *const RawFenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence
    ) -> VkResult;
}