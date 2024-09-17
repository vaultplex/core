use anchor_lang::prelude::*;

use crate::{error::VaultError, ExtensionType, LOCK_EXTENSION_SIZE, TIME_INTERVAL_EXTENSION_SIZE};


// Base Vault Account
#[account]
pub struct Vault {
    pub authority: Pubkey,  // Authority of the vault
    pub seed: u64,          // Seed for uniqueness
    pub bump: u8,
    // pub deposit_bump: u8,
    pub extensions: Vec<u8>, // Extensions data stored as a byte vector
}

impl Vault {
    // Account discriminator (8) + authority (32) + seed (8) + vault type (1) + bump (1) + vec (4) = 54
    pub const BASE_SIZE: usize = 53; 

    // Calculate total size with extensions
    pub fn total_size() -> usize {
        Self::BASE_SIZE
            + LOCK_EXTENSION_SIZE
            + TIME_INTERVAL_EXTENSION_SIZE
    }

    // Read a specific extension from the Vault account data
    pub fn read_extension<T: AnchorDeserialize>(&self, data: &[u8], ext_type: ExtensionType) -> Result<T> {
        let offset = ext_type.offset();
        let size = ext_type.size();

        if data.len() < offset + size {
            return Err(VaultError::ExtensionOffsetFailed.into());
        }

        T::try_from_slice(&data[offset..offset + size]).map_err(|_| VaultError::ExtensionOffsetFailed.into())
    }

    // Write a specific extension to the Vault account data
    // Write a specific extension to the Vault account data
    pub fn write_extension<T: AnchorSerialize>(data: &mut [u8], ext_type: ExtensionType, extension: &T) -> Result<()> {
        let offset = ext_type.offset();
        let size = ext_type.size();
        
        if data.len() < offset + size {
            return Err(VaultError::ExtensionOffsetFailed.into());
        }

        let serialized_data = extension.try_to_vec().map_err(|_| VaultError::ExtensionOffsetFailed)?;
        
        data[offset..offset + size].copy_from_slice(&serialized_data);
        
        Ok(())
    }
}

