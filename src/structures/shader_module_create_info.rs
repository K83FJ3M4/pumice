use crate::{ShaderModuleCreateFlags, StructureType};
use std::ffi::c_void;
use std::ptr::null;
use std::mem::size_of_val;

#[derive(Clone, Copy)]
pub struct ShaderModuleCreateInfo<'a> {
    pub code: &'a [u32]
}

#[repr(C)]
pub(crate) struct RawShaderModuleCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: ShaderModuleCreateFlags,
    pub(crate) code_size: usize,
    pub(crate) p_code: *const u32
}

impl<'a> ShaderModuleCreateInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawShaderModuleCreateInfo {
        RawShaderModuleCreateInfo {
            s_type: StructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: ShaderModuleCreateFlags::empty(),
            code_size: size_of_val(self.code),
            p_code: self.code.as_ptr()
        }
    }
}