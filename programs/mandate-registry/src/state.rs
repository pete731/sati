use anchor_lang::prelude::*;

/// Payment mandate with lifecycle management
#[account]
pub struct MandateAccount {
    /// PDA bump seed
    pub bump: u8,

    /// Agent identity hash (references Identity Registry)
    pub agent_id_hash: [u8; 32],

    /// User authority who created mandate
    pub user_authority: Pubkey,

    /// Mandate type (0=Intent, 1=Cart)
    pub mandate_type: u8,

    /// Current lifecycle state
    pub status: MandateStatus,

    /// When mandate was created
    pub created_at: i64,

    /// When mandate expires (0 = no expiry)
    pub expires_at: i64,

    /// Hash of user's context at creation (location, budget, preferences)
    /// Used for drift detection
    pub context_hash: [u8; 32],

    /// When context should be revalidated (0 = no revalidation needed)
    pub revalidation_threshold: i64,

    /// Number of times mandate has been executed (partial execution tracking)
    pub execution_count: u64,

    /// Maximum allowed executions (0 = unlimited)
    pub max_executions: u64,

    /// ZK compression state root pointing to full mandate details
    pub mandate_state_root: [u8; 32],

    /// Last updated timestamp
    pub updated_at: i64,
}

impl MandateAccount {
    /// Account space including 8-byte discriminator:
    /// 8 (discriminator) + 1 (bump) + 32 (agent_id_hash) + 32 (user_authority) +
    /// 1 (mandate_type) + 1 (status) + 8 (created_at) + 8 (expires_at) +
    /// 32 (context_hash) + 8 (revalidation_threshold) + 8 (execution_count) +
    /// 8 (max_executions) + 32 (mandate_state_root) + 8 (updated_at)
    /// = 187 bytes
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 1 + 1 + 8 + 8 + 32 + 8 + 8 + 8 + 32 + 8;
}

/// Mandate lifecycle states
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MandateStatus {
    /// Mandate created and ready for execution
    Active = 0,
    /// Mandate has expired based on time
    Expired = 1,
    /// Manually revoked by user or agent
    Revoked = 2,
    /// Successfully executed (for single-use mandates)
    Executed = 3,
    /// Context drift detected, requires user revalidation
    Zombie = 4,
    /// Amended (new mandate created, old one deprecated)
    Amended = 5,
}

/// Events

#[event]
pub struct MandateCreated {
    pub mandate_state_root: [u8; 32],
    pub agent_id_hash: [u8; 32],
    pub user_authority: Pubkey,
    pub mandate_type: u8,
    pub expires_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct MandateRevoked {
    pub mandate_state_root: [u8; 32],
    pub reason: String,
    pub revoked_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MandateExecuted {
    pub mandate_state_root: [u8; 32],
    pub execution_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct MandateContextDrift {
    pub mandate_state_root: [u8; 32],
    pub old_context: [u8; 32],
    pub new_context: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct MandateRevalidated {
    pub mandate_state_root: [u8; 32],
    pub new_context: [u8; 32],
    pub revalidated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MandateAmended {
    pub old_mandate: [u8; 32],
    pub new_mandate: [u8; 32],
    pub timestamp: i64,
}
