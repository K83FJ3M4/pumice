use super::SpecializationMapEntry;
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct SpecializationInfo<'a> {
    pub map_entries: &'a [SpecializationMapEntry],
    pub data: &'a [u8]
}

#[repr(C)]
pub(crate) struct RawSpecializationInfo {
    pub(crate) map_entry_count: u32,
    pub(crate) p_map_entries: *const SpecializationMapEntry,
    pub(crate) data_size: usize,
    pub(crate) p_data: *const c_void
}

impl<'a> SpecializationInfo<'a> {
    pub(crate) fn into_raw(self) -> RawSpecializationInfo {
        RawSpecializationInfo {
            map_entry_count: self.map_entries.len() as u32,
            p_map_entries: self.map_entries.as_ptr(),
            data_size: self.data.len(),
            p_data: self.data.as_ptr() as *const c_void
        }
    }
}