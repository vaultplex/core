use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::{Vault, VaultDeposit, VaultType};

// Initialize Deposit Account Based on Vault Type
#[derive(Accounts)]
pub struct InitializeDepositSolAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,  // Vault creator or authority

    // We need the main Vault account which contains all extensions
    #[account(
        has_one = authority,                                    // Only the vault creator can initialize the deposit account
        seeds = [b"vault", vault.seed.to_le_bytes().as_ref()],  // We add a constraint for the seeds
        bump = vault.bump                                       // To use less compute units
    )]
    pub vault: Account<'info, Vault>,  // The parent vault

    // SOL Deposit Account - Only initialized if vault_type is SOL
    #[account(
        init_if_needed,
        payer = authority,
        space = VaultDeposit::SIZE,
        seeds = [b"deposit", vault.key().as_ref()],  // PDA seed: "deposit" + vault pubkey
        constraint = vault.vault_type == VaultType::SOL,  // Only for SOL vaults
        bump,
    )]
    pub deposit_account_sol: Account<'info, VaultDeposit>,  // System account PDA for SOL deposits

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeDepositSolAccount<'info> {
    pub fn initialize_deposit_sol_account(&mut self, bumps: &InitializeDepositSolAccountBumps) -> Result<()> {
        // We do the checking (TODO: Try to see if with the constrain is enough)
        require!(self.vault.vault_type == VaultType::SOL, VaultError::WrongType); 

        self.deposit_account_sol.set_inner(VaultDeposit {
            vault: self.vault.key(),
            total_amount: u64::MIN,
            bump: bumps.deposit_account_sol,
        });

        Ok(())
    }
}
