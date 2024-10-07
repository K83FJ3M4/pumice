use crate::{Device, DeviceMemory};

impl Device {
    pub unsafe fn unmap_memory(self, memory: DeviceMemory) {
        vk_unmap_memory(self, memory);
    }
    
}

extern "C" {
    #[link_name = "vkUnmapMemory"]
    fn vk_unmap_memory(
        device: Device,
        memory: DeviceMemory
    );
}