use crate::{PhysicalDevice, PhysicalDeviceMemoryProperties, RawPhysicalDeviceMemoryProperties};
use std::mem::MaybeUninit;

impl PhysicalDevice {
    pub unsafe fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties {
        let mut properties = MaybeUninit::uninit();
        vk_get_physical_device_memory_properties(self, properties.as_mut_ptr());
        PhysicalDeviceMemoryProperties::from_raw(properties.assume_init())
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceMemoryProperties"]
    pub fn vk_get_physical_device_memory_properties(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut RawPhysicalDeviceMemoryProperties,
    );
}