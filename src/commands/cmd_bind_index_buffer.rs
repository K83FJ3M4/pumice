use crate::{Buffer, CommandBuffer, DeviceSize, IndexType};

impl CommandBuffer {
    pub unsafe fn cmd_bind_index_buffer(
        self,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType
    ) {
        vk_cmd_bind_index_buffer(
            self,
            buffer,
            offset,
            index_type
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBindIndexBuffer"]
    fn vk_cmd_bind_index_buffer(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType
    );
}