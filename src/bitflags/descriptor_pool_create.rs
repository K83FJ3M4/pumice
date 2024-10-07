use crate::bitflags;

bitflags! { pub enum DescriptorPoolCreateFlags:
    FreeDescriptorSet = 0x00000001;
}