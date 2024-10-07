use crate::{InstanceCreateFlags, StructureType};
use super::{str_into_raw, ApplicationInfo, RawApplicationInfo};
use std::ffi::{c_char, c_void};
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct InstanceCreateInfo<'a> {
    pub application_info: Option<ApplicationInfo<'a>>,
    pub enabled_layer_names: &'a [&'a str],
    pub enabled_extension_names: &'a [&'a str]
}

#[repr(C)]
pub(crate) struct RawInstanceCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: InstanceCreateFlags,
    pub(crate) p_application_info: *const RawApplicationInfo,
    pub(crate) enabled_layer_count: u32,
    pub(crate) pp_enabled_layer_names: *const *const c_char,
    pub(crate) enabled_extension_count: u32,
    pub(crate) pp_enabled_extension_names: *const *const c_char
}

impl<'a> InstanceCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawInstanceCreateInfo {
        let p_application_info = match self.application_info {
            Some(application_info) => bump.alloc(application_info.into_raw(bump)),
            None => null()
        };

        let enabled_layer_names = bump.alloc_slice_fill_iter(
            self.enabled_layer_names.into_iter().map(|s| str_into_raw(s, bump))
        );

        let enabled_extension_names = bump.alloc_slice_fill_iter(
            self.enabled_extension_names.into_iter().map(|s| str_into_raw(s, bump))
        );

        RawInstanceCreateInfo {
            s_type: StructureType::InstanceCreateInfo,
            p_next: null(),
            flags: InstanceCreateFlags::empty(),
            p_application_info,
            enabled_layer_count: self.enabled_layer_names.len() as u32,
            pp_enabled_layer_names: enabled_layer_names.as_ptr(),
            enabled_extension_count: self.enabled_extension_names.len() as u32,
            pp_enabled_extension_names: enabled_extension_names.as_ptr()
        }
    }
}