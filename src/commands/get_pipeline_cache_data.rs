use crate::{Device, PipelineCache, VkResult};
use std::ffi::c_void;
use super::Result;
use std::ptr::null_mut;

impl Device {
    pub unsafe fn get_pipeline_cache_data(self, pipeline_cache: PipelineCache) -> Result<Vec<u8>> {
        let mut data_size = 0;
        let mut data = Vec::new();

        match vk_get_pipeline_cache_data(
            self,
            pipeline_cache,
            &mut data_size,
            null_mut()
        ) {
            VkResult::Success => (),
            result => return Err(result),
        }

        data.reserve(data_size);
        data.set_len(data_size);

        match vk_get_pipeline_cache_data(
            self,
            pipeline_cache,
            &mut data_size,
            data.as_mut_ptr() as *mut c_void
        ) {
            VkResult::Success => Ok(data),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkGetPipelineCacheData"]
    pub fn vk_get_pipeline_cache_data(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult;
}