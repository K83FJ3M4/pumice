use crate::{PhysicalDevice, QueueFamilyProperties};

impl PhysicalDevice {
    pub unsafe fn get_queue_family_properties(self) -> Vec<QueueFamilyProperties> {
        let mut queue_family_property_count = 0;    
        let mut queue_family_properties = Vec::new();
        
        vk_get_physical_device_queue_family_properties(
            self,
            &mut queue_family_property_count,
            std::ptr::null_mut(),
        );
        
        queue_family_properties.reserve(queue_family_property_count as usize);
        queue_family_properties.set_len(queue_family_property_count as usize);

        vk_get_physical_device_queue_family_properties(
            self,
            &mut queue_family_property_count,
            queue_family_properties.as_mut_ptr(),
        );

        queue_family_properties
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceQueueFamilyProperties"]
    pub fn vk_get_physical_device_queue_family_properties(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    );
}