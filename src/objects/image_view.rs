use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ImageView(NonDispatchableHandle);

impl ImageView {
    pub const fn null() -> Self {
        Self(0)
    }
}