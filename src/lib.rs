mod bitflags;
mod constants;
mod enums;
mod objects;
mod structures;
mod commands;

pub use bitflags::*;
pub use constants::*;
pub use enums::*;
pub use objects::*;
pub use structures::*;
pub use commands::Result;
pub use commands::MappedMemory;

pub type DeviceAddress = u64;
pub type DeviceSize = u64;
pub type SampleMask = u32;
type Bool32 = u32;
type Flags = u32;

// TODO
// Add validation
// Add pNext fields and the ability to write extensions in other crates