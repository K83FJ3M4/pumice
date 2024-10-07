use crate::{Buffer, CommandBuffer, DeviceSize};

impl CommandBuffer {
    pub unsafe fn cmd_dispatch_indirect(
        self,
        buffer: Buffer,
        offset: DeviceSize
    ) {
        vk_cmd_dispatch_indirect(
            self,
            buffer,
            offset
        )
    }
}

extern "C" {
    #[link_name = "vkCmdDispatchIndirect"]
    fn vk_cmd_dispatch_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize
    );
}