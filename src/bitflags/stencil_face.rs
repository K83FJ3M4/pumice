use crate::bitflags;

bitflags! { pub enum StencilFaceFlags:
    Front = 0x00000001;
    Back = 0x00000002;
    FrontAndBack = 0x00000003;
}