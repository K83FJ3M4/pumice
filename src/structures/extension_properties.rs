use super::CharArray;
use crate::MAX_EXTENSION_NAME_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExtensionProperties {
    pub extension_name: CharArray<MAX_EXTENSION_NAME_SIZE>,
    pub spec_version: u32
}