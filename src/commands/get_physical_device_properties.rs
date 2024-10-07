use crate::{PhysicalDevice, PhysicalDeviceProperties, RawPhysicalDeviceProperties};
use std::mem::MaybeUninit;

impl PhysicalDevice {
    pub unsafe fn get_properties(self) -> PhysicalDeviceProperties {
        let mut properties = MaybeUninit::uninit();
        vk_get_physical_device_properties(self, properties.as_mut_ptr());
        PhysicalDeviceProperties::from_raw(properties.assume_init())
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceProperties"]
    fn vk_get_physical_device_properties(
        physical_device: PhysicalDevice,
        p_properties: *mut RawPhysicalDeviceProperties,
    );
}