#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DynamicState {
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8,
}
