use crate::ComponentSwizzle;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle
}