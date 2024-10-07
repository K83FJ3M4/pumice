#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VendorId {
    Khronos = 0x10000,
    Viv = 0x10001,
    Vsi = 0x10002,
    Kazan = 0x10003,
    Codeplay = 0x10004,
    Mesa = 0x10005,
    PoCL = 0x10006,
    Mobileye = 0x10007,
}