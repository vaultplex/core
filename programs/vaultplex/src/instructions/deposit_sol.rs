use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::{error::VaultError, state::*, LockExtension};

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault_config", vault_config.seed.to_le_bytes().as_ref()],
        bump = vault_config.bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"vault", vault_config.seed.to_le_bytes().as_ref()],
        bump = vault_config.vault_bump,
    )]
    pub vault: SystemAccount<'info>, // PDA where SOL is stored

    pub system_program: Program<'info, System>,
}

impl<'info> DepositSol<'info> {
    pub fn deposit_sol(&mut self, amount: u64) -> Result<()> {
        let vault_config = &mut self.vault_config;

        // Check if the vault is locked
        let lock_extension: LockExtension = vault_config.read_extension(LOCK_EXTENSION_OFFSET)?;
        if lock_extension.is_locked {
            return Err(VaultError::VaultLocked.into());
        }

        // Transfer SOL from the user to the vault's PDA
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.user.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        transfer(cpi_context, amount)?;

        Ok(())
    }
}
