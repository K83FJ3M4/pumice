use crate::{CommandBufferLevel, CommandPool, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct CommandBufferAllocateInfo {
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
pub(crate) struct RawCommandBufferAllocateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) command_pool: CommandPool,
    pub(crate) level: CommandBufferLevel,
    pub(crate) command_buffer_count: u32,
}

impl CommandBufferAllocateInfo {
    pub(crate) fn into_raw(&self) -> RawCommandBufferAllocateInfo {
        RawCommandBufferAllocateInfo {
            s_type: StructureType::CommandBufferAllocateInfo,
            p_next: null(),
            command_pool: self.command_pool,
            level: self.level,
            command_buffer_count: self.command_buffer_count,
        }
    }
}