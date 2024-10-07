use crate::bitflags;

bitflags! { pub enum CommandBufferUsageFlags:
    OneTimeSubmit = 0x00000001;
    RenderPassContinue = 0x00000002;
    SimultaneousUse = 0x00000004;
}