use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Semaphore(NonDispatchableHandle);

impl Semaphore {
    pub const fn null() -> Self {
        Self(0)
    }
}