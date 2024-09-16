use anchor_lang::prelude::*;

// Access Control Type Enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AccessControlType {
    Public,   // Anyone can add funds
    Private,  // Only the creator (authority) can add funds
    // Protected,  // Placeholder for future implementation
}

// Access Control Extension Definition
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AccessControlExtension {
    pub access_control_type: AccessControlType,  // Type of access control
    pub access_control_authority: Pubkey,        // Authority to manage the access control extension
}

impl AccessControlExtension {
    pub const SIZE: usize = 32 + 1; // Size: AccessControlType (1) + Authority (32)

    // Initialize a new AccessControlExtension
    pub fn new(access_control_authority: Pubkey, access_control_type: AccessControlType) -> Self {
        Self {
            access_control_type,
            access_control_authority,
        }
    }

    // Check if a depositor is allowed based on the access type
    pub fn is_depositor_allowed(&self, depositor: &Pubkey) -> bool {
        match self.access_control_type {
            AccessControlType::Public => true,  // Anyone can deposit
            AccessControlType::Private => *depositor == self.access_control_authority,  // Only the authority can deposit
            // AccessType::Protected => false, // Whitelist -> Future implementation
        }
    }
}