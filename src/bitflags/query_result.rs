use crate::bitflags;

bitflags! { pub enum QueryResultFlags:
    Result64 = 0x00000001;
    Wait = 0x00000002;
    WithAvailability = 0x00000004;
    Partial = 0x00000008;
}