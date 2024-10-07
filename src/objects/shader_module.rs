use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ShaderModule(NonDispatchableHandle);

impl ShaderModule {
    pub const fn null() -> Self {
        Self(0)
    }
}