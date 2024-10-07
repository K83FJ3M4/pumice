use crate::{Format, ImageTiling, ImageType, ImageUsageFlags, PhysicalDevice, SampleCountFlags, SparseImageFormatProperties};

impl PhysicalDevice {
    pub unsafe fn get_sparse_image_format_properties(
        self,
        format: Format,
        r#type: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
    ) -> Vec<SparseImageFormatProperties> {
        let mut property_count = 0;
        let mut properties = Vec::new();

        vk_get_physical_device_sparse_image_format_properties(
            self,
            format,
            r#type,
            samples,
            usage,
            tiling,
            &mut property_count,
            std::ptr::null_mut(),
        );

        properties.reserve(property_count as usize);
        properties.set_len(property_count as usize);

        vk_get_physical_device_sparse_image_format_properties(
            self,
            format,
            r#type,
            samples,
            usage,
            tiling,
            &mut property_count,
            properties.as_mut_ptr(),
        );

        properties
    }
}

extern "C" {
    #[link_name = "vkGetPhysicalDeviceSparseImageFormatProperties"]
    fn vk_get_physical_device_sparse_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        type_: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    );
}