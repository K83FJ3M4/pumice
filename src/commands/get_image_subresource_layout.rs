use crate::{Device, Image, ImageSubresource, SubresourceLayout};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_image_subresource_layout(self, image: Image, subresource: &ImageSubresource) -> SubresourceLayout {
        let mut layout = MaybeUninit::uninit();
        vk_get_image_subresource_layout(self, image, subresource, layout.as_mut_ptr());
        layout.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetImageSubresourceLayout"]
    fn vk_get_image_subresource_layout(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    );
}