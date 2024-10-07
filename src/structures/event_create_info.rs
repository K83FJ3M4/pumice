use crate::{EventCreateFlags, StructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct RawEventCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: EventCreateFlags
}