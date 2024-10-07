use crate::{AllocationCallbacks, Device, ShaderModule};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_shader_module(self, shader_module: ShaderModule) {
        vk_destroy_shader_module(self, shader_module, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyShaderModule"]
    pub fn vk_destroy_shader_module(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    );
}