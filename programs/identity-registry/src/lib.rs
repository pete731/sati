use anchor_lang::prelude::*;

declare_id!("SAT1111111111111111111111111111111111111111");

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

#[program]
pub mod identity_registry {
    use super::*;

    /// Register a new agent identity
    pub fn register_agent(
        ctx: Context<RegisterAgent>,
        agent_id: String,
        // TODO: Add CompressedMetadata and ZkProof params when Light SDK integrated
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Initialize account fields
        agent.bump = ctx.bumps.agent;
        agent.owner = ctx.accounts.owner.key();
        agent.agent_id_hash = solana_program::hash::hash(agent_id.as_bytes()).to_bytes();
        agent.state_root = [0u8; 32]; // TODO: Set from compressed metadata
        agent.ledger_ref = Clock::get()?.slot;
        agent.status = 1; // Active
        agent.registered_at = Clock::get()?.unix_timestamp;
        agent.updated_at = Clock::get()?.unix_timestamp;
        agent.did_hash = None;
        agent.delegation_count = 0;
        agent.active_delegations = [Pubkey::default(); 8];
        agent.validation_count = 0;
        agent.validation_attestations = [Pubkey::default(); 4];

        msg!("Agent registered: {}", agent_id);

        // TODO: Emit AgentRegistered event

        Ok(())
    }

    /// Update agent metadata
    pub fn update_agent(
        ctx: Context<UpdateAgent>,
        // TODO: Add CompressedMetadata and ZkProof params when Light SDK integrated
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // TODO: Verify ZK proof and update state_root
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Agent updated");

        // TODO: Emit AgentUpdated event

        Ok(())
    }

    /// Add a delegation attestation to the agent
    pub fn add_delegation(ctx: Context<AddDelegation>, sas_attestation: Pubkey) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Add delegation using impl method
        agent.add_delegation(sas_attestation)?;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Delegation added: {}", sas_attestation);

        // TODO: Emit DelegationAdded event

        Ok(())
    }

    /// Remove a delegation attestation from the agent
    pub fn remove_delegation(
        ctx: Context<RemoveDelegation>,
        sas_attestation: Pubkey,
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Remove delegation using impl method
        agent.remove_delegation(sas_attestation)?;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Delegation removed: {}", sas_attestation);

        // TODO: Emit DelegationRemoved event

        Ok(())
    }

    /// Add a validation attestation to the agent
    pub fn add_validation(ctx: Context<AddValidation>, sas_attestation: Pubkey) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Add validation using impl method
        agent.add_validation(sas_attestation)?;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Validation added: {}", sas_attestation);

        // TODO: Emit ValidationAdded event

        Ok(())
    }

    /// Remove a validation attestation from the agent
    pub fn remove_validation(
        ctx: Context<RemoveValidation>,
        sas_attestation: Pubkey,
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Remove validation using impl method
        agent.remove_validation(sas_attestation)?;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Validation removed: {}", sas_attestation);

        // TODO: Emit ValidationRemoved event

        Ok(())
    }

    /// Update agent's DID
    pub fn update_did(ctx: Context<UpdateDID>, did: String) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Store DID as hash
        let did_hash = solana_program::hash::hash(did.as_bytes()).to_bytes();
        agent.did_hash = Some(did_hash);
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("DID updated: {}", did);

        // TODO: Emit DIDUpdated event

        Ok(())
    }

    /// Update agent status
    pub fn update_status(ctx: Context<UpdateStatus>, new_status: u8) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // TODO: Add status transition validation
        agent.status = new_status;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Status updated to: {}", new_status);

        Ok(())
    }

    /// Deregister an agent
    pub fn deregister_agent(ctx: Context<DeregisterAgent>) -> Result<()> {
        let agent = &mut ctx.accounts.agent;

        // Check if already deregistered
        require!(
            agent.status != 3,
            crate::errors::ErrorCode::AlreadyDeregistered
        );

        // Mark as deregistered (don't close account)
        agent.status = 3;
        agent.updated_at = Clock::get()?.unix_timestamp;

        msg!("Agent deregistered");

        Ok(())
    }
}
