use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Mandate has expired")]
    MandateExpired,

    #[msg("Mandate has already been revoked")]
    MandateRevoked,

    #[msg("Mandate has already been executed")]
    MandateExecuted,

    #[msg("Mandate is in zombie state, requires revalidation")]
    MandateZombie,

    #[msg("Mandate has been amended, use new mandate")]
    MandateAmended,

    #[msg("Context has drifted, revalidation required")]
    ContextDrift,

    #[msg("Invalid mandate type")]
    InvalidMandateType,

    #[msg("Mandate is not expired yet")]
    NotExpired,

    #[msg("Maximum execution count reached")]
    MaxExecutionsReached,

    #[msg("Unauthorized to execute mandate")]
    UnauthorizedExecution,

    #[msg("Unauthorized to revalidate mandate")]
    UnauthorizedRevalidation,

    #[msg("ZK proof verification failed")]
    InvalidProof,

    #[msg("Compressed mandate data is invalid")]
    InvalidMandateData,
}
