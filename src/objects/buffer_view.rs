use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BufferView(NonDispatchableHandle);

impl BufferView {
    pub const fn null() -> Self {
        Self(0)
    }
}