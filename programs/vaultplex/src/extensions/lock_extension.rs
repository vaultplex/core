use anchor_lang::prelude::*;

pub const LOCK_EXTENSION_SIZE: usize = 33; // Pubkey (32) + bool (1)

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LockExtension {
    pub lock_authority: Pubkey, // Authority to lock/unlock the vault
    pub is_locked: bool,        // Status indicating if the vault is locked
}

impl LockExtension {
    pub const SIZE: usize = LOCK_EXTENSION_SIZE; // Authority pubkey (32 bytes) + is_locked bool (1 byte)

    // Initialize a new LockExtension
    pub fn new(lock_authority: Pubkey) -> Self {
        Self {
            lock_authority,
            is_locked: false,
        }
    }
}