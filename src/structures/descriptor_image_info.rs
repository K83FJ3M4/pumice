use crate::{ImageLayout, ImageView, Sampler};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout
}