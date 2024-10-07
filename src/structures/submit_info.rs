use crate::{CommandBuffer, PipelineStageFlags, Semaphore, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct SubmitInfo<'a> {
    pub wait_semaphores: &'a [Semaphore],
    pub wait_dst_stage_mask: &'a [PipelineStageFlags],
    pub command_buffers: &'a [CommandBuffer],
    pub signal_semaphores: &'a [Semaphore]
}

#[repr(C)]
pub(crate) struct RawSubmitInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) wait_semaphore_count: u32,
    pub(crate) p_wait_semaphores: *const Semaphore,
    pub(crate) p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub(crate) command_buffer_count: u32,
    pub(crate) p_command_buffers: *const CommandBuffer,
    pub(crate) signal_semaphore_count: u32,
    pub(crate) p_signal_semaphores: *const Semaphore
}

impl<'a> SubmitInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawSubmitInfo {
        RawSubmitInfo {
            s_type: StructureType::SubmitInfo,
            p_next: null(),
            wait_semaphore_count: self.wait_semaphores.len() as u32,
            p_wait_semaphores: self.wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: self.wait_dst_stage_mask.as_ptr(),
            command_buffer_count: self.command_buffers.len() as u32,
            p_command_buffers: self.command_buffers.as_ptr(),
            signal_semaphore_count: self.signal_semaphores.len() as u32,
            p_signal_semaphores: self.signal_semaphores.as_ptr()
        }
    }
}