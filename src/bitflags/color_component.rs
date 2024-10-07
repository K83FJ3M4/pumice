use crate::bitflags;

bitflags! { pub enum ColorComponentFlags:
    R = 0x00000001;
    G = 0x00000002;
    B = 0x00000004;
    A = 0x00000008;
}