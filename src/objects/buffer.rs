use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Buffer(NonDispatchableHandle);

impl Buffer {
    pub const fn null() -> Self {
        Self(0)
    }
}