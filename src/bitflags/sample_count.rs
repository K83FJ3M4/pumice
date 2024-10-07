use crate::bitflags;

bitflags! { pub enum SampleCountFlags:
    Count1 = 0x00000001;
    Count2 = 0x00000002;
    Count4 = 0x00000004;
    Count8 = 0x00000008;
    Count16 = 0x00000010;
    Count32 = 0x00000020;
    Count64 = 0x00000040;
}