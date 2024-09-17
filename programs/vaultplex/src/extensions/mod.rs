use anchor_lang::prelude::*;

/* pub mod access_control_extension;
pub use access_control_extension::*; */

pub mod lock_extension;
pub use lock_extension::*;

pub mod time_interval_extension;
pub use time_interval_extension::*;

use crate::Vault;

// Constants for extension sizes
// pub const DEPOSIT_EXTENSION_SIZE: usize = 24; // Fixed size for the DepositExtension (3 u64 fields)
// pub const TIME_INTERVAL_EXTENSION_SIZE: usize = 16; // u64 (8) + u64 (8)

// Enum to represent different extension types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ExtensionType {
    None,
    LockExtension,
    TimeIntervalExtension,
    /* DepositExtension, */
    // AccessControlExtension,
    // Future extensions can be added here
}

// Helper to get the size of each extension
impl ExtensionType {
    pub fn size(&self) -> usize {
        match self {
            ExtensionType::None => 0,
            ExtensionType::LockExtension => LOCK_EXTENSION_SIZE,
            ExtensionType::TimeIntervalExtension => TIME_INTERVAL_EXTENSION_SIZE,
            // ExtensionType::DepositExtension => DEPOSIT_EXTENSION_SIZE,
            /* ExtensionType::AccessControlExtension => AccessControlExtension::SIZE,
            ExtensionType::LockExtension => LockExtension::SIZE,
            ExtensionType::TimeIntervalExtension => TimeIntervalExtension::SIZE, */
            // Add sizes for future extensions here
        }
    }

    pub fn offset(&self) -> usize {
        match self {
            ExtensionType::None => 0,
            ExtensionType::LockExtension => Vault::BASE_SIZE/*  + DEPOSIT_EXTENSION_SIZE */,
            ExtensionType::TimeIntervalExtension => Vault::BASE_SIZE /* + DEPOSIT_EXTENSION_SIZE */ + LOCK_EXTENSION_SIZE,
            // ExtensionType::DepositExtension => Vault::BASE_SIZE,
        }
    }
}