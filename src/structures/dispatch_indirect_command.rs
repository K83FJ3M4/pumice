#[repr(C)]
#[derive(Clone, Copy)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32
}