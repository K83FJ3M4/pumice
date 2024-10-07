use crate::{Semaphore, StructureType};
use std::ffi::c_void;
use super::{RawSparseBufferMemoryBindInfo, RawSparseImageMemoryBindInfo, RawSparseImageOpaqueMemoryBindInfo, SparseBufferMemoryBindInfo, SparseImageMemoryBindInfo, SparseImageOpaqueMemoryBindInfo};
use std::ptr::null;
use bumpalo::Bump;

#[derive(Clone, Copy)]
pub struct BindSparseInfo<'a> {
    pub wait_semaphores: &'a [Semaphore],
    pub buffer_binds: &'a [SparseBufferMemoryBindInfo<'a>],
    pub image_opaque_binds: &'a [SparseImageOpaqueMemoryBindInfo<'a>],
    pub image_binds: &'a [SparseImageMemoryBindInfo<'a>],
    pub signal_semaphores: &'a [Semaphore]
}

#[repr(C)]
pub(crate) struct RawBindSparseInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) wait_semaphore_count: u32,
    pub(crate) p_wait_semaphores: *const Semaphore,
    pub(crate) buffer_bind_count: u32,
    pub(crate) p_buffer_binds: *const RawSparseBufferMemoryBindInfo,
    pub(crate) image_opaque_bind_count: u32,
    pub(crate) p_image_opaque_binds: *const RawSparseImageOpaqueMemoryBindInfo,
    pub(crate) image_bind_count: u32,
    pub(crate) p_image_binds: *const RawSparseImageMemoryBindInfo,
    pub(crate) signal_semaphore_count: u32,
    pub(crate) p_signal_semaphores: *const Semaphore
}

impl<'a> BindSparseInfo<'a> {
    pub(crate) fn into_raw(&self, bump: &Bump) -> RawBindSparseInfo {
        let buffer_binds = bump.alloc_slice_fill_iter(
            self.buffer_binds.into_iter().map(SparseBufferMemoryBindInfo::into_raw)
        ); 

        let image_opaque_binds = bump.alloc_slice_fill_iter(
            self.image_opaque_binds.into_iter().map(SparseImageOpaqueMemoryBindInfo::into_raw)
        );

        let image_binds = bump.alloc_slice_fill_iter(
            self.image_binds.into_iter().map(SparseImageMemoryBindInfo::into_raw)
        );

        RawBindSparseInfo {
            s_type: StructureType::BindSparseInfo,
            p_next: null(),
            wait_semaphore_count: self.wait_semaphores.len() as u32,
            p_wait_semaphores: self.wait_semaphores.as_ptr(),
            buffer_bind_count: buffer_binds.len() as u32,
            p_buffer_binds: buffer_binds.as_ptr(),
            image_opaque_bind_count: image_opaque_binds.len() as u32,
            p_image_opaque_binds: image_opaque_binds.as_ptr(),
            image_bind_count: image_binds.len() as u32,
            p_image_binds: image_binds.as_ptr(),
            signal_semaphore_count: self.signal_semaphores.len() as u32,
            p_signal_semaphores: self.signal_semaphores.as_ptr()
        }
    }
}