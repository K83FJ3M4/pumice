#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SubpassContents {
    Inline = 0,
    SecondaryCommandBuffers = 1,
}