use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::VaultError, AccessControlExtension, ExtensionType, LockExtension, TimeIntervalExtension};


// VaultType enum to differentiate between vault types
#[constant]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VaultType {
    SOL,
    SPL, //            (MVP/v1)  It should support SPL & SPL2022 tokens
    // METADATA        (MVP/v2)
    // MPL_CORE,       (MVP/v3)
    // HYBRID/ADVANCED (ROADMAP v5)
}

// Base Vault Account
#[account]
pub struct Vault {
    pub authority: Pubkey,  // Authority of the vault
    pub seed: u64,          // Seed for uniqueness
    pub bump: u8,
    pub vault_type: VaultType,
    pub extensions: Vec<u8>, // Extensions data stored as a byte vector
}

impl Vault {
    pub const BASE_SIZE: usize = 8 + 32 + 8 + 1 + 1 + 4; // Account discriminator (8) + authority (32) + seed (8) + vault type (1) + bump (1) + vec (4)

    // Calculate total size with extensions
    pub fn total_size(extension_size: usize) -> usize {
        Self::BASE_SIZE + extension_size
    }

    // Read a specific extension from the Vault account data
    pub fn read_extension<T: BorshDeserialize>(&self, extension_type: ExtensionType) -> Option<T> {
        let extension_size = extension_type.size();
        let offset = self.get_extension_offset(extension_type)?;

        // Deserialize the extension data from the byte vector
        T::try_from_slice(&self.extensions[offset..offset + extension_size]).ok()
    }

    // Resize the extensions vector to accommodate new data
    pub fn resize_extensions(&mut self, new_size: usize) {
        self.extensions.resize(new_size, 0);  // Resize with zero-filled
    }

    // Write a specific extension to the Vault account data
    pub fn write_extension<T: BorshSerialize>(&mut self, extension_type: ExtensionType, extension: &T) -> Result<()> {
        let extension_size = extension_type.size();
        let offset = match self.get_extension_offset(extension_type) {
            Some(it) => it,
            None => return Err(VaultError::ExtensionOffsetFailed.into())
        };

        // Ensure the extensions vector is resized correctly before writing
        if self.extensions.len() < offset + extension_size {
            self.resize_extensions(offset + extension_size);
        }

        // Serialize the extension data into the byte vector
        let serialized_data = extension.try_to_vec()?;

        self.extensions[offset..offset + extension_size].copy_from_slice(&serialized_data);
        
        Ok(())
    }

    // Hook to check the LockExtension and apply logic if present
    pub fn check_lock_extension(&self, signer: &Signer) -> Result<()> {
        // Attempt to read the LockExtension
        if let Some(lock_extension) = self.read_extension::<LockExtension>(ExtensionType::LockExtension) {
            // Ensure the vault is not locked
            require!(!lock_extension.is_locked, VaultError::VaultLocked);

            // Ensure the signer is the lock authority if attempting to lock or unlock
            require_keys_eq!(
                signer.key(),
                lock_extension.lock_authority,
                VaultError::Unauthorized
            );
        }
        Ok(())
    }

    // Hook to check the TimeIntervalExtension and apply logic if present
    pub fn check_time_interval_extension(&self, current_slot: u64) -> Result<()> {
        // Attempt to read the TimeIntervalExtension
        if let Some(time_interval_extension) = self.read_extension::<TimeIntervalExtension>(ExtensionType::TimeIntervalExtension) {
            // Check if the vault is within the allowed time interval
            require!(
                time_interval_extension.is_open(current_slot),
                VaultError::VaultClosedForDeposits
            );
        }
        Ok(())
    }

    // Check if a depositor is allowed based on the AccessControlExtension
    pub fn check_access_extension(&self, depositor: &Signer) -> Result<()> {
        if let Some(access_extension) = self.read_extension::<AccessControlExtension>(ExtensionType::AccessControlExtension) {
            require!(
                access_extension.is_depositor_allowed(&depositor.key()),
                VaultError::Unauthorized
            );
        }
        Ok(())
    }

    // Calculate the offset for an extension type within the byte vector
    pub fn get_extension_offset(&self, extension_type: ExtensionType) -> Option<usize> {
        let mut offset = 0;
        for ext_type in self.get_extension_types() {
            if ext_type == extension_type {
                return Some(offset);
            }
            offset += ext_type.size();
        }
        None
    }

    // Get all extension types currently active in the account
    pub fn get_extension_types(&self) -> Vec<ExtensionType> {
        // Implement logic to extract currently used extensions from the account data
        vec![ExtensionType::LockExtension, ExtensionType::TimeIntervalExtension] // Placeholder example
    }
}

