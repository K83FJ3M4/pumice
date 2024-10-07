use crate::{Buffer, CommandBuffer, DeviceSize};

impl CommandBuffer {
    pub unsafe fn cmd_fill_buffer(
        self,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32
    ) {
        vk_cmd_fill_buffer(
            self,
            dst_buffer,
            dst_offset,
            size,
            data
        )
    }
}

extern "C" {
    #[link_name = "vkCmdFillBuffer"]
    fn vk_cmd_fill_buffer(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32
    );
}