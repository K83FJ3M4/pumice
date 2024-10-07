use crate::{BindSparseInfo, Fence, Queue, RawBindSparseInfo, VkResult};
use bumpalo::Bump;

impl Queue {
    pub unsafe fn bind_sparse(
        self,
        bind_infos: &[BindSparseInfo],
        fence: Fence
    ) -> VkResult {
        let bump = Bump::new();

        let bind_infos = bump.alloc_slice_fill_iter(
            bind_infos.into_iter().map(|bind_info| bind_info.into_raw(&bump))
        );

        vk_queue_bind_sparse(
            self,
            bind_infos.len() as u32,
            bind_infos.as_ptr(),
            fence
        )
    }
}

extern "C" {
    #[link_name = "vkQueueBindSparse"]
    fn vk_queue_bind_sparse(
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const RawBindSparseInfo,
        fence: Fence
    ) -> VkResult;
}