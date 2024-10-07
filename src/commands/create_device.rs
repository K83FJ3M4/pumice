use crate::{AllocationCallbacks, Device, DeviceCreateInfo, PhysicalDevice, RawDeviceCreateInfo, VkResult};
use super::Result;
use std::ptr::null;
use bumpalo::Bump;

impl PhysicalDevice {
    pub unsafe fn create_device(
        self,
        create_info: &DeviceCreateInfo
    ) -> Result<Device> {
        let bump = Bump::new();
        let mut device = Device::null();
        let create_info = create_info.into_raw(&bump);

        match vk_create_device(
            self,
            &create_info,
            null(),
            &mut device
        ) {
            VkResult::Success => Ok(device),
            result => return Err(result)
        }
    }
}

extern "C" {
    #[link_name = "vkCreateDevice"]
    fn vk_create_device(
        physical_device: PhysicalDevice,
        p_create_info: *const RawDeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device
    ) -> VkResult;
}