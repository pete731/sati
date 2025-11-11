use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum delegation limit reached (8 max)")]
    DelegationLimitReached,

    #[msg("Delegation not found in active delegations")]
    DelegationNotFound,

    #[msg("Delegation is not registered with this agent")]
    DelegationNotRegistered,

    #[msg("Maximum validation limit reached (4 max)")]
    ValidationLimitReached,

    #[msg("Validation not found in active validations")]
    ValidationNotFound,

    #[msg("Invalid agent status transition")]
    InvalidStatus,

    #[msg("Agent is already deregistered")]
    AlreadyDeregistered,

    #[msg("Agent ID already exists")]
    AgentIdExists,

    #[msg("Invalid DID format")]
    InvalidDID,

    #[msg("ZK proof verification failed")]
    InvalidProof,

    #[msg("Compressed metadata is invalid")]
    InvalidMetadata,
}
