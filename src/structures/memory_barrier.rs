use crate::{AccessFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct MemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags
}

#[repr(C)]
pub(crate) struct RawMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags
}

impl MemoryBarrier {
    pub(crate) fn into_raw(&self) -> RawMemoryBarrier {
        RawMemoryBarrier {
            s_type: StructureType::MemoryBarrier,
            p_next: null(),
            src_access_mask: self.src_access_mask,
            dst_access_mask: self.dst_access_mask
        }
    }
}