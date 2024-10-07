use crate::{CommandBuffer, Pipeline, PipelineBindPoint};

impl CommandBuffer {
    pub unsafe fn cmd_bind_pipeline(self, pipeline_bind_point: PipelineBindPoint, pipeline: Pipeline) {
        vk_cmd_bind_pipeline(
            self,
            pipeline_bind_point,
            pipeline
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBindPipeline"]
    fn vk_cmd_bind_pipeline(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline
    );
}