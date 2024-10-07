use crate::{CommandBuffer, Event, PipelineStageFlags};

impl CommandBuffer {
    pub unsafe fn cmd_reset_event(
        self,
        event: Event,
        stage_mask: PipelineStageFlags
    ) {
        vk_cmd_reset_event(self, event, stage_mask);
    }
}

extern "C" {
    #[link_name = "vkCmdResetEvent"]
    pub fn vk_cmd_reset_event(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags
    );
}