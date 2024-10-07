use crate::bitflags;

bitflags! { pub enum PipelineCreateFlags:
    DisableOptimization = 0x00000001;
    AllowDerivatives = 0x00000002;
    Derivative = 0x00000004;
}