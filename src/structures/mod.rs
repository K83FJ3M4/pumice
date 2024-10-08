mod allocation_callbacks;
mod application_info;
mod attachment_description;
mod attachment_reference;
mod bind_sparse_info;
mod sparse_memory_bind;
mod sparse_buffer_memory_bind_info;
mod sparse_image_opaque_memory_bind_info;
mod sparse_image_memory_bind_info;
mod buffer_copy;
mod buffer_create_info;
mod image_subresource_layers;
mod offset_3d;
mod extent_3d;
mod buffer_image_copy;
mod buffer_memory_barrier;
mod buffer_view_create_info;
mod clear_depth_stencil_value;
mod clear_color_value;
mod clear_value;
mod clear_attachment;
mod offset_2d;
mod extent_2d;
mod rect_2d;
mod clear_rect;
mod command_buffer_inheritance_info;
mod command_buffer_allocate_info;
mod command_buffer_begin_info;
mod command_pool_create_info;
mod component_mapping;
mod specialization_map_entry;
mod specialization_info;
mod pipeline_shader_stage_create_info;
mod compute_pipeline_create_info;
mod copy_descriptor_set;
mod descriptor_buffer_info;
mod descriptor_image_info;
mod descriptor_pool_size;
mod descriptor_pool_create_info;
mod descriptor_set_allocate_info;
mod descriptor_set_layout_binding;
mod descriptor_set_layout_create_info;
mod device_create_info;
mod device_queue_create_info;
mod physical_device_features;
mod dispatch_indirect_command;
mod draw_indexed_indirect_command;
mod draw_indirect_command;
mod event_create_info;
mod extension_properties;
mod fence_create_info;
mod format_properties;
mod framebuffer_create_info;
mod viewport;
mod stencil_op_state;
mod vertex_input_binding_description;
mod vertex_input_attribute_description;
mod pipeline_vertex_input_state_create_info;
mod pipeline_input_assembly_state_create_info;
mod pipeline_tessellation_state_create_info;
mod pipeline_viewport_state_create_info;
mod pipeline_rasterization_state_create_info;
mod pipeline_multisample_state_create_info;
mod pipeline_depth_stencil_state_create_info;
mod pipeline_color_blend_attachment_state;
mod pipeline_color_blend_state_create_info;
mod pipeline_dynamic_state_create_info;
mod graphics_pipeline_create_info;
mod image_blit;
mod image_copy;
mod image_create_info;
mod image_format_properties;
mod image_memory_barrier;
mod image_subresource_range;
mod image_resolve;
mod image_subresource;
mod image_view_create_info;
mod instance_create_info;
mod layer_properties;
mod mapped_memory_range;
mod memory_allocate_info;
mod memory_barrier;
mod memory_heap;
mod memory_requirements;
mod memory_type;
mod physical_device_limits;
mod physical_device_memory_properties;
mod physical_device_sparse_properties;
mod physical_device_properties;
mod pipeline_cache_create_info;
mod pipeline_cache_header_version_one;
mod push_constant_range;
mod query_pool_create_info;
mod queue_family_properties;
mod render_pass_begin_info;
mod subpass_description;
mod subpass_dependency;
mod render_pass_create_info;
mod sampler_create_info;
mod semaphore_create_info;
mod sparse_image_format_properties;
mod sparse_image_memory_bind;
mod sparse_image_memory_requirements;
mod submit_info;
mod subresource_layout;
mod write_descriptor_set;
mod pipeline_layout_create_info;
mod shader_module_create_info;

pub(crate) use allocation_callbacks::AllocationCallbacks;
pub(crate) use application_info::RawApplicationInfo;
pub(crate) use bind_sparse_info::RawBindSparseInfo;
pub(crate) use sparse_buffer_memory_bind_info::RawSparseBufferMemoryBindInfo;
pub(crate) use sparse_image_opaque_memory_bind_info::RawSparseImageOpaqueMemoryBindInfo;
pub(crate) use sparse_image_memory_bind_info::RawSparseImageMemoryBindInfo;
pub(crate) use buffer_create_info::RawBufferCreateInfo;
pub(crate) use buffer_memory_barrier::RawBufferMemoryBarrier;
pub(crate) use buffer_view_create_info::RawBufferViewCreateInfo;
pub(crate) use command_buffer_inheritance_info::RawCommandBufferInheritanceInfo;
pub(crate) use command_buffer_allocate_info::RawCommandBufferAllocateInfo;
pub(crate) use command_buffer_begin_info::RawCommandBufferBeginInfo;
pub(crate) use command_pool_create_info::RawCommandPoolCreateInfo;
pub(crate) use specialization_info::RawSpecializationInfo;
pub(crate) use descriptor_set_layout_binding::RawDescriptorSetLayoutBinding;
pub(crate) use pipeline_shader_stage_create_info::RawPipelineShaderStageCreateInfo;
pub(crate) use compute_pipeline_create_info::RawComputePipelineCreateInfo;
pub(crate) use copy_descriptor_set::RawCopyDescriptorSet;
pub(crate) use descriptor_pool_create_info::RawDescriptorPoolCreateInfo;
pub(crate) use descriptor_set_allocate_info::RawDescriptorSetAllocateInfo;
pub(crate) use descriptor_set_layout_create_info::RawDescriptorSetLayoutCreateInfo;
pub(crate) use device_queue_create_info::RawDeviceQueueCreateInfo;
pub(crate) use physical_device_features::RawPhysicalDeviceFeatures;
pub(crate) use device_create_info::RawDeviceCreateInfo;
pub(crate) use event_create_info::RawEventCreateInfo;
pub(crate) use fence_create_info::RawFenceCreateInfo;
pub(crate) use framebuffer_create_info::RawFramebufferCreateInfo;
pub(crate) use pipeline_vertex_input_state_create_info::RawPipelineVertexInputStateCreateInfo;
pub(crate) use pipeline_input_assembly_state_create_info::RawPipelineInputAssemblyStateCreateInfo;
pub(crate) use pipeline_tessellation_state_create_info::RawPipelineTessellationStateCreateInfo;
pub(crate) use pipeline_viewport_state_create_info::RawPipelineViewportStateCreateInfo;
pub(crate) use pipeline_rasterization_state_create_info::RawPipelineRasterizationStateCreateInfo;
pub(crate) use pipeline_multisample_state_create_info::RawPipelineMultisampleStateCreateInfo;
pub(crate) use pipeline_depth_stencil_state_create_info::RawPipelineDepthStencilStateCreateInfo;
pub(crate) use pipeline_color_blend_state_create_info::RawPipelineColorBlendStateCreateInfo;
pub(crate) use pipeline_dynamic_state_create_info::RawPipelineDynamicStateCreateInfo;
pub(crate) use graphics_pipeline_create_info::RawGraphicsPipelineCreateInfo;
pub(crate) use image_create_info::RawImageCreateInfo;
pub(crate) use image_memory_barrier::RawImageMemoryBarrier;
pub(crate) use image_view_create_info::RawImageViewCreateInfo;
pub(crate) use instance_create_info::RawInstanceCreateInfo;
pub(crate) use mapped_memory_range::RawMappedMemoryRange;
pub(crate) use memory_allocate_info::RawMemoryAllocateInfo;
pub(crate) use memory_barrier::RawMemoryBarrier;
pub(crate) use physical_device_limits::RawPhysicalDeviceLimits;
pub(crate) use physical_device_memory_properties::RawPhysicalDeviceMemoryProperties;
pub(crate) use physical_device_sparse_properties::RawPhysicalDeviceSparseProperties;
pub(crate) use physical_device_properties::RawPhysicalDeviceProperties;
pub(crate) use pipeline_cache_create_info::RawPipelineCacheCreateInfo;
pub(crate) use query_pool_create_info::RawQueryPoolCreateInfo;
pub(crate) use render_pass_begin_info::RawRenderPassBeginInfo;
pub(crate) use subpass_description::RawSubpassDescription;
pub(crate) use render_pass_create_info::RawRenderPassCreateInfo;
pub(crate) use sampler_create_info::RawSamplerCreateInfo;
pub(crate) use semaphore_create_info::RawSemaphoreCreateInfo;
pub(crate) use submit_info::RawSubmitInfo;
pub(crate) use write_descriptor_set::RawWriteDescriptorSet;
pub(crate) use pipeline_layout_create_info::RawPipelineLayoutCreateInfo;
pub(crate) use shader_module_create_info::RawShaderModuleCreateInfo;

pub use application_info::ApplicationInfo;
pub use attachment_description::AttachmentDescription;
pub use attachment_reference::AttachmentReference;
pub use bind_sparse_info::BindSparseInfo;
pub use sparse_memory_bind::SparseMemoryBind;
pub use sparse_buffer_memory_bind_info::SparseBufferMemoryBindInfo;
pub use sparse_image_opaque_memory_bind_info::SparseImageOpaqueMemoryBindInfo;
pub use sparse_image_memory_bind_info::SparseImageMemoryBindInfo;
pub use buffer_copy::BufferCopy;
pub use buffer_create_info::BufferCreateInfo;
pub use image_subresource_layers::ImageSubresourceLayers;
pub use offset_3d::Offset3D;
pub use extent_3d::Extent3D;
pub use buffer_image_copy::BufferImageCopy;
pub use buffer_memory_barrier::BufferMemoryBarrier;
pub use buffer_view_create_info::BufferViewCreateInfo;
pub use clear_depth_stencil_value::ClearDepthStencilValue;
pub use clear_color_value::ClearColorValue;
pub use clear_value::ClearValue;
pub use clear_attachment::ClearAttachment;
pub use offset_2d::Offset2D;
pub use extent_2d::Extent2D;
pub use rect_2d::Rect2D;
pub use clear_rect::ClearRect;
pub use command_buffer_inheritance_info::CommandBufferInheritanceInfo;
pub use command_buffer_allocate_info::CommandBufferAllocateInfo;
pub use command_buffer_begin_info::CommandBufferBeginInfo;
pub use command_pool_create_info::CommandPoolCreateInfo;
pub use component_mapping::ComponentMapping;
pub use specialization_map_entry::SpecializationMapEntry;
pub use specialization_info::SpecializationInfo;
pub use pipeline_shader_stage_create_info::PipelineShaderStageCreateInfo;
pub use compute_pipeline_create_info::ComputePipelineCreateInfo;
pub use copy_descriptor_set::CopyDescriptorSet;
pub use descriptor_buffer_info::DescriptorBufferInfo;
pub use descriptor_image_info::DescriptorImageInfo;
pub use descriptor_pool_size::DescriptorPoolSize;
pub use descriptor_pool_create_info::DescriptorPoolCreateInfo;
pub use descriptor_set_allocate_info::DescriptorSetAllocateInfo;
pub use descriptor_set_layout_binding::DescriptorSetLayoutBinding;
pub use descriptor_set_layout_create_info::DescriptorSetLayoutCreateInfo;
pub use device_queue_create_info::DeviceQueueCreateInfo;
pub use physical_device_features::PhysicalDeviceFeatures;
pub use device_create_info::DeviceCreateInfo;
pub use dispatch_indirect_command::DispatchIndirectCommand;
pub use draw_indexed_indirect_command::DrawIndexedIndirectCommand;
pub use draw_indirect_command::DrawIndirectCommand;
pub use extension_properties::ExtensionProperties;
pub use fence_create_info::FenceCreateInfo;
pub use format_properties::FormatProperties;
pub use framebuffer_create_info::FramebufferCreateInfo;
pub use viewport::Viewport;
pub use stencil_op_state::StencilOpState;
pub use vertex_input_binding_description::VertexInputBindingDescription;
pub use vertex_input_attribute_description::VertexInputAttributeDescription;
pub use pipeline_vertex_input_state_create_info::PipelineVertexInputStateCreateInfo;
pub use pipeline_input_assembly_state_create_info::PipelineInputAssemblyStateCreateInfo;
pub use pipeline_tessellation_state_create_info::PipelineTessellationStateCreateInfo;
pub use pipeline_viewport_state_create_info::PipelineViewportStateCreateInfo;
pub use pipeline_rasterization_state_create_info::PipelineRasterizationStateCreateInfo;
pub use pipeline_multisample_state_create_info::PipelineMultisampleStateCreateInfo;
pub use pipeline_depth_stencil_state_create_info::PipelineDepthStencilStateCreateInfo;
pub use pipeline_color_blend_attachment_state::PipelineColorBlendAttachmentState;
pub use pipeline_color_blend_state_create_info::PipelineColorBlendStateCreateInfo;
pub use pipeline_dynamic_state_create_info::PipelineDynamicStateCreateInfo;
pub use graphics_pipeline_create_info::GraphicsPipelineCreateInfo;
pub use image_blit::ImageBlit;
pub use image_copy::ImageCopy;
pub use image_create_info::ImageCreateInfo;
pub use image_format_properties::ImageFormatProperties;
pub use image_memory_barrier::ImageMemoryBarrier;
pub use image_subresource_range::ImageSubresourceRange;
pub use image_resolve::ImageResolve;
pub use image_subresource::ImageSubresource;
pub use image_view_create_info::ImageViewCreateInfo;
pub use instance_create_info::InstanceCreateInfo;
pub use layer_properties::LayerProperties;
pub use mapped_memory_range::MappedMemoryRange;
pub use memory_allocate_info::MemoryAllocateInfo;
pub use memory_barrier::MemoryBarrier;
pub use memory_heap::MemoryHeap;
pub use memory_requirements::MemoryRequirements;
pub use memory_type::MemoryType;
pub use physical_device_limits::PhysicalDeviceLimits;
pub use physical_device_memory_properties::PhysicalDeviceMemoryProperties;
pub use physical_device_sparse_properties::PhysicalDeviceSparseProperties;
pub use physical_device_properties::PhysicalDeviceProperties;
pub use pipeline_cache_create_info::PipelineCacheCreateInfo;
pub use pipeline_cache_header_version_one::PipelineCacheHeaderVersionOne;
pub use push_constant_range::PushConstantRange;
pub use query_pool_create_info::QueryPoolCreateInfo;
pub use queue_family_properties::QueueFamilyProperties;
pub use render_pass_begin_info::RenderPassBeginInfo;
pub use subpass_description::SubpassDescription;
pub use subpass_dependency::SubpassDependency;
pub use render_pass_create_info::RenderPassCreateInfo;
pub use sampler_create_info::SamplerCreateInfo;
pub use sparse_image_format_properties::SparseImageFormatProperties;
pub use sparse_image_memory_bind::SparseImageMemoryBind;
pub use sparse_image_memory_requirements::SparseImageMemoryRequirements;
pub use submit_info::SubmitInfo;
pub use subresource_layout::SubresourceLayout;
pub use write_descriptor_set::WriteDescriptorSet;
pub use pipeline_layout_create_info::PipelineLayoutCreateInfo;
pub use shader_module_create_info::ShaderModuleCreateInfo;

use std::ffi::c_char;
use std::ops::{Deref, DerefMut};
use bumpalo::Bump;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CharArray<const C: usize> {
    pub(crate) data: [c_char; C],
}

#[derive(Clone, Copy)]
pub struct Array<const C: usize, T> {
    pub(crate) count: u32,
    pub(crate) data: [T; C],
}

pub(crate) fn str_into_raw(s: &str, bump: &Bump) -> *const c_char {
    if s.contains('\0') { panic!("String contains null character"); }
    let data = bump.alloc_slice_fill_default(s.len() + 1);
    data[..s.len()].copy_from_slice(s.as_bytes());
    return data.as_ptr() as *const c_char;
}

impl<const C: usize> Deref for CharArray<C> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::str::from_utf8_unchecked(
                std::slice::from_raw_parts(
                    self.data.as_ptr() as *const u8,
                    self.data.into_iter().position(|c| c == 0)
                        .unwrap_or(self.data.len())
                )
            )
        }
    }
}

impl<const C: usize, T> Deref for Array<C, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[..self.count as usize]
    }
}

impl<const C: usize, T> DerefMut for Array<C, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..self.count as usize]
    }
}