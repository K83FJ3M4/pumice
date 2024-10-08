use crate::bitflags;

bitflags! { pub enum ShaderStageFlags:
    Vertex = 0x00000001;
    TessellationControl = 0x00000002;
    TessellationEvaluation = 0x00000004;
    Geometry = 0x00000008;
    Fragment = 0x00000010;
    Compute = 0x00000020;
    AllGraphics = 0x0000001F;
    All = 0x7FFFFFFF;
}