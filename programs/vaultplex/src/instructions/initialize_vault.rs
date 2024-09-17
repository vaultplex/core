use anchor_lang::prelude::*;
use crate::state::*;  // Import state definitions

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Vault::total_size(),
        seeds = [b"vault", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, seed: u64, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            authority: self.authority.key(),
            seed,
            bump: bumps.vault,
            extensions: vec![],
        });

        /* self.deposit_account.set_inner(Deposit) */

        Ok(())
    }
}
