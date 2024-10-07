use crate::{Device, Extent2D, RenderPass};
use std::mem::MaybeUninit;

impl Device {
    pub unsafe fn get_render_area_granularity(self, render_pass: RenderPass) -> Extent2D {
        let mut granularity = MaybeUninit::uninit();
        vk_get_render_area_granularity(self, render_pass, granularity.as_mut_ptr());
        granularity.assume_init()
    }
}

extern "C" {
    #[link_name = "vkGetRenderAreaGranularity"]
    fn vk_get_render_area_granularity(
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    );
}