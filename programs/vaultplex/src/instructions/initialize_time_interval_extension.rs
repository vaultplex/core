use anchor_lang::prelude::*;
use crate::{state::*, ExtensionType, TimeIntervalExtension};  // Import state definitions

#[derive(Accounts)]
#[instruction(start_slot: u64, end_slot: u64)]
pub struct InitializeTimeIntervalExtension<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Vault creator or existing authority

    #[account(
        mut,
        has_one = authority,
        realloc = Vault::total_size(TimeIntervalExtension::SIZE), // Reallocate space for the time interval extension
        realloc::zero = true,
        realloc::payer = authority,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_time_interval_extension(ctx: Context<InitializeTimeIntervalExtension>, start_slot: u64, end_slot: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let time_interval_extension = TimeIntervalExtension::new(start_slot, end_slot);
    vault.write_extension(ExtensionType::TimeIntervalExtension, &time_interval_extension)?;
    Ok(())
}
