use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::*;

#[derive(Accounts)]
pub struct PerpCloseMarket<'info> {
    #[account(
        constraint = group.load()?.testing == 1,
        has_one = admin,
    )]
    pub group: AccountLoader<'info, Group>,
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = group,
        has_one = bids,
        has_one = asks,
        has_one = event_queue,
        close = sol_destination
    )]
    pub perp_market: AccountLoader<'info, PerpMarket>,

    #[account(
        mut,
        close = sol_destination
    )]
    pub bids: AccountLoader<'info, BookSide>,

    #[account(
        mut,
        close = sol_destination
    )]
    pub asks: AccountLoader<'info, BookSide>,

    #[account(
        mut,
        close = sol_destination
    )]
    pub event_queue: AccountLoader<'info, EventQueue>,

    #[account(mut)]
    pub sol_destination: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[allow(clippy::too_many_arguments)]
pub fn perp_close_market(_ctx: Context<PerpCloseMarket>) -> Result<()> {
    Ok(())
}