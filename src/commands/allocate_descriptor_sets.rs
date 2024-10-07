use crate::{DescriptorSet, DescriptorSetAllocateInfo, Device, RawDescriptorSetAllocateInfo, Result, VkResult};

impl Device {
    pub unsafe fn allocate_descriptor_sets(self, allocate_info: &DescriptorSetAllocateInfo) -> Result<Vec<DescriptorSet>> {
        let mut descriptor_sets = vec![DescriptorSet::null(); allocate_info.set_layouts.len()];
        let allocate_info = allocate_info.into_raw();
        match vk_allocate_descriptor_sets(
            self,
            &allocate_info,
            descriptor_sets.as_mut_ptr()
        ) {
            VkResult::Success => Ok(descriptor_sets),
            error => Err(error)
        }
    }
}

extern "C" {
    #[link_name = "vkAllocateDescriptorSets"]
    fn vk_allocate_descriptor_sets(
        device: Device,
        p_allocate_info: *const RawDescriptorSetAllocateInfo,
        p_descriptor_sets: *mut DescriptorSet
    ) -> VkResult;
}