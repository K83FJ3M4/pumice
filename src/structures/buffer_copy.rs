use crate::DeviceSize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize
}