use crate::bitflags;

bitflags! { pub enum CommandPoolResetFlags:
    ReleaseResources = 0x00000001;
}