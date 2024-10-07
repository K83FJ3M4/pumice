use crate::StructureType;
use std::ffi::{c_char, c_void};
use std::ptr::null;
use super::str_into_raw;
use bumpalo::Bump;

#[derive(Clone, Copy)]
pub struct ApplicationInfo<'a> {
    pub application_name: Option<&'a str>,
    pub application_version: u32,
    pub engine_name: Option<&'a str>,
    pub engine_version: u32
}

#[repr(C)]
pub(crate) struct RawApplicationInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) p_application_name: *const c_char,
    pub(crate) application_version: u32,
    pub(crate) p_engine_name: *const c_char,
    pub(crate) engine_version: u32,
    pub(crate) api_version: u32
}

impl<'a> ApplicationInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawApplicationInfo {
        let p_application_name = match self.application_name {
            Some(application_name) => str_into_raw(application_name, bump),
            None => null()
        };

        let p_engine_name = match self.engine_name {
            Some(engine_name) => str_into_raw(engine_name, bump),
            None => null()
        };

        RawApplicationInfo {
            s_type: StructureType::ApplicationInfo,
            p_next: null(),
            p_application_name,
            application_version: self.application_version,
            p_engine_name,
            engine_version: self.engine_version,
            api_version: 1 << 22
        }
    }
}