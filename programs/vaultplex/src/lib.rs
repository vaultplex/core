pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod extensions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use extensions::*;

declare_id!("9a8aV7w4h7xpoFKnCuXvwef3pXexBbeJBUmMh79sQ5xb");

#[program]
pub mod vaultplex {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, seed: u64, /* , vault_type: VaultType */) -> Result<()> {
        ctx.accounts.initialize_vault(seed, /* vault_type,  */&ctx.bumps)
        
    }

    pub fn initialize_lock_extension(ctx: Context<InitializeLockExtension>, lock_authority: Pubkey) -> Result<()> {
        ctx.accounts.initialize_lock_extension(lock_authority)
    }

    pub fn lock_vault(ctx: Context<LockVault>) -> Result<()> {
        ctx.accounts.lock_vault()
    }

    pub fn unlock_vault(ctx: Context<LockVault>) -> Result<()> {
        ctx.accounts.unlock_vault()
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_sol(amount)
    }

    pub fn initialize_deposit_period_extension(ctx: Context<InitializeDepositPeriodExtension>, start_slot: u64, end_slot: u64) -> Result<()> {
        ctx.accounts.initialize_deposit_period_extension(start_slot, end_slot)
    }

   /*  pub fn initialize_lock_extension(ctx: Context<InitializeLockExtension>, lock_authority: Pubkey) -> Result<()> {
        instructions::initialize_lock_extension::initialize_lock_extension(ctx, lock_authority)
    }

    pub fn initialize_time_interval_extension(ctx: Context<InitializeTimeIntervalExtension>, start_slot: u64, end_slot: u64) -> Result<()> {
        instructions::initialize_time_interval_extension::initialize_time_interval_extension(ctx, start_slot, end_slot)
    }

    pub fn deposit(ctx: Context<Deposit>, seed: u64, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, seed, amount)
    }

    pub fn lock_vault(ctx: Context<LockVault>) -> Result<()> {
        instructions::lock_vault::lock(ctx)
    }

    pub fn unlock_vault(ctx: Context<LockVault>) -> Result<()> {
        instructions::lock_vault::unlock(ctx)
    } */
}