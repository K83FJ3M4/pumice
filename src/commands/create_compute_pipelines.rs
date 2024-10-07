use crate::{AllocationCallbacks, ComputePipelineCreateInfo, Device, Pipeline, PipelineCache, RawComputePipelineCreateInfo, VkResult};
use super::Result;
use std::ptr::null;
use bumpalo::Bump;

impl Device {
    pub unsafe fn create_compute_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo]
    ) -> Result<Vec<Pipeline>> {
        let bump = Bump::new();
        let mut pipelines = vec![Pipeline::null(); create_infos.len()];
        let create_infos = bump.alloc_slice_fill_iter(
            create_infos.into_iter().map(|create_info| create_info.into_raw(&bump))
        );

        match vk_create_compute_pipelines(
            self,
            pipeline_cache,
            create_infos.len().try_into().unwrap_or(u32::MAX),
            create_infos.as_ptr(),
            null(),
            pipelines.as_mut_ptr()
        ) {
            VkResult::Success => Ok(pipelines),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateComputePipelines"]
    fn vk_create_compute_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RawComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline
    ) -> VkResult;
}