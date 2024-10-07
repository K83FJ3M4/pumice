use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DescriptorPool(NonDispatchableHandle);

impl DescriptorPool {
    pub const fn null() -> Self {
        Self(0)
    }
}