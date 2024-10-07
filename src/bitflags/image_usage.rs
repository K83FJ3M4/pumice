use crate::bitflags;

bitflags! { pub enum ImageUsageFlags:
    TransferSrc = 0x00000001;
    TransferDst = 0x00000002;
    Sampled = 0x00000004;
    Storage = 0x00000008;
    ColorAttachment = 0x00000010;
    DepthStencilAttachment = 0x00000020;
    TransientAttachment = 0x00000040;
    InputAttachment = 0x00000080;
}