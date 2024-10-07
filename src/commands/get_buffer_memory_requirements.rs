use crate::{Buffer, Device, MemoryRequirements};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_buffer_memory_requirements(self, buffer: Buffer) -> MemoryRequirements {
        let mut memory_requirements = MaybeUninit::uninit();
        vk_get_buffer_memory_requirements(self, buffer, memory_requirements.as_mut_ptr());
        memory_requirements.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetBufferMemoryRequirements"]
    fn vk_get_buffer_memory_requirements(
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    );
}