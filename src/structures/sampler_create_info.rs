use crate::{Bool32, BorderColor, CompareOp, Filter, SamplerAddressMode, SamplerCreateFlags, SamplerMipmapMode, StructureType, FALSE, TRUE};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct SamplerCreateInfo {
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: bool,
    pub max_anisotropy: f32,
    pub compare_enable: bool,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: bool
}

#[repr(C)]
pub(crate) struct RawSamplerCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: SamplerCreateFlags,
    pub(crate) mag_filter: Filter,
    pub(crate) min_filter: Filter,
    pub(crate) mipmap_mode: SamplerMipmapMode,
    pub(crate) address_mode_u: SamplerAddressMode,
    pub(crate) address_mode_v: SamplerAddressMode,
    pub(crate) address_mode_w: SamplerAddressMode,
    pub(crate) mip_lod_bias: f32,
    pub(crate) anisotropy_enable: Bool32,
    pub(crate) max_anisotropy: f32,
    pub(crate) compare_enable: Bool32,
    pub(crate) compare_op: CompareOp,
    pub(crate) min_lod: f32,
    pub(crate) max_lod: f32,
    pub(crate) border_color: BorderColor,
    pub(crate) unnormalized_coordinates: Bool32
}

impl SamplerCreateInfo {
    pub(crate) fn into_raw(&self) -> RawSamplerCreateInfo {
        RawSamplerCreateInfo {
            s_type: StructureType::SamplerCreateInfo,
            p_next: null(),
            flags: SamplerCreateFlags::empty(),
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_mode: self.mipmap_mode,
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mip_lod_bias: self.mip_lod_bias,
            anisotropy_enable: if self.anisotropy_enable { TRUE } else { FALSE },
            max_anisotropy: self.max_anisotropy,
            compare_enable: if self.compare_enable { TRUE } else { FALSE },
            compare_op: self.compare_op,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            border_color: self.border_color,
            unnormalized_coordinates: if self.unnormalized_coordinates { TRUE } else { FALSE }
        }
    }
}