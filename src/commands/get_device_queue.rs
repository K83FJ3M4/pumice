use crate::{Device, Queue};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_queue(self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut queue = MaybeUninit::uninit();
        vk_get_device_queue(self, queue_family_index, queue_index, queue.as_mut_ptr());
        queue.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetDeviceQueue"]
    fn vk_get_device_queue(
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue
    );
}