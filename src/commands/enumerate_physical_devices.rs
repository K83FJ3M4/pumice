use crate::{Instance, PhysicalDevice, VkResult};
use super::Result;
use std::ptr::null_mut;

impl Instance {
    pub unsafe fn enumerate_physical_devices(self) -> Result<Vec<PhysicalDevice>> {
        let mut physical_device_count = 0;
        let mut physical_devices = Vec::new();

        match vk_enumerate_physical_devices(
            self,
            &mut physical_device_count,
            null_mut()
        ) {
            VkResult::Success => (),
            result => return Err(result),
        }

        physical_devices.reserve(physical_device_count as usize);
        physical_devices.set_len(physical_device_count as usize);

        match vk_enumerate_physical_devices(
            self,
            &mut physical_device_count,
            physical_devices.as_mut_ptr()
        ) {
            VkResult::Success => Ok(physical_devices),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkEnumeratePhysicalDevices"]
    pub fn vk_enumerate_physical_devices(
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> VkResult;
}