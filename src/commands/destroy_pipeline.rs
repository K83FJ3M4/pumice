use crate::{AllocationCallbacks, Device, Pipeline};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_pipeline(self, pipeline: Pipeline) {
        vk_destroy_pipeline(self, pipeline, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyPipeline"]
    pub fn vk_destroy_pipeline(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    );
}