use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_draw(
        self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32
    ) {
        vk_cmd_draw(
            self,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance
        )
    }
}

extern "C" {
    #[link_name = "vkCmdDraw"]
    fn vk_cmd_draw(
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32
    );
}