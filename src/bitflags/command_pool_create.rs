use crate::bitflags;

bitflags! { pub enum CommandPoolCreateFlags:
    Transient = 0x00000001;
    ResetCommandBuffer = 0x00000002;
}