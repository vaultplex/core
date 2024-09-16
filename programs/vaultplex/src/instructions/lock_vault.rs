use anchor_lang::prelude::*;
use crate::{ 
    Vault,
    error::VaultError, 
    extensions::*, 
};

#[derive(Accounts)]
pub struct LockVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Authority allowed to modify the vault

    #[account(mut)]
    pub lock_authority: Signer<'info>, // Authority allowed to lock/unlock

    #[account(
        mut,
        seeds = [b"vault", vault.seed.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
}

impl<'info> LockVault<'info> {
    pub fn lock_vault(&mut self) -> Result<()> {
        let mut lock_extension: LockExtension = self.vault.read_extension(ExtensionType::LockExtension).ok_or(VaultError::ExtensionNotFound)?;

        // Check if the lock_authority is authorized
        require_keys_eq!(
            self.lock_authority.key(),
            lock_extension.lock_authority,
            VaultError::Unauthorized
        );

        // Lock the vault
        lock_extension.is_locked = true;
        
        self.vault.write_extension(ExtensionType::LockExtension, &lock_extension)?;

        Ok(())
    }
}

/* pub fn lock(ctx: Context<LockVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let mut lock_extension: LockExtension = vault.read_extension(ExtensionType::LockExtension).ok_or(VaultError::Unauthorized)?;

    // Check if the lock_authority is authorized
    require_keys_eq!(
        ctx.accounts.lock_authority.key(),
        lock_extension.lock_authority,
        VaultError::Unauthorized
    );

    // Lock the vault
    lock_extension.is_locked = true;
    vault.write_extension(ExtensionType::LockExtension, &lock_extension)?;

    Ok(())
}

pub fn unlock(ctx: Context<LockVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let mut lock_extension: LockExtension = vault.read_extension(ExtensionType::LockExtension).ok_or(VaultError::Unauthorized)?;

    // Check if the lock_authority is authorized
    require_keys_eq!(
        ctx.accounts.lock_authority.key(),
        lock_extension.lock_authority,
        VaultError::Unauthorized
    );

    // Unlock the vault
    lock_extension.is_locked = false;
    vault.write_extension(ExtensionType::LockExtension, &lock_extension)?;

    Ok(())
}
 */