use crate::{AllocationCallbacks, Device, Framebuffer, FramebufferCreateInfo, RawFramebufferCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_framebuffer(self, create_info: &FramebufferCreateInfo) -> Result<Framebuffer> {
        let mut framebuffer = Framebuffer::null();
        let create_info = create_info.into_raw();

        match vk_create_framebuffer(
            self,
            &create_info,
            null(),
            &mut framebuffer
        ) {
            VkResult::Success => Ok(framebuffer),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateFramebuffer"]
    fn vk_create_framebuffer(
        device: Device,
        p_create_info: *const RawFramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer
    ) -> VkResult;
}