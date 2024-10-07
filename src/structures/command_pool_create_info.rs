use crate::{CommandPoolCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct CommandPoolCreateInfo {
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32
}

#[repr(C)]
pub(crate) struct RawCommandPoolCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: CommandPoolCreateFlags,
    pub(crate) queue_family_index: u32
}

impl CommandPoolCreateInfo {
    pub(crate) fn into_raw(&self) -> RawCommandPoolCreateInfo {
        RawCommandPoolCreateInfo {
            s_type: StructureType::CommandPoolCreateInfo,
            p_next: null(),
            flags: self.flags,
            queue_family_index: self.queue_family_index
        }
    }
}