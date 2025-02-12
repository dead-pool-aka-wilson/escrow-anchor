use crate::instructions::make_offer::MakeOfferArgs;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub mod cancel_offer;
pub mod collect_fee;
pub mod initialize;
pub mod make_offer;
pub mod set_fees;
pub mod set_manager;
pub mod take_offer;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = funding_account, space = 8 + EscrowState::INIT_SPACE, seeds = [b"state"], bump)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFees<'info> {
    #[account(mut)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(mut)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    /// CHECK: This is new manager of Escrow Account
    /// Anchor does not sanity check on this account
    pub new_manager: UncheckedAccount<'info>,
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
