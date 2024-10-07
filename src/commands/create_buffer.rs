use crate::{AllocationCallbacks, Buffer, BufferCreateInfo, Device, RawBufferCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_buffer(
        self,
        create_info: &BufferCreateInfo
    ) -> Result<Buffer> {
        let mut buffer = Buffer::null();
        let create_info = create_info.into_raw();

        match vk_create_buffer(
            self,
            &create_info,
            null(),
            &mut buffer
        ) {
            VkResult::Success => Ok(buffer),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateBuffer"]
    fn vk_create_buffer(
        device: Device,
        p_create_info: *const RawBufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer
    ) -> VkResult;
}