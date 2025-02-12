use crate::state::EscrowState;
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Default)]
pub struct SetFeesArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}

#[derive(Accounts)]
pub struct SetFees<'info> {
    #[account(mut)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
}
