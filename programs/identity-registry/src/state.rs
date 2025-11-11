use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

/// On-chain agent identity (ZK compressed)
#[account]
pub struct AgentIdentity {
    /// PDA bump seed
    pub bump: u8,

    /// Agent owner authority
    pub owner: Pubkey,

    /// Unique agent identifier hash
    pub agent_id_hash: [u8; 32],

    /// ZK compression state root
    /// Points to full metadata in ledger storage
    pub state_root: [u8; 32],

    /// Ledger transaction reference
    pub ledger_ref: u64,

    /// Agent status (0=Pending, 1=Active, 2=Inactive, 3=Deregistered)
    pub status: u8,

    /// Registration timestamp
    pub registered_at: i64,

    /// Last update timestamp
    pub updated_at: i64,

    /// Decentralized Identifier (DID) for cross-chain interoperability
    /// Supports: did:solana:<pubkey>, did:pkh:eip155:1:<eth_address>, did:web:<domain>
    /// Stored as fixed-size hash, full DID in metadata
    pub did_hash: Option<[u8; 32]>,

    /// Number of active delegations
    pub delegation_count: u8,

    /// Active user delegation attestations (SAS pubkeys)
    /// Fixed-size array (MAX_DELEGATIONS = 8)
    pub active_delegations: [Pubkey; 8],

    /// Number of active validations
    pub validation_count: u8,

    /// Active validation attestations (SAS pubkeys)
    /// Fixed-size array (MAX_VALIDATIONS = 4)
    pub validation_attestations: [Pubkey; 4],
}

impl AgentIdentity {
    pub const MAX_DELEGATIONS: usize = 8;
    pub const MAX_VALIDATIONS: usize = 4;

    /// Account space including 8-byte discriminator: 549 bytes
    /// (vs aeamcp's 2,500 bytes = 4.5x smaller)
    pub const SPACE: usize =
        8 + 1 + 32 + 32 + 32 + 8 + 1 + 8 + 8 + 33 + 1 + (32 * 8) + 1 + (32 * 4);

    /// Check if delegation can be added
    pub fn can_add_delegation(&self) -> bool {
        (self.delegation_count as usize) < Self::MAX_DELEGATIONS
    }

    /// Check if validation can be added
    pub fn can_add_validation(&self) -> bool {
        (self.validation_count as usize) < Self::MAX_VALIDATIONS
    }

    /// Add delegation to array
    pub fn add_delegation(&mut self, attestation: Pubkey) -> Result<()> {
        require!(self.can_add_delegation(), ErrorCode::DelegationLimitReached);

        self.active_delegations[self.delegation_count as usize] = attestation;
        self.delegation_count += 1;
        Ok(())
    }

    /// Remove delegation from array
    pub fn remove_delegation(&mut self, attestation: Pubkey) -> Result<()> {
        let index = self.active_delegations[..self.delegation_count as usize]
            .iter()
            .position(|a| a == &attestation)
            .ok_or(ErrorCode::DelegationNotFound)?;

        // Shift remaining elements
        for i in index..(self.delegation_count as usize - 1) {
            self.active_delegations[i] = self.active_delegations[i + 1];
        }

        self.delegation_count -= 1;
        self.active_delegations[self.delegation_count as usize] = Pubkey::default();
        Ok(())
    }

    /// Add validation attestation
    pub fn add_validation(&mut self, attestation: Pubkey) -> Result<()> {
        require!(self.can_add_validation(), ErrorCode::ValidationLimitReached);

        self.validation_attestations[self.validation_count as usize] = attestation;
        self.validation_count += 1;
        Ok(())
    }

    /// Remove validation attestation
    pub fn remove_validation(&mut self, attestation: Pubkey) -> Result<()> {
        let index = self.validation_attestations[..self.validation_count as usize]
            .iter()
            .position(|a| a == &attestation)
            .ok_or(ErrorCode::ValidationNotFound)?;

        // Shift remaining elements
        for i in index..(self.validation_count as usize - 1) {
            self.validation_attestations[i] = self.validation_attestations[i + 1];
        }

        self.validation_count -= 1;
        self.validation_attestations[self.validation_count as usize] = Pubkey::default();
        Ok(())
    }
}

/// Events (from spec lines 377-427)

#[event]
pub struct AgentRegistered {
    pub agent_id_hash: [u8; 32],
    pub owner: Pubkey,
    pub state_root: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct AgentUpdated {
    pub agent_id_hash: [u8; 32],
    pub new_state_root: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct DelegationAdded {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DelegationRemoved {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub reason: String,
    pub timestamp: i64,
}

#[event]
pub struct ValidationAdded {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ValidationRemoved {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DIDUpdated {
    pub agent_id_hash: [u8; 32],
    pub did_hash: [u8; 32],
    pub timestamp: i64,
}
