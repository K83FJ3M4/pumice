use crate::bitflags;

bitflags! { pub enum FormatFeatureFlags:
    SampledImage = 0x00000001;
    StorageImage = 0x00000002;
    StorageImageAtomic = 0x00000004;
    UniformTexelBuffer = 0x00000008;
    StorageTexelBuffer = 0x00000010;
    StorageTexelBufferAtomic = 0x00000020;
    VertexBuffer = 0x00000040;
    ColorAttachment = 0x00000080;
    ColorAttachmentBlend = 0x00000100;
    DepthStencilAttachment = 0x00000200;
    BlitSrc = 0x00000400;
    BlitDst = 0x00000800;
    SampledImageFilterLinear = 0x00001000; 
}