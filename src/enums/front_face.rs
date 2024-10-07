#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}