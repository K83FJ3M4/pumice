use crate::{Device, DeviceMemory, DeviceSize};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_memory_commitment(self, memory: DeviceMemory) -> DeviceSize {
        let mut committed_memory_in_bytes = MaybeUninit::uninit();
        vk_get_device_memory_commitment(self, memory, committed_memory_in_bytes.as_mut_ptr());
        committed_memory_in_bytes.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetDeviceMemoryCommitment"]
    fn vk_get_device_memory_commitment(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize
    );
}