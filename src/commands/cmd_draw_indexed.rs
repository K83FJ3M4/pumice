use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_draw_indexed(
        self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32
    ) {
        vk_cmd_draw_indexed(
            self,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance
        )
    }
}

extern "C" {
    #[link_name = "vkCmdDrawIndexed"]
    fn vk_cmd_draw_indexed(
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32
    );
}