use crate::{CommandBuffer, QueryControlFlags, QueryPool};

impl CommandBuffer {
    pub unsafe fn cmd_begin_query(self, query_pool: QueryPool, query: u32, flags: QueryControlFlags) {
        vk_cmd_begin_query(
            self,
            query_pool,
            query,
            flags
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBeginQuery"]
    fn vk_cmd_begin_query(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags  
    );
}