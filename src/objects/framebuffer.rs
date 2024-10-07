use super::NonDispatchableHandle;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Framebuffer(NonDispatchableHandle);

impl Framebuffer {
    pub const fn null() -> Self {
        Self(0)
    }
}