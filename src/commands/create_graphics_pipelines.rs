use crate::{AllocationCallbacks, Device, GraphicsPipelineCreateInfo, Pipeline, PipelineCache, RawGraphicsPipelineCreateInfo, VkResult};
use super::Result;
use bumpalo::Bump;
use std::ptr::null;

impl Device {
    pub unsafe fn create_graphics_pipelines(
        self,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo]
    ) -> Result<Vec<Pipeline>> {
        let bump = Bump::new();
        let mut pipelines = vec![Pipeline::null(); create_infos.len()];
        let create_infos = bump.alloc_slice_fill_iter(
            create_infos.into_iter().map(|create_info| create_info.into_raw(&bump))
        );

        match vk_create_graphics_pipelines(
            self,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            null(),
            pipelines.as_mut_ptr()
        ) {
            VkResult::Success => Ok(pipelines),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateGraphicsPipelines"]
    fn vk_create_graphics_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RawGraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline
    ) -> VkResult;
}