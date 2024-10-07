use crate::bitflags;

bitflags! { pub enum FenceCreateFlags:
    Signaled = 0x00000001;
}