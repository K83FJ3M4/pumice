use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Sampler(NonDispatchableHandle);

impl Sampler {
    pub const fn null() -> Self {
        Self(0)
    }
}