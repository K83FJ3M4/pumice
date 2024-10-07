#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    Uint16 = 0,
    Uint32 = 1,
}