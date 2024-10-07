#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PipelineBindPoint {
    Graphics = 0,
    Compute = 1,
}