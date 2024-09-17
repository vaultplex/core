use anchor_lang::prelude::*;

pub const TIME_INTERVAL_EXTENSION_SIZE: usize = 16; // u64 (8) + u64 (8)

// Time Interval Extension Definition
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TimeIntervalExtension {
    pub start_slot: u64,  // Starting slot for the vault's open period
    pub end_slot: u64,    // Ending slot for the vault's open period
}

impl TimeIntervalExtension {
    pub const SIZE: usize = TIME_INTERVAL_EXTENSION_SIZE; // Size of the extension: start_slot (8 bytes) + end_slot (8 bytes)

    pub fn new(start_slot: u64, end_slot: u64) -> Self {
        Self { start_slot, end_slot }
    }

    // Check if the current slot is within the open interval
    pub fn is_open(&self, current_slot: u64) -> bool {
        current_slot >= self.start_slot && current_slot <= self.end_slot
    }
}