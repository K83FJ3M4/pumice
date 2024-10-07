use crate::{Device, PipelineCache, VkResult};
use super::Result;

impl Device {
    pub unsafe fn merge_pipeline_caches(
        self,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> Result {
        match vk_merge_pipeline_caches(
            self,
            dst_cache,
            src_caches.len() as u32,
            src_caches.as_ptr(),
        ) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkMergePipelineCaches"]
    pub fn vk_merge_pipeline_caches(
        device: Device,
        dst_cache: PipelineCache,
        src_caches_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> VkResult;
}