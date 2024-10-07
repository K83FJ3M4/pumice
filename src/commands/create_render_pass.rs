use crate::{AllocationCallbacks, Device, RawRenderPassCreateInfo, RenderPass, RenderPassCreateInfo, VkResult};
use super::Result;
use bumpalo::Bump;

impl Device {
    pub unsafe fn create_render_pass(self, create_info: &RenderPassCreateInfo) -> Result<RenderPass> {
        let bump = Bump::new();
        let mut render_pass = RenderPass::null();
        let create_info = create_info.into_raw(&bump);
        match vk_create_render_pass(
            self,
            &create_info,
            std::ptr::null(),
            &mut render_pass
        ) {
            VkResult::Success => Ok(render_pass),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateRenderPass"]
    fn vk_create_render_pass(
        device: Device,
        p_create_info: *const RawRenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass
    ) -> VkResult;
}