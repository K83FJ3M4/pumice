use crate::bitflags;

bitflags! { pub enum CommandBufferResetFlags:
    ReleaseResources = 0x00000001;
}