#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttachmentStoreOp {
    Store = 0,
    DontCare = 1,
}