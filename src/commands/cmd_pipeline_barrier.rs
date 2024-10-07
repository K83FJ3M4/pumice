use crate::{BufferMemoryBarrier, CommandBuffer, DependencyFlags, ImageMemoryBarrier, MemoryBarrier, PipelineStageFlags, RawBufferMemoryBarrier, RawImageMemoryBarrier, RawMemoryBarrier};
use bumpalo::Bump;

impl CommandBuffer {
    pub unsafe fn cmd_pipeline_barrier(
        self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier]
    ) {
        let bump = Bump::new();

        let memory_barriers = bump.alloc_slice_fill_iter(
            memory_barriers.into_iter().map(MemoryBarrier::into_raw)
        );

        let buffer_memory_barriers = bump.alloc_slice_fill_iter(
            buffer_memory_barriers.into_iter().map(BufferMemoryBarrier::into_raw)
        );

        let image_memory_barriers = bump.alloc_slice_fill_iter(
            image_memory_barriers.into_iter().map(ImageMemoryBarrier::into_raw)
        );

        vk_cmd_pipeline_barrier(
            self,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barriers.len().try_into().unwrap_or(u32::MAX),
            memory_barriers.as_ptr(),
            buffer_memory_barriers.len().try_into().unwrap_or(u32::MAX),
            buffer_memory_barriers.as_ptr(),
            image_memory_barriers.len().try_into().unwrap_or(u32::MAX),
            image_memory_barriers.as_ptr()
        )
    }
}

extern "C" {
    #[link_name = "vkCmdPipelineBarrier"]
    fn vk_cmd_pipeline_barrier(
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: u32,
        memory_barriers: *const RawMemoryBarrier,
        buffer_memory_barriers: u32,
        buffer_memory_barriers: *const RawBufferMemoryBarrier,
        image_memory_barriers: u32,
        image_memory_barriers: *const RawImageMemoryBarrier
    );
}