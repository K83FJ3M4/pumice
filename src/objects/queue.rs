use super::Handle;
use std::ptr::null_mut;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Queue(Handle);

impl Queue {
    pub const fn null() -> Self {
        Self(null_mut())
    }
}