use crate::{Buffer, CommandBuffer, DeviceSize, QueryPool, QueryResultFlags};

impl CommandBuffer {
    pub unsafe fn cmd_copy_query_pool_results(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags
    ) {
        vk_cmd_copy_query_pool_results(
            self,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags
        )
    }
}

extern "C" {
    #[link_name = "vkCmdCopyQueryPoolResults"]
    fn vk_cmd_copy_query_pool_results(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags
    );
}