#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}
