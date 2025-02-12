use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct CollectFeeArgs {
    pub should_close_fee_account: bool,
}

#[derive(Accounts)]
pub struct CollectFee<'info> {
    pub escrow_state: Account<'info, EscrowState>,
    #[account(mut)]
    pub escrow_manager: Signer<'info>,
    pub token_mint_account: Account<'info, Mint>,
    #[account(mut,associated_token::mint = token_mint_account , associated_token::authority = escrow_state)]
    pub escrow_fee_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = escrow_manager, associated_token::mint = token_mint_account, associated_token::authority = escrow_manager)]
    pub manager_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
