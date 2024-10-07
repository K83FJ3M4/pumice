use crate::{AllocationCallbacks, Device, PipelineLayout};
use std::ptr::null;

impl Device {
    pub unsafe fn destroy_pipeline_layout(self, pipeline_layout: PipelineLayout) {
        vk_destroy_pipeline_layout(self, pipeline_layout, null());
    }
}

extern "C" {
    #[link_name = "vkDestroyPipelineLayout"]
    pub fn vk_destroy_pipeline_layout(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    );
}