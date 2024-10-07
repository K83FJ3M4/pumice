use crate::{AllocationCallbacks, Device, RawShaderModuleCreateInfo, ShaderModule, ShaderModuleCreateInfo, VkResult};
use super::Result;

impl Device {
    pub unsafe fn create_shader_module(self, create_info: &ShaderModuleCreateInfo) -> Result<ShaderModule> {
        let mut shader_module = ShaderModule::null();
        let create_info = create_info.into_raw();

        match vk_create_shader_module(
            self,
            &create_info,
            std::ptr::null(),
            &mut shader_module
        ) {
            VkResult::Success => Ok(shader_module),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateShaderModule"]
    fn vk_create_shader_module(
        device: Device,
        create_info: *const RawShaderModuleCreateInfo,
        allocator: *const AllocationCallbacks,
        shader_module: *mut ShaderModule
    ) -> VkResult;
}