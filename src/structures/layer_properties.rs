use crate::{MAX_EXTENSION_NAME_SIZE, MAX_DESCRIPTION_SIZE};
use super::CharArray;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LayerProperties {
    pub layer_name: CharArray<MAX_EXTENSION_NAME_SIZE>,
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: CharArray<MAX_DESCRIPTION_SIZE>
}