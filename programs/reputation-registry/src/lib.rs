use anchor_lang::prelude::*;

declare_id!("SAT2111111111111111111111111111111111111111");

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

#[program]
pub mod reputation_registry {
    use super::*;

    /// Authorize a client to submit feedback for an agent
    pub fn authorize_feedback(
        ctx: Context<AuthorizeFeedback>,
        agent_id_hash: [u8; 32],
        client: Pubkey,
        max_submissions: u16,
        expires_in_seconds: i64,
    ) -> Result<()> {
        let auth = &mut ctx.accounts.authorization;

        auth.bump = ctx.bumps.authorization;
        auth.agent_id_hash = agent_id_hash;
        auth.client = client;
        auth.max_submissions = max_submissions;
        auth.submissions_used = 0;
        auth.created_at = Clock::get()?.unix_timestamp;
        auth.expires_at = auth.created_at + expires_in_seconds;

        msg!("Feedback authorization created for client: {}", client);

        // TODO: Emit FeedbackAuthorized event

        Ok(())
    }

    /// Revoke feedback authorization for a client
    pub fn revoke_authorization(ctx: Context<RevokeAuthorization>) -> Result<()> {
        let auth = &ctx.accounts.authorization;

        msg!("Authorization revoked for client: {}", auth.client);

        // TODO: Emit AuthorizationRevoked event

        // Account is closed via close constraint

        Ok(())
    }

    /// Submit feedback for an agent (event-based, not stored on-chain)
    pub fn submit_feedback(
        ctx: Context<SubmitFeedback>,
        feedback_hash: [u8; 32],
        // TODO: Add payment proof and rating params
    ) -> Result<()> {
        let auth = &mut ctx.accounts.authorization;

        // Verify client is authorized
        require!(
            auth.client == ctx.accounts.client.key(),
            crate::errors::ErrorCode::UnauthorizedClient
        );

        // Check expiry
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            auth.expires_at == 0 || current_time < auth.expires_at,
            crate::errors::ErrorCode::AuthorizationExpired
        );

        // Check submission limit
        require!(
            auth.submissions_used < auth.max_submissions,
            crate::errors::ErrorCode::MaxSubmissionsReached
        );

        // Increment counter
        auth.submissions_used += 1;

        msg!("Feedback submitted: {:?}", feedback_hash);

        // TODO: Emit FeedbackSubmitted event
        // TODO: Verify payment proof if provided

        Ok(())
    }

    /// Update previously submitted feedback
    pub fn update_feedback(
        _ctx: Context<UpdateFeedback>,
        feedback_hash: [u8; 32],
        // TODO: Add new feedback params
    ) -> Result<()> {
        // TODO: Implement feedback update logic
        // This would check the ledger for the original feedback
        // and emit an updated event if within time window

        msg!("Feedback update requested: {:?}", feedback_hash);

        // TODO: Emit FeedbackUpdated event

        Ok(())
    }

    /// Revoke previously submitted feedback
    pub fn revoke_feedback(
        _ctx: Context<RevokeFeedback>,
        feedback_hash: [u8; 32],
        reason: String,
    ) -> Result<()> {
        // TODO: Implement revocation logic
        // This would emit a revocation event that indexers can process

        msg!("Feedback revoked: {:?}, reason: {}", feedback_hash, reason);

        // TODO: Emit FeedbackRevoked event

        Ok(())
    }
}
