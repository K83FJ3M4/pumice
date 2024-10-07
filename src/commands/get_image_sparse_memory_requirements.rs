use crate::{Device, Image, SparseImageMemoryRequirements};
use std::ptr::null_mut;

impl Device {
    pub unsafe fn get_image_sparse_memory_requirements(self, image: Image) -> Vec<SparseImageMemoryRequirements> {
        let mut sparse_memory_requirement_count = 0;
        let mut sparse_memory_requirements = Vec::new();
        vk_get_image_sparse_memory_requirements(
            self,
            image,
            &mut sparse_memory_requirement_count,
            null_mut()
        );
        sparse_memory_requirements.reserve(sparse_memory_requirement_count as usize);
        sparse_memory_requirements.set_len(sparse_memory_requirement_count as usize);
        vk_get_image_sparse_memory_requirements(
            self,
            image,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr()
        );
        sparse_memory_requirements
    }
}

extern "C" {
    #[link_name = "vkGetImageSparseMemoryRequirements"]
    fn vk_get_image_sparse_memory_requirements(
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    );
}