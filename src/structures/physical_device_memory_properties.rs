use super::{Array, MemoryHeap, MemoryType};
use crate::{MAX_MEMORY_TYPES, MAX_MEMORY_HEAPS};

#[derive(Clone, Copy)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_types: Array<MAX_MEMORY_TYPES, MemoryType>,
    pub memory_heaps: Array<MAX_MEMORY_HEAPS, MemoryHeap>
}
#[repr(C)]
pub(crate) struct RawPhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS]
}

impl PhysicalDeviceMemoryProperties {
    pub(crate) fn from_raw(raw: RawPhysicalDeviceMemoryProperties) -> PhysicalDeviceMemoryProperties {
        PhysicalDeviceMemoryProperties {
            memory_types: Array {
                data: raw.memory_types,
                count: raw.memory_type_count
            },
            memory_heaps: Array {
                data: raw.memory_heaps,
                count: raw.memory_heap_count
            }
        }
    }
}