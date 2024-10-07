use super::Handle;
use std::ptr::null_mut;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PhysicalDevice(Handle);

impl PhysicalDevice {
    pub const fn null() -> Self {
        Self(null_mut())
    }
}