use crate::{AllocationCallbacks, BufferView, BufferViewCreateInfo, Device, RawBufferViewCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_buffer_view(self, create_info: &BufferViewCreateInfo) -> Result<BufferView> {
        let mut buffer_view = BufferView::null();
        let create_info = create_info.into_raw();

        match vk_create_buffer_view(
            self,
            &create_info,
            null(),
            &mut buffer_view
        ) {
            VkResult::Success => Ok(buffer_view),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateBufferView"]
    pub fn vk_create_buffer_view(
        device: Device,
        p_create_info: *const RawBufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer_view: *mut BufferView
    ) -> VkResult;
}