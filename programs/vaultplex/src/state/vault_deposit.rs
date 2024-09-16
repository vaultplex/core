use anchor_lang::prelude::*;

// VaultDepositAccount Definition
#[account]
pub struct VaultDeposit {
    pub vault: Pubkey,     // Reference to the parent Vault
    pub total_amount: u64, // Total amount of all deposits
    pub bump: u8,          // PDA bump
}

impl VaultDeposit {
    // Size of the VaultDepositAccount
    pub const SIZE: usize = 32 + 8 + 1; // Vault Pubkey (32) + Total Amount (8) + Bump (1)
}
