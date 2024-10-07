use crate::bitflags;

bitflags! { pub enum SparseImageFormatFlags:
    SingleMiptail = 0x00000001;
    AlignedMipSize = 0x00000002;
    NonstandardBlockSize = 0x00000004;
}