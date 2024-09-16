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
        space = Vault::total_size(0), // No extensions yet
        seeds = [b"vault", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    /* #[account(
        init,  // Always initialize the deposit account when the vault is created
        payer = authority,  // The payer is the authority of the vault
        space = VaultDepositAccount::SIZE,  // Allocate space for the deposit account
        seeds = [b"deposit", vault.key().as_ref()],  // PDA seed: "deposit" + vault pubkey
        bump
    )]
    pub deposit_account: Account<'info, VaultDepositAccount>,  // The deposit PDA
 */
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, seed: u64, vault_type: VaultType, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            authority: self.authority.key(),
            seed,
            bump: bumps.vault,
            vault_type,
            extensions: vec![],
        });

        /* self.deposit_account.set_inner(Deposit) */

        Ok(())
    }
}
