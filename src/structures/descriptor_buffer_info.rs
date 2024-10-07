use crate::{Buffer, DeviceSize};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}