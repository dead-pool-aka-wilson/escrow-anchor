use crate::state::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct MakeOfferArgs {
    pub id: u64,
    pub token_a_offered_amount: u64,
    pub token_b_wanted_amount: u64,
}

#[derive(Accounts)]
#[instruction(args: MakeOfferArgs)]
pub struct MakeOffer<'info> {
    #[account(init, space=8+Offer::INIT_SPACE, payer = funding_account, seeds = [b"offer",maker.key().as_ref(), args.id.to_le_bytes().as_ref() ], bump)]
    pub escrow_account: Account<'info, Offer>,
    pub token_a_mint_account: Account<'info, Mint>,
    pub token_b_mint_account: Account<'info, Mint>,
    #[account(mut, associated_token::mint = token_a_mint_account, associated_token::authority = maker)]
    pub maker_token_a_account: Account<'info, TokenAccount>,
    #[account(init, payer = funding_account,  associated_token::mint = token_a_mint_account, associated_token::authority = escrow_account)]
    pub escrow_token_a_vault_account: Account<'info, TokenAccount>,
    pub maker: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
