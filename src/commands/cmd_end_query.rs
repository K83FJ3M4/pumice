use crate::{CommandBuffer, QueryPool};

impl CommandBuffer {
    pub unsafe fn cmd_end_query(self, query_pool: QueryPool, query: u32) {
        vk_cmd_end_query(
            self,
            query_pool,
            query
        )
    }
}

extern "C" {
    #[link_name = "vkCmdEndQuery"]
    fn vk_cmd_end_query(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32
    );
}