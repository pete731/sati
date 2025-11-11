use crate::errors::ErrorCode;
use crate::state::{MandateAccount, MandateStatus};
use anchor_lang::prelude::*;

/// Context for creating a new payment mandate
#[derive(Accounts)]
#[instruction(mandate_id: String)]
pub struct CreateMandate<'info> {
    #[account(
        init,
        payer = user,
        space = MandateAccount::SPACE,
        seeds = [b"mandate", mandate_id.as_bytes()],
        bump
    )]
    pub mandate: Account<'info, MandateAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Context for revoking a mandate
#[derive(Accounts)]
pub struct RevokeMandate<'info> {
    #[account(
        mut,
        constraint = mandate.user_authority == authority.key() @ ErrorCode::UnauthorizedExecution
    )]
    pub mandate: Account<'info, MandateAccount>,

    pub authority: Signer<'info>,
}

/// Context for amending a mandate
#[derive(Accounts)]
pub struct AmendMandate<'info> {
    #[account(
        mut,
        constraint = old_mandate.user_authority == authority.key() @ ErrorCode::UnauthorizedExecution
    )]
    pub old_mandate: Account<'info, MandateAccount>,

    pub authority: Signer<'info>,
}

/// Context for executing a mandate
#[derive(Accounts)]
pub struct ExecuteMandate<'info> {
    #[account(mut)]
    pub mandate: Account<'info, MandateAccount>,

    pub executor: Signer<'info>,
}

/// Context for checking context drift
#[derive(Accounts)]
pub struct CheckContextDrift<'info> {
    #[account(mut)]
    pub mandate: Account<'info, MandateAccount>,
}

/// Context for revalidating a zombie mandate
#[derive(Accounts)]
pub struct RevalidateMandate<'info> {
    #[account(
        mut,
        constraint = mandate.user_authority == user.key() @ ErrorCode::UnauthorizedRevalidation
    )]
    pub mandate: Account<'info, MandateAccount>,

    pub user: Signer<'info>,
}

/// Context for enforcing mandate expiry
#[derive(Accounts)]
pub struct EnforceExpiry<'info> {
    #[account(mut)]
    pub mandate: Account<'info, MandateAccount>,
}
