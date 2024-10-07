#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize
}