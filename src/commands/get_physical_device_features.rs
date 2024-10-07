use crate::{PhysicalDevice, PhysicalDeviceFeatures, RawPhysicalDeviceFeatures};
use std::mem::MaybeUninit;

impl PhysicalDevice {
    pub unsafe fn get_features(self) -> PhysicalDeviceFeatures {
        let mut features = MaybeUninit::uninit();
        vk_get_physical_device_features(self, features.as_mut_ptr());
        PhysicalDeviceFeatures::from_raw(features.assume_init())
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceFeatures"]
    pub fn vk_get_physical_device_features(
        physical_device: PhysicalDevice,
        p_features: *mut RawPhysicalDeviceFeatures
    );
}