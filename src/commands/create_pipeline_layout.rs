use crate::{AllocationCallbacks, Device, PipelineLayout, PipelineLayoutCreateInfo, RawPipelineLayoutCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_pipeline_layout(self, create_info: &PipelineLayoutCreateInfo) -> Result<PipelineLayout> {
        let mut pipeline_layout = PipelineLayout::null();
        let create_info = create_info.into_raw();
        match vk_create_pipeline_layout(
            self,
            &create_info,
            null(),
            &mut pipeline_layout
        ) {
            VkResult::Success => Ok(pipeline_layout),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreatePipelineLayout"]
    fn vk_create_pipeline_layout(
        device: Device,
        p_create_info: *const RawPipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *mut PipelineLayout
    ) -> VkResult;
}