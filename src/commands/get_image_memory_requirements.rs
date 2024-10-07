use crate::{Device, Image, MemoryRequirements};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_image_memory_requirements(self, image: Image) -> MemoryRequirements {
        let mut memory_requirements = MaybeUninit::uninit();
        vk_get_image_memory_requirements(self, image, memory_requirements.as_mut_ptr());
        memory_requirements.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetImageMemoryRequirements"]
    fn vk_get_image_memory_requirements(
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements
    );
}