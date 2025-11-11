use crate::errors::ErrorCode;
use crate::state::FeedbackAuthorization;
use anchor_lang::prelude::*;

/// Context for authorizing a client to submit feedback
#[derive(Accounts)]
#[instruction(agent_id_hash: [u8; 32], client: Pubkey)]
pub struct AuthorizeFeedback<'info> {
    #[account(
        init,
        payer = agent_owner,
        space = FeedbackAuthorization::SPACE,
        seeds = [b"feedback_auth", agent_id_hash.as_ref(), client.as_ref()],
        bump
    )]
    pub authorization: Account<'info, FeedbackAuthorization>,

    #[account(mut)]
    pub agent_owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Context for submitting feedback
#[derive(Accounts)]
pub struct SubmitFeedback<'info> {
    #[account(mut)]
    pub authorization: Account<'info, FeedbackAuthorization>,

    pub client: Signer<'info>,
}

/// Context for revoking feedback authorization
#[derive(Accounts)]
pub struct RevokeAuthorization<'info> {
    #[account(
        mut,
        close = agent_owner
    )]
    pub authorization: Account<'info, FeedbackAuthorization>,

    #[account(mut)]
    pub agent_owner: Signer<'info>,
}

/// Context for updating feedback
#[derive(Accounts)]
pub struct UpdateFeedback<'info> {
    pub client: Signer<'info>,
}

/// Context for revoking feedback
#[derive(Accounts)]
pub struct RevokeFeedback<'info> {
    pub authority: Signer<'info>,
}
