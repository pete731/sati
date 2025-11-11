use crate::errors::ErrorCode;
use crate::state::AgentIdentity;
use anchor_lang::prelude::*;

/// Context for registering a new agent identity
#[derive(Accounts)]
#[instruction(agent_id: String)]
pub struct RegisterAgent<'info> {
    /// Agent identity PDA derived from agent_id string
    /// NOTE: Using plain string in seeds, storing hash in account field
    #[account(
        init,
        payer = owner,
        space = AgentIdentity::SPACE,
        seeds = [b"agent", agent_id.as_bytes()],
        bump
    )]
    pub agent: Account<'info, AgentIdentity>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Context for updating agent metadata
#[derive(Accounts)]
pub struct UpdateAgent<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for adding a delegation attestation
#[derive(Accounts)]
pub struct AddDelegation<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for removing a delegation attestation
#[derive(Accounts)]
pub struct RemoveDelegation<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for adding a validation attestation
#[derive(Accounts)]
pub struct AddValidation<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for removing a validation attestation
#[derive(Accounts)]
pub struct RemoveValidation<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for updating agent DID
#[derive(Accounts)]
pub struct UpdateDID<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for updating agent status
#[derive(Accounts)]
pub struct UpdateStatus<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}

/// Context for deregistering an agent
#[derive(Accounts)]
pub struct DeregisterAgent<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::InvalidStatus
    )]
    pub agent: Account<'info, AgentIdentity>,

    pub owner: Signer<'info>,
}
