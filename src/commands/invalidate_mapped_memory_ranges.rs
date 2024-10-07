use crate::{Device, MappedMemoryRange, RawMappedMemoryRange, VkResult};
use super::Result;
use bumpalo::Bump;

impl Device {
    pub unsafe fn invalidate_mapped_memory_ranges(self, memory_ranges: &[MappedMemoryRange]) -> Result {
        let bump = Bump::new();
        let memory_ranges = bump.alloc_slice_fill_iter(
            memory_ranges.into_iter().map(MappedMemoryRange::into_raw)
        );

        match vk_invalidate_mapped_memory_ranges(
            self,
            memory_ranges.len() as u32,
            memory_ranges.as_ptr()
        ) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}

extern "C" {
    #[link_name = "vkInvalidateMappedMemoryRanges"]
    fn vk_invalidate_mapped_memory_ranges(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const RawMappedMemoryRange
    ) -> VkResult;
}