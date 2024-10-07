use crate::{SemaphoreCreateFlags, StructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct RawSemaphoreCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: SemaphoreCreateFlags,
}