/* use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::error::VaultError;
use crate::state::*;  // Import state definitions

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,  // The depositor (it doesn't need to be the creator)

    #[account(
        seeds = [b"vault", vault.seed.to_le_bytes().as_ref()],
        constraint = vault.vault_type == VaultType::SOL,  // Ensure the vault is of type SOL
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,  // The parent vault

    #[account(
        init_if_needed,  // Initialize if the PDA does not exist. The first funder will have to pay for it
        payer = depositor,  // Payer for the account initialization
        seeds = [b"deposit", vault.key().as_ref()],  // PDA seed: "deposit" + vault pubkey
        space = VaultDepositAccount::SIZE,  // Allocate space for the account
        bump,
    )]
    pub deposit_account: Account<'info, VaultDepositAccount>,  // The deposit PDA

    pub system_program: Program<'info, System>,  // System program for SOL deposits
}

impl<'info> DepositSol<'info> {
    pub fn deposit_sol(&mut self, amount: u64, bumps: &DepositSolBumps) -> Result<()> {
        // Step 1: Initialize the deposit account if needed (handled by init_if_needed)

        // Step 2: Check extensions
        vault.check_lock_extension(depositor)?;
        vault.check_access_extension(depositor)?;  // Access check
        vault.check_time_interval_extension(Clock::get()?.slot)?;

        // Step 3: Handle SOL deposit - transfer SOL from depositor to the deposit account PDA
        invoke(
            &system_instruction::transfer(
                &depositor.key(),
                &deposit_account.key(),
                amount,
            ),
            &[
                depositor.to_account_info(),
                deposit_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Step 4: Initialize the deposit account's vault and bump if it's newly created
        if deposit_account.total_amount == 0 {
            deposit_account.vault = vault.key();  // Set the vault reference
            deposit_account.bump = *ctx.bumps.get("deposit_account").unwrap();  // Save the bump
        }

        // Step 5: Use checked_add to safely update the deposit account's total amount
        deposit_account.total_amount = deposit_account
            .total_amount
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;  // Ensure no overflow occurs

        Ok(())
    }
}

    let vault = &ctx.accounts.vault;
    let depositor = &ctx.accounts.depositor;
    let deposit_account = &mut ctx.accounts.deposit_account;

    // Step 1: Initialize the deposit account if needed (handled by init_if_needed)

    // Step 2: Check extensions
    vault.check_lock_extension(depositor)?;
    vault.check_access_extension(depositor)?;  // Access check
    vault.check_time_interval_extension(Clock::get()?.slot)?;

    // Step 3: Handle SOL deposit - transfer SOL from depositor to the deposit account PDA
    invoke(
        &system_instruction::transfer(
            &depositor.key(),
            &deposit_account.key(),
            amount,
        ),
        &[
            depositor.to_account_info(),
            deposit_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Step 4: Initialize the deposit account's vault and bump if it's newly created
    if deposit_account.total_amount == 0 {
        deposit_account.vault = vault.key();  // Set the vault reference
        deposit_account.bump = *ctx.bumps.get("deposit_account").unwrap();  // Save the bump
    }

    // Step 5: Use checked_add to safely update the deposit account's total amount
    deposit_account.total_amount = deposit_account
        .total_amount
        .checked_add(amount)
        .ok_or(VaultError::Overflow)?;  // Ensure no overflow occurs

    Ok(())
}

 #[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", seed.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"svault", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub s_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, _seed: u64, amount: u64) -> Result<()> {
        // Check if the LockExtension is present and handle its logic
        self.vault.check_lock_extension(&self.user)?;

        Ok(())
    }
}


pub fn deposit(ctx: Context<Deposit>, _seed: u64, amount: u64) -> Result<()> {
    let vault = &ctx.accounts.vault;

    // Check if the LockExtension is present and handle its logic
    vault.check_lock_extension(&ctx.accounts.user)?;

    let slot = Clock::get()?.slot;

    // Check if the TimeIntervalExtension is present and handle its logic
    vault.check_time_interval_extension(slot)?;

    // If all checks pass, perform the deposit
    let transfer_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.s_vault.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
    );

    transfer(transfer_ctx, amount)
}
 */ 