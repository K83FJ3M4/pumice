use crate::{Device, DeviceMemory, DeviceSize, MemoryMapFlags, VkResult};
use std::ffi::c_void;
use super::Result;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;

impl Device {
    pub unsafe fn map_memory(
        self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
    ) -> Result<&'static mut [u8]> {
        let mut data = null_mut();
        match vk_map_memory(
            self,
            memory,
            offset,
            size,
            MemoryMapFlags::empty(),
            &mut data
        ) {
            VkResult::Success => Ok(from_raw_parts_mut(data as *mut u8, size as usize)),
            result => Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkMapMemory"]
    fn vk_map_memory(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void
    ) -> VkResult;
}