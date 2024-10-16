use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CommandPool(NonDispatchableHandle);

impl CommandPool {
    pub const fn null() -> Self {
        Self(0)
    }

    pub fn is_null(self) -> bool {
        self.0 == 0
    }
}