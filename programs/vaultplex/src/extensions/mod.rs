use anchor_lang::prelude::*;

pub mod access_control_extension;
pub use access_control_extension::*;

pub mod lock_extension;
pub use lock_extension::*;

pub mod time_interval_extension;
pub use time_interval_extension::*;

// Enum to represent different extension types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ExtensionType {
    AccessControlExtension,
    LockExtension,
    TimeIntervalExtension,
    // Future extensions can be added here
}

// Helper to get the size of each extension
impl ExtensionType {
    pub fn size(&self) -> usize {
        match self {
            ExtensionType::AccessControlExtension => AccessControlExtension::SIZE,
            ExtensionType::LockExtension => LockExtension::SIZE,
            ExtensionType::TimeIntervalExtension => TimeIntervalExtension::SIZE,
            // Add sizes for future extensions here
        }
    }
}