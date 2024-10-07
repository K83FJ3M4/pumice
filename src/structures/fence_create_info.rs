use crate::{FenceCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct FenceCreateInfo {
    pub flags: FenceCreateFlags
}

#[repr(C)]
pub(crate) struct RawFenceCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: FenceCreateFlags
}

impl FenceCreateInfo {
    pub(crate) fn into_raw(&self) -> RawFenceCreateInfo {
        RawFenceCreateInfo {
            s_type: StructureType::FenceCreateInfo,
            p_next: null(),
            flags: self.flags
        }
    }
}