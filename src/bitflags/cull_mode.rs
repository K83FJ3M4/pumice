use crate::bitflags;

bitflags! { pub enum CullModeFlags:
    None = 0x00000000;
    Front = 0x00000001;
    Back = 0x00000002;
    FrontAndBack = 0x00000003;
}