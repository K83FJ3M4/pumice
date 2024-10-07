use crate::{CommandBuffer, PipelineStageFlags, QueryPool};

impl CommandBuffer {
    pub unsafe fn cmd_write_timestamp(
        self,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32
    ) {
        vk_cmd_write_timestamp(
            self,
            pipeline_stage,
            query_pool,
            query
        );
    }
}

extern "C" {
    #[link_name = "vkCmdWriteTimestamp"]
    pub fn vk_cmd_write_timestamp(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32
    );
}