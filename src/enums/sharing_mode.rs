#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SharingMode {
    Exclusive = 0,
    Concurrent = 1,
}