use crate::{DeviceQueueCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct DeviceQueueCreateInfo<'a> {
    pub queue_family_index: u32,
    pub queue_priorities: &'a [f32]
}

#[repr(C)]
pub(crate) struct RawDeviceQueueCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: DeviceQueueCreateFlags,
    pub(crate) queue_family_index: u32,
    pub(crate) queue_count: u32,
    pub(crate) p_queue_priorities: *const f32
}

impl<'a> DeviceQueueCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawDeviceQueueCreateInfo {
        RawDeviceQueueCreateInfo {
            s_type: StructureType::DeviceQueueCreateInfo,
            p_next: null(),
            flags: DeviceQueueCreateFlags::empty(),
            queue_family_index: self.queue_family_index,
            queue_count: self.queue_priorities.len() as u32,
            p_queue_priorities: self.queue_priorities.as_ptr()
        }
    }
}