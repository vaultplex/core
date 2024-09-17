/* use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Mint};

use crate::error::VaultError;
use crate::{Vault, VaultDeposit, VaultType};

// Initialize Deposit Account Based on Vault Type
#[derive(Accounts)]
pub struct InitializeDepositSplAccount<'info> {
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
     pub deposit_account_spl: Account<'info, VaultDeposit>,  // System account PDA for SOL deposits
 

} */