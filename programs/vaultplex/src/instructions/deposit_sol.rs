use crate::DepositPeriodExtension;
use crate::{error::VaultError, state::*, LockExtension};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

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
        if let Ok(lock_extension) =
            vault_config.read_extension::<LockExtension>(LOCK_EXTENSION_OFFSET)
        {
            // If the extension exists, check if the vault is locked
            lock_extension.check_lock()?; // Vault is locked if `is_locked` is true
        }

        // Get the current slot
        let current_slot = Clock::get()?.slot;

        // Check if the deposit period is open
        if let Ok(deposit_period_extension) =
            vault_config.read_extension::<DepositPeriodExtension>(DEPOSIT_PERIOD_EXTENSION_OFFSET)
        {
            // If the extension exists, check if the deposit period is open
            deposit_period_extension.is_deposit_allowed(current_slot)?;
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
