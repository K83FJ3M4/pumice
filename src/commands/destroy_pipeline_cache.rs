use crate::{AllocationCallbacks, Device, PipelineCache};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_pipeline_cache(self, pipeline_cache: PipelineCache) {
        vk_destroy_pipeline_cache(self, pipeline_cache, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyPipelineCache"]
    pub fn vk_destroy_pipeline_cache(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    );
}