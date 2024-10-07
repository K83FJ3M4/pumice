use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PipelineCache(NonDispatchableHandle);

impl PipelineCache {
    pub const fn null() -> Self {
        Self(0)
    }
}