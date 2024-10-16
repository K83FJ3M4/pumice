use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DescriptorSetLayout(NonDispatchableHandle);

impl DescriptorSetLayout {
    pub const fn null() -> Self {
        Self(0)
    }

    pub fn is_null(self) -> bool {
        self.0 == 0
    }
}