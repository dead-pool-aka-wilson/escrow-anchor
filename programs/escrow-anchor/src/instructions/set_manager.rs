use crate::state::EscrowState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(mut)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    /// CHECK: This is new manager of Escrow Account
    /// Anchor does not sanity check on this account
    pub new_manager: UncheckedAccount<'info>,
}
