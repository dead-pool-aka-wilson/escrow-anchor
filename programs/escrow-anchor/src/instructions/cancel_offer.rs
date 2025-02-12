use crate::state::{EscrowState, Offer};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    pub escrow_account: Account<'info, Offer>,
    pub token_a_mint_account: Account<'info, Mint>,
    #[account(mut, associated_token::mint = token_a_mint_account, associated_token::authority = maker)]
    pub maker_token_a_account: Account<'info, TokenAccount>,
    #[account(mut,associated_token::mint = token_a_mint_account, associated_token::authority = escrow_account)]
    pub escrow_token_a_vault_account: Account<'info, TokenAccount>,
    pub maker: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
