use crate::Bool32;

pub const ATTACHMENT_UNUSED: u32 = !0;
pub const LOG_CLAMP_NONE: f32 = 1000.0;
pub const MAX_DESCRIPTION_SIZE: usize = 256;
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const MAX_MEMORY_HEAPS: usize = 16;
pub const MAX_MEMORY_TYPES: usize = 32;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const UUID_SIZE: usize = 16;
pub const WHOLE_SIZE: u64 = !0;

#[allow(unused)]
pub(crate) const FALSE: Bool32 = 0;
#[allow(unused)]
pub(crate) const TRUE: Bool32 = 1;