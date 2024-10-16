use crate::{Device, DeviceMemory, DeviceSize, MemoryMapFlags, VkResult};
use std::ffi::c_void;
use super::Result;
use std::ptr::null_mut;
use std::mem::{align_of, size_of};
use std::ops::{Deref, DerefMut};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct MappedMemory<T> {
    data: *mut T,
    size: usize
}

impl Device {
    pub unsafe fn map_memory<T>(
        self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
    ) -> Result<MappedMemory<T>> {
        let mut data = null_mut();
        match vk_map_memory(
            self,
            memory,
            offset,
            size,
            MemoryMapFlags::empty(),
            &mut data
        ) {
            VkResult::Success => {
                if size.next_multiple_of(size_of::<T>() as u64) != size {
                    return Err(VkResult::ErrorMemoryMapFailed);
                }

                if offset.next_multiple_of(align_of::<T>() as u64) != offset {
                    return Err(VkResult::ErrorMemoryMapFailed);
                }

                Ok(MappedMemory {
                    data: data as *mut T,
                    size: size as usize
                })
            },
            result => Err(result)
        }
    }
}

impl<T> MappedMemory<T> {
    pub fn null() -> Self {
        MappedMemory {
            data: null_mut(),
            size: 0
        }
    }
}

impl<T> Deref for MappedMemory<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            from_raw_parts(self.data, self.size)
        }
    }
}

impl<T> DerefMut for MappedMemory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            from_raw_parts_mut(self.data, self.size)
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