use crate::bitflags;

bitflags! { pub enum BufferCreateFlags:
    SparseBinding = 0x00000001;
    SparseResidency = 0x00000002;
    SparseAliased = 0x00000004;
}