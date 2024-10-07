use crate::{AllocationCallbacks, Device, RenderPass};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_render_pass(self, render_pass: RenderPass) {
        vk_destroy_render_pass(self, render_pass, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyRenderPass"]
    pub fn vk_destroy_render_pass(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    );
}