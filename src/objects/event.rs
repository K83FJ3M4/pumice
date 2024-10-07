use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Event(NonDispatchableHandle);

impl Event {
    pub const fn null() -> Self {
        Self(0)
    }
}