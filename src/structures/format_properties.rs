use crate::FormatFeatureFlags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags
}