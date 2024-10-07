use crate::bitflags;

bitflags! { pub enum MemoryPropertyFlags:
    DeviceLocal = 0x00000001;
    HostVisible = 0x00000002;
    HostCoherent = 0x00000004;
    HostCached = 0x00000008;
    LazilyAllocated = 0x00000010;
}
