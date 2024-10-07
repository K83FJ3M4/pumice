use crate::bitflags;

bitflags! { pub enum ImageCreateFlags:
    SparseBinding = 0x00000001;
    SparseResidency = 0x00000002;
    SparseAliased = 0x00000004;
    MutableFormat = 0x00000008;
    CubeCompatible = 0x00000010;
}