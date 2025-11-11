use anchor_lang::prelude::*;

declare_id!("SAT3111111111111111111111111111111111111111");

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

#[program]
pub mod mandate_registry {
    use super::*;

    /// Create a new payment mandate
    pub fn create_mandate(
        ctx: Context<CreateMandate>,
        mandate_id: String,
        agent_id_hash: [u8; 32],
        mandate_type: u8,
        context_hash: [u8; 32],
        expires_in_seconds: i64,
        revalidation_threshold_seconds: i64,
        max_executions: u64,
        // TODO: Add CompressedMandate and ZkProof params when Light SDK integrated
    ) -> Result<()> {
        let mandate = &mut ctx.accounts.mandate;
        let current_time = Clock::get()?.unix_timestamp;

        mandate.bump = ctx.bumps.mandate;
        mandate.agent_id_hash = agent_id_hash;
        mandate.user_authority = ctx.accounts.user.key();
        mandate.mandate_type = mandate_type;
        mandate.status = MandateStatus::Active;
        mandate.created_at = current_time;
        mandate.expires_at = if expires_in_seconds > 0 {
            current_time + expires_in_seconds
        } else {
            0
        };
        mandate.context_hash = context_hash;
        mandate.revalidation_threshold = if revalidation_threshold_seconds > 0 {
            current_time + revalidation_threshold_seconds
        } else {
            0
        };
        mandate.execution_count = 0;
        mandate.max_executions = max_executions;
        mandate.mandate_state_root = solana_program::hash::hash(mandate_id.as_bytes()).to_bytes();
        mandate.updated_at = current_time;

        msg!("Mandate created: {}", mandate_id);

        // TODO: Emit MandateCreated event

        Ok(())
    }

    /// Revoke a mandate
    pub fn revoke_mandate(ctx: Context<RevokeMandate>, reason: String) -> Result<()> {
        let mandate = &mut ctx.accounts.mandate;

        // Check not already revoked
        require!(
            mandate.status != MandateStatus::Revoked,
            crate::errors::ErrorCode::MandateRevoked
        );

        mandate.status = MandateStatus::Revoked;
        mandate.updated_at = Clock::get()?.unix_timestamp;

        msg!("Mandate revoked: {}", reason);

        // TODO: Emit MandateRevoked event

        Ok(())
    }

    /// Amend a mandate
    pub fn amend_mandate(
        ctx: Context<AmendMandate>,
        new_mandate_state_root: [u8; 32],
        // TODO: Add CompressedMandate and ZkProof params when Light SDK integrated
    ) -> Result<()> {
        let old_mandate = &mut ctx.accounts.old_mandate;

        // Check mandate is active
        require!(
            old_mandate.status == MandateStatus::Active,
            crate::errors::ErrorCode::MandateRevoked
        );

        let old_state_root = old_mandate.mandate_state_root;

        // Mark old mandate as amended
        old_mandate.status = MandateStatus::Amended;
        old_mandate.updated_at = Clock::get()?.unix_timestamp;

        msg!(
            "Mandate amended: old={:?}, new={:?}",
            old_state_root,
            new_mandate_state_root
        );

        // TODO: Emit MandateAmended event
        // TODO: Create new mandate account (needs separate instruction)

        Ok(())
    }

    /// Execute a mandate
    pub fn execute_mandate(
        ctx: Context<ExecuteMandate>,
        // TODO: Add payment signature param
    ) -> Result<()> {
        let mandate = &mut ctx.accounts.mandate;
        let current_time = Clock::get()?.unix_timestamp;

        // Check mandate is active
        require!(
            mandate.status == MandateStatus::Active,
            crate::errors::ErrorCode::MandateRevoked
        );

        // Check not expired
        if mandate.expires_at > 0 {
            require!(
                current_time < mandate.expires_at,
                crate::errors::ErrorCode::MandateExpired
            );
        }

        // Check execution limit
        if mandate.max_executions > 0 {
            require!(
                mandate.execution_count < mandate.max_executions,
                crate::errors::ErrorCode::MaxExecutionsReached
            );
        }

        // Increment execution count
        mandate.execution_count += 1;

        // If max executions reached, mark as executed
        if mandate.max_executions > 0 && mandate.execution_count >= mandate.max_executions {
            mandate.status = MandateStatus::Executed;
        }

        mandate.updated_at = current_time;

        msg!("Mandate executed: count={}", mandate.execution_count);

        // TODO: Emit MandateExecuted event
        // TODO: Verify payment signature

        Ok(())
    }

    /// Check for context drift in a mandate
    pub fn check_context_drift(
        ctx: Context<CheckContextDrift>,
        current_context_hash: [u8; 32],
    ) -> Result<bool> {
        let mandate = &mut ctx.accounts.mandate;
        let current_time = Clock::get()?.unix_timestamp;

        // Compare context hashes
        let has_drifted = mandate.context_hash != current_context_hash;

        // Check time-based revalidation
        let needs_revalidation =
            mandate.revalidation_threshold > 0 && current_time >= mandate.revalidation_threshold;

        if has_drifted || needs_revalidation {
            // Mark as Zombie
            mandate.status = MandateStatus::Zombie;
            mandate.updated_at = current_time;

            msg!(
                "Context drift detected: drifted={}, time_revalidation={}",
                has_drifted,
                needs_revalidation
            );

            // TODO: Emit MandateContextDrift event

            return Ok(true);
        }

        Ok(false)
    }

    /// Revalidate a zombie mandate
    pub fn revalidate_mandate(
        ctx: Context<RevalidateMandate>,
        new_context_hash: [u8; 32],
    ) -> Result<()> {
        let mandate = &mut ctx.accounts.mandate;
        let current_time = Clock::get()?.unix_timestamp;

        // Check mandate is in Zombie state
        require!(
            mandate.status == MandateStatus::Zombie,
            crate::errors::ErrorCode::MandateZombie
        );

        // Update context and reactivate
        mandate.context_hash = new_context_hash;
        mandate.status = MandateStatus::Active;
        mandate.revalidation_threshold = if mandate.revalidation_threshold > 0 {
            // Reset revalidation threshold to same duration
            current_time + (mandate.revalidation_threshold - mandate.created_at)
        } else {
            0
        };
        mandate.updated_at = current_time;

        msg!("Mandate revalidated with new context");

        // TODO: Emit MandateRevalidated event

        Ok(())
    }

    /// Enforce mandate expiry
    pub fn enforce_expiry(ctx: Context<EnforceExpiry>) -> Result<()> {
        let mandate = &mut ctx.accounts.mandate;
        let current_time = Clock::get()?.unix_timestamp;

        // Check mandate has expiry set
        require!(mandate.expires_at > 0, crate::errors::ErrorCode::NotExpired);

        // Check it's actually expired
        require!(
            current_time >= mandate.expires_at,
            crate::errors::ErrorCode::NotExpired
        );

        // Mark as expired
        mandate.status = MandateStatus::Expired;
        mandate.updated_at = current_time;

        msg!("Mandate expired");

        Ok(())
    }
}
