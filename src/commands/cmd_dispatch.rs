use crate::CommandBuffer;

impl CommandBuffer {
    pub unsafe fn cmd_dispatch(self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        vk_cmd_dispatch(self, group_count_x, group_count_y, group_count_z)
    }
}

extern "C" {
    #[link_name = "vkCmdDispatch"]
    fn vk_cmd_dispatch(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32
    );
}