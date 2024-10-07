use crate::{CommandBufferUsageFlags, StructureType};
use super::{CommandBufferInheritanceInfo, RawCommandBufferInheritanceInfo};
use std::ffi::c_void;
use std::ptr::null;
use bumpalo::Bump;

#[derive(Clone, Copy)]
pub struct CommandBufferBeginInfo {
    pub flags: CommandBufferUsageFlags,
    pub inheritance_info: Option<CommandBufferInheritanceInfo>
}

#[repr(C)]
pub(crate) struct RawCommandBufferBeginInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: CommandBufferUsageFlags,
    pub(crate) p_inheritance_info: *const RawCommandBufferInheritanceInfo
}

impl CommandBufferBeginInfo {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawCommandBufferBeginInfo {
        let p_inheritance_info = match self.inheritance_info {
            Some(inheritance_info) => bump.alloc(inheritance_info.into_raw()),
            None => null()
        };

        RawCommandBufferBeginInfo {
            s_type: StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: self.flags,
            p_inheritance_info
        }
    }
}