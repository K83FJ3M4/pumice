use crate::{AllocationCallbacks, Device, PipelineCache, PipelineCacheCreateInfo, RawPipelineCacheCreateInfo, VkResult};
use super::Result;
use std::ptr::null;

impl Device {
    pub unsafe fn create_pipeline_cache(self, create_info: &PipelineCacheCreateInfo) -> Result<PipelineCache> {
        let mut pipeline_cache = PipelineCache::null();
        let create_info = create_info.into_raw();
        match vk_create_pipeline_cache(
            self,
            &create_info,
            null(),
            &mut pipeline_cache
        ) {
            VkResult::Success => Ok(pipeline_cache),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreatePipelineCache"]
    fn vk_create_pipeline_cache(
        device: Device,
        p_create_info: *const RawPipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *mut PipelineCache
    ) -> VkResult;
}