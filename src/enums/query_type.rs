#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    Occlusion = 0,
    PipelineStatistics = 1,
    Timestamp = 2,
}