use super::Handle;
use std::ptr::null_mut;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Device(Handle);

impl Device {
    pub const fn null() -> Self {
        Self(null_mut())
    }

    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}