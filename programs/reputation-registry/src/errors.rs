use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Feedback authorization not found")]
    AuthorizationNotFound,

    #[msg("Feedback authorization has expired")]
    AuthorizationExpired,

    #[msg("Maximum submissions reached for this authorization")]
    MaxSubmissionsReached,

    #[msg("Client is not authorized to submit feedback")]
    UnauthorizedClient,

    #[msg("Feedback not found")]
    FeedbackNotFound,

    #[msg("Cannot update feedback after time window")]
    UpdateWindowExpired,

    #[msg("Invalid rating score")]
    InvalidRating,

    #[msg("Payment proof verification failed")]
    InvalidPaymentProof,

    #[msg("Feedback already revoked")]
    FeedbackRevoked,
}
