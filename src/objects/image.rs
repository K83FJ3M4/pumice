use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Image(NonDispatchableHandle);

impl Image {
    pub const fn null() -> Self {
        Self(0)
    }
}