use crate::{Queue, VkResult};
use super::Result;

impl Queue {
    pub unsafe fn wait_idle(self) -> Result {
        match vk_queue_wait_idle(self) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkQueueWaitIdle"]
    fn vk_queue_wait_idle(
        queue: Queue
    ) -> VkResult;
}