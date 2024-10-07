use crate::{Fence, Queue, RawSubmitInfo, SubmitInfo, VkResult};
use super::Result;
use bumpalo::Bump;

impl Queue {
    pub unsafe fn submit(
        self,
        submits: &[SubmitInfo],
        fence: Fence
    ) -> Result {
        let bump = Bump::new();

        let submits = bump.alloc_slice_fill_iter(
            submits.into_iter().map(SubmitInfo::into_raw)
        );

        match vk_queeu_submit(
            self,
            submits.len() as u32,
            submits.as_ptr(),
            fence
        ) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkQueueSubmit"]
    fn vk_queeu_submit(
        queue: Queue,
        submit_count: u32,
        p_submits: *const RawSubmitInfo,
        fence: Fence
    ) -> VkResult;
}