use crate::{Buffer, CommandBuffer, DeviceSize};
use std::ffi::c_void;

impl CommandBuffer {
    pub unsafe fn cmd_update_buffer(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        data: &[u8]
    ) {
        vk_cmd_update_buffer(
            self,
            buffer,
            offset,
            data.len().try_into().unwrap_or(DeviceSize::MAX),
            data.as_ptr() as *const c_void
        );
    }
}

extern "C" {
    #[link_name = "vkCmdUpdateBuffer"]
    pub fn vk_cmd_update_buffer(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void
    );
}