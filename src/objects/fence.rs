use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Fence(NonDispatchableHandle);

impl Fence {
    pub const fn null() -> Self {
        Self(0)
    }
}