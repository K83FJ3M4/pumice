use crate::{DeviceCreateFlags, StructureType};
use super::{str_into_raw, DeviceQueueCreateInfo, PhysicalDeviceFeatures, RawDeviceQueueCreateInfo, RawPhysicalDeviceFeatures};
use std::ffi::{c_char, c_void};
use bumpalo::Bump;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct DeviceCreateInfo<'a> {
    pub queue_create_infos: &'a [DeviceQueueCreateInfo<'a>],
    pub enabled_extension_names: &'a [&'a str],
    pub enabled_features: Option<PhysicalDeviceFeatures>
}

#[repr(C)]
pub(crate) struct RawDeviceCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: DeviceCreateFlags,
    pub(crate) queue_create_info_count: u32,
    pub(crate) p_queue_create_infos: *const RawDeviceQueueCreateInfo,
    pub(crate) enabled_layer_count: u32,
    pub(crate) pp_enabled_layer_names: *const *const c_char,
    pub(crate) enabled_extension_count: u32,
    pub(crate) pp_enabled_extension_names: *const *const c_char,
    pub(crate) p_enabled_features: *const RawPhysicalDeviceFeatures
}

impl<'a> DeviceCreateInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawDeviceCreateInfo {
        let queue_create_infos = bump.alloc_slice_fill_iter(
            self.queue_create_infos.into_iter().map(DeviceQueueCreateInfo::into_raw)
        );

        let enabled_extension_names = bump.alloc_slice_fill_iter(
            self.enabled_extension_names.into_iter().map(|s| str_into_raw(s, bump))
        );

        let enabled_features = match self.enabled_features {
            Some(enabled_features) => bump.alloc(enabled_features.into_raw()),
            None => null()
        };

        RawDeviceCreateInfo {
            s_type: StructureType::DeviceCreateInfo,
            p_next: null(),
            flags: DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: null(),
            enabled_extension_count: enabled_extension_names.len() as u32,
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
            p_enabled_features: enabled_features
        }
    }
}