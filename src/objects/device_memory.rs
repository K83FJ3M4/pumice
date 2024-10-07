use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DeviceMemory(NonDispatchableHandle);

impl DeviceMemory {
    pub const fn null() -> Self {
        Self(0)
    }
}