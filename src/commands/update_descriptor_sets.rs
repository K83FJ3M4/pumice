use crate::{CopyDescriptorSet, Device, RawCopyDescriptorSet, RawWriteDescriptorSet, WriteDescriptorSet};
use bumpalo::Bump;

impl Device {
    pub unsafe fn update_descriptor_sets(
        self,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet]
    ) {
        let bump = Bump::new();

        let descriptor_writes = bump.alloc_slice_fill_iter(
            descriptor_writes.into_iter().map(WriteDescriptorSet::into_raw)
        );

        let descriptor_copies = bump.alloc_slice_fill_iter(
            descriptor_copies.into_iter().map(CopyDescriptorSet::into_raw)
        );

        vk_update_descriptor_sets(
            self,
            descriptor_writes.len() as u32,
            descriptor_writes.as_ptr(),
            descriptor_copies.len() as u32,
            descriptor_copies.as_ptr()
        );
    }
}

extern "C" {
    #[link_name = "vkUpdateDescriptorSets"]
    fn vk_update_descriptor_sets(
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const RawWriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const RawCopyDescriptorSet
    );
}