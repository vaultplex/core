use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Default, Clone, Debug)]
pub struct LockExtension {
    pub lock_authority: Pubkey, // Authority to lock/unlock the vault
    pub is_locked: bool,        // Status indicating if the vault is locked
}

impl LockExtension {
    pub fn new(lock_authority: Pubkey) -> Self {
        Self {
            lock_authority,
            is_locked: false,
        }
    }
}
