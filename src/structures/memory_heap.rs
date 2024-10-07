use crate::{DeviceSize, MemoryHeapFlags};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags
}