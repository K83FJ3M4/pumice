use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct RenderPass(NonDispatchableHandle);

impl RenderPass {
    pub const fn null() -> Self {
        Self(0)
    }
}