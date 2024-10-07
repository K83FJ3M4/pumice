use super::{Extent3D, ImageSubresource, Offset3D};
use crate::{DeviceMemory, DeviceSize, SparseMemoryBindFlags};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags
}