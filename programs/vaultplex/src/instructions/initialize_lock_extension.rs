use anchor_lang::prelude::*;
use crate::{state::*, ExtensionType, LockExtension};  // Import state definitions

#[derive(Accounts)]
#[instruction(lock_authority: Pubkey)]
pub struct InitializeLockExtension<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Vault creator or existing authority

    #[account(
        mut,
        has_one = authority,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeLockExtension<'info> {
    pub fn initialize_lock_extension(&mut self, lock_authority: Pubkey) -> Result<()> {
        let lock_extension = LockExtension::new(lock_authority);

        // Create a binding to extend the lifetime of the mutable reference
        let vault_account_info = self.vault.to_account_info();
        let vault_data = &mut vault_account_info.data.borrow_mut();

        // Write the lock extension to the vault's predefined slot for LockExtension
        Vault::write_extension(vault_data, ExtensionType::LockExtension, &lock_extension)
    }
}
