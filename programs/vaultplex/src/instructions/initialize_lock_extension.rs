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
        realloc = Vault::total_size(LockExtension::SIZE), // Reallocate space for the lock extension
        realloc::zero = true, 
        realloc::payer = authority,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeLockExtension<'info> {
    pub fn initialize_lock_extension(&mut self, lock_authority: Pubkey) -> Result<()> {
        let lock_extension = LockExtension::new(lock_authority);

        self.vault.write_extension(ExtensionType::LockExtension, &lock_extension)    
    }
}
