use crate::Image;
use super::SparseMemoryBind;

#[derive(Clone, Copy)]
pub struct SparseImageMemoryBindInfo<'a> {
    pub image: Image,
    pub binds: &'a [SparseMemoryBind]
}

#[repr(C)]
pub(crate) struct RawSparseImageMemoryBindInfo {
    pub(crate) image: Image,
    pub(crate) bind_count: u32,
    pub(crate) p_binds: *const SparseMemoryBind
}

impl<'a> SparseImageMemoryBindInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawSparseImageMemoryBindInfo {
        RawSparseImageMemoryBindInfo {
            image: self.image,
            bind_count: self.binds.len() as u32,
            p_binds: self.binds.as_ptr()
        }
    }
}