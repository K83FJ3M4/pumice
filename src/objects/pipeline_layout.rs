use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PipelineLayout(NonDispatchableHandle);

impl PipelineLayout {
    pub const fn null() -> Self {
        Self(0)
    }
}