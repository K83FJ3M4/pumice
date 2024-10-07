use crate::MemoryPropertyFlags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32
}