use crate::{CommandBuffer, DescriptorSet, PipelineBindPoint, PipelineLayout};

impl CommandBuffer {
    pub unsafe fn cmd_bind_descriptor_sets(
        self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32]
    ) {
        vk_cmd_bind_descriptor_sets(
            self,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_sets.len().try_into().unwrap_or(u32::MAX),
            descriptor_sets.as_ptr(),
            dynamic_offsets.len().try_into().unwrap_or(u32::MAX),
            dynamic_offsets.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdBindDescriptorSets"]
    fn vk_cmd_bind_descriptor_sets(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32
    );
}