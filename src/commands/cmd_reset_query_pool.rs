use crate::{CommandBuffer, QueryPool};

impl CommandBuffer {
    pub unsafe fn cmd_reset_query_pool(
        self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32
    ) {
        vk_cmd_reset_query_pool(self, query_pool, first_query, query_count);
    }
}

extern "C" {
    #[link_name = "vkCmdResetQueryPool"]
    pub fn vk_cmd_reset_query_pool(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32
    );
}