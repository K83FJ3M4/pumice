use crate::bitflags;

bitflags! { pub enum BufferUsageFlags:
    TransferSrc = 0x00000001;
    TransferDst = 0x00000002;
    UniformTexelBuffer = 0x00000004;
    StorageTexelBuffer = 0x00000008;
    UniformBuffer = 0x00000010;
    StorageBuffer = 0x00000020;
    IndexBuffer = 0x00000040;
    VertexBuffer = 0x00000080;
    IndirectBuffer = 0x00000100;
}