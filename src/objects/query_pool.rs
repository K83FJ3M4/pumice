use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct QueryPool(NonDispatchableHandle);

impl QueryPool {
    pub const fn null() -> Self {
        Self(0)
    }
}