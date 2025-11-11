use anchor_lang::prelude::*;

/// Feedback authorization (prevents spam)
#[account]
pub struct FeedbackAuthorization {
    pub bump: u8,
    pub agent_id_hash: [u8; 32],
    pub client: Pubkey,
    pub max_submissions: u16,
    pub submissions_used: u16,
    pub expires_at: i64,
    pub created_at: i64,
}

impl FeedbackAuthorization {
    /// Account space including 8-byte discriminator:
    /// 8 (discriminator) + 1 (bump) + 32 (agent_id_hash) + 32 (client) +
    /// 2 (max_submissions) + 2 (submissions_used) + 8 (expires_at) + 8 (created_at)
    /// = 93 bytes
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 2 + 2 + 8 + 8;
}

/// Events

#[event]
pub struct FeedbackAuthorized {
    pub agent_id_hash: [u8; 32],
    pub client: Pubkey,
    pub max_submissions: u16,
    pub expires_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct AuthorizationRevoked {
    pub agent_id_hash: [u8; 32],
    pub client: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct FeedbackSubmitted {
    pub agent_id_hash: [u8; 32],
    pub client: Pubkey,
    pub feedback_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct FeedbackUpdated {
    pub feedback_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct FeedbackRevoked {
    pub feedback_hash: [u8; 32],
    pub reason: String,
    pub timestamp: i64,
}
