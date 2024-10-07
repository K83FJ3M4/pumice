use crate::Buffer;
use super::SparseMemoryBind;

#[derive(Clone, Copy)]
pub struct SparseBufferMemoryBindInfo<'a> {
    pub buffer: Buffer,
    pub binds: &'a [SparseMemoryBind]
}

#[repr(C)]
pub(crate) struct RawSparseBufferMemoryBindInfo {
    pub(crate) buffer: Buffer,
    pub(crate) bind_count: u32,
    pub(crate) p_binds: *const SparseMemoryBind
}

impl<'a> SparseBufferMemoryBindInfo<'a> {
    pub(crate) fn into_raw(&self) -> RawSparseBufferMemoryBindInfo {
        RawSparseBufferMemoryBindInfo {
            buffer: self.buffer,
            bind_count: self.binds.len() as u32,
            p_binds: self.binds.as_ptr()
        }
    }
}