mod access;
mod attachment_description;
mod buffer_create;
mod buffer_usage;
mod buffer_view_create;
mod color_component;
mod command_buffer_reset;
mod command_buffer_usage;
mod command_pool_create;
mod command_pool_reset;
mod cull_mode;
mod dependency;
mod descriptor_pool_create;
mod descriptor_pool_reset;
mod descriptor_set_layout_create;
mod device_create;
mod device_queue_create;
mod event_create;
mod fence_create;
mod format_feature;
mod framebuffer_create;
mod image_aspect;
mod image_create;
mod image_usage;
mod image_view_create;
mod instance_create;
mod memory_heap;
mod memory_map;
mod memory_property;
mod pipeline_cache_create;
mod pipeline_color_blend_state_create;
mod pipeline_create;
mod pipeline_depth_stencil_state_create;
mod pipeline_dynamic_state_create;
mod pipeline_input_assembly_state_create;
mod pipeline_layout_create;
mod pipeline_multisample_state_create;
mod pipeline_rasterization_state_create;
mod pipeline_shader_stage_create;
mod pipeline_stage;
mod pipeline_tessellation_state_create;
mod pipeline_vertex_input_state_create;
mod pipeline_viewport_state_create;
mod query_control;
mod query_pipeline_statistic;
mod query_pool_create;
mod query_result;
mod queue;
mod render_pass_create;
mod sample_count;
mod sampler_create;
mod semaphore_create;
mod shader_module_create;
mod shader_stage;
mod sparse_image_format;
mod sparse_memory_bind;
mod stencil_face;
mod subpass_description;

pub use access::AccessFlags;
pub use attachment_description::AttachmentDescriptionFlags;
pub use buffer_create::BufferCreateFlags;
pub use buffer_usage::BufferUsageFlags;
pub(crate) use buffer_view_create::BufferViewCreateFlags;
pub use color_component::ColorComponentFlags;
pub use command_buffer_reset::CommandBufferResetFlags;
pub use command_buffer_usage::CommandBufferUsageFlags;
pub use command_pool_create::CommandPoolCreateFlags;
pub use command_pool_reset::CommandPoolResetFlags;
pub use cull_mode::CullModeFlags;
pub use dependency::DependencyFlags;
pub use descriptor_pool_create::DescriptorPoolCreateFlags;
pub(crate) use descriptor_pool_reset::DescriptorPoolResetFlags;
pub(crate) use descriptor_set_layout_create::DescriptorSetLayoutCreateFlags;
pub(crate) use device_create::DeviceCreateFlags;
pub(crate) use device_queue_create::DeviceQueueCreateFlags;
pub(crate) use event_create::EventCreateFlags;
pub use fence_create::FenceCreateFlags;
pub use format_feature::FormatFeatureFlags;
pub(crate) use framebuffer_create::FramebufferCreateFlags;
pub use image_aspect::ImageAspectFlags;
pub use image_create::ImageCreateFlags;
pub use image_usage::ImageUsageFlags;
pub(crate) use image_view_create::ImageViewCreateFlags;
pub(crate) use instance_create::InstanceCreateFlags;
pub use memory_heap::MemoryHeapFlags;
pub(crate) use memory_map::MemoryMapFlags;
pub use memory_property::MemoryPropertyFlags;
pub(crate) use pipeline_cache_create::PipelineCacheCreateFlags;
pub(crate) use pipeline_color_blend_state_create::PipelineColorBlendStateCreateFlags;
pub use pipeline_create::PipelineCreateFlags;
pub(crate) use pipeline_depth_stencil_state_create::PipelineDepthStencilStateCreateFlags;
pub(crate) use pipeline_dynamic_state_create::PipelineDynamicStateCreateFlags;
pub(crate) use pipeline_input_assembly_state_create::PipelineInputAssemblyStateCreateFlags;
pub(crate) use pipeline_layout_create::PipelineLayoutCreateFlags;
pub(crate) use pipeline_multisample_state_create::PipelineMultisampleStateCreateFlags;
pub(crate) use pipeline_rasterization_state_create::PipelineRasterizationStateCreateFlags;
pub(crate) use pipeline_shader_stage_create::PipelineShaderStageCreateFlags;
pub use pipeline_stage::PipelineStageFlags;
pub(crate) use pipeline_tessellation_state_create::PipelineTessellationStateCreateFlags;
pub(crate) use pipeline_vertex_input_state_create::PipelineVertexInputStateCreateFlags;
pub(crate) use pipeline_viewport_state_create::PipelineViewportStateCreateFlags;
pub use query_control::QueryControlFlags;
pub use query_pipeline_statistic::QueryPipelineStatisticFlags;
pub(crate) use query_pool_create::QueryPoolCreateFlags;
pub use query_result::QueryResultFlags;
pub use queue::QueueFlags;
pub(crate) use render_pass_create::RenderPassCreateFlags;
pub use sample_count::SampleCountFlags;
pub(crate) use sampler_create::SamplerCreateFlags;
pub(crate) use semaphore_create::SemaphoreCreateFlags;
pub(crate) use shader_module_create::ShaderModuleCreateFlags;
pub use shader_stage::ShaderStageFlags;
pub use sparse_image_format::SparseImageFormatFlags;
pub use sparse_memory_bind::SparseMemoryBindFlags;
pub use stencil_face::StencilFaceFlags;
pub(crate) use subpass_description::SubpassDescriptionFlags;

#[macro_export]
macro_rules! bitflags {
    ($visibility:vis enum $name:ident: $($item:ident = $value:expr;)*) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        $visibility struct $name(crate::Flags);

        #[allow(unused)]
        #[allow(non_upper_case_globals)]
        impl $name {
            $visibility fn empty() -> Self {
                $name(0)
            }

            $($visibility const $item: Self = Self($value);)*
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                $name(self.0 & rhs.0)
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                $name(self.0 | rhs.0)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                $name(self.0 & !rhs.0)
            }
        }
    };
}