use crate::Image;
use super::SparseMemoryBind;

#[derive(Clone, Copy)]
pub struct SparseImageOpaqueMemoryBindInfo<'a> {
    pub image: Image,
    pub binds: &'a [SparseMemoryBind]
}

#[repr(C)]
pub(crate) struct RawSparseImageOpaqueMemoryBindInfo {
    pub(crate) image: Image,
    pub(crate) bind_count: u32,
    pub(crate) p_binds: *const SparseMemoryBind
}

impl<'a> SparseImageOpaqueMemoryBindInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawSparseImageOpaqueMemoryBindInfo {
        RawSparseImageOpaqueMemoryBindInfo {
            image: self.image,
            bind_count: self.binds.len() as u32,
            p_binds: self.binds.as_ptr()
        }
    }
}