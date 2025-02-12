use crate::state::{EscrowState, Offer};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_account: Account<'info, Offer>,
    pub token_a_mint_account: Account<'info, Mint>,
    pub token_b_mint_account: Account<'info, Mint>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_b_mint_account, associated_token::authority = maker)]
    pub maker_token_b_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_a_mint_account, associated_token::authority = taker)]
    pub taker_token_a_account: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = token_b_mint_account, associated_token::authority = taker)]
    pub taker_token_b_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_a_mint_account, associated_token::authority = escrow_state)]
    pub escrow_token_a_fee_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_b_mint_account, associated_token::authority = escrow_state)]
    pub escrow_token_b_fee_account: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = token_a_mint_account, associated_token::authority = escrow_account)]
    pub escrow_token_a_vault_account: Account<'info, TokenAccount>,
    /// CHECK : address of maker wallet
    pub maker: UncheckedAccount<'info>,
    pub taker: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
