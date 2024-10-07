use crate::{Buffer, CommandBuffer, DeviceSize};

impl CommandBuffer {
    pub unsafe fn cmd_draw_indexed_indirect(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32
    ) {
        vk_cmd_draw_indexed_indirect(
            self,
            buffer,
            offset,
            draw_count,
            stride
        )
    }
}

extern "C" {
    #[link_name = "vkCmdDrawIndexedIndirect"]
    fn vk_cmd_draw_indexed_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32
    );
}