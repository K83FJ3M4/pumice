use crate::{AllocationCallbacks, Device, RawSamplerCreateInfo, Sampler, SamplerCreateInfo, VkResult};
use super::Result;

impl Device {
    pub unsafe fn create_sampler(self, create_info: &SamplerCreateInfo) -> Result<Sampler> {
        let mut sampler = Sampler::null();
        let create_info = create_info.into_raw();
        match vk_create_sampler(
            self,
            &create_info,
            std::ptr::null(),
            &mut sampler
        ) {
            VkResult::Success => Ok(sampler),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateSampler"]
    fn vk_create_sampler(
        device: Device,
        p_create_info: *const RawSamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *mut Sampler
    ) -> VkResult;
}