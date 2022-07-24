use crate::state::Grant;
use anchor_lang::prelude::*;

/// This instruction lets an author cancel a grant.
#[derive(Accounts)]
pub struct CancelGrantAuthor<'info> {
    #[account(mut)]
    author: Signer<'info>,

    #[account(
        mut,
        seeds = [b"grant", grant.grant_num.to_be_bytes().as_ref()],
        bump = grant.bump,
        has_one = author
    )]
    grant: Account<'info, Grant>,

    system_program: Program<'info, System>,
}

pub fn cancel_grant_author(ctx: Context<CancelGrantAuthor>) -> Result<()> {
    ctx.accounts.grant.cancel_grant()
}
