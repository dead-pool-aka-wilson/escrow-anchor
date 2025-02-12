use crate::{
    errors::EscrowError,
    state::{EscrowState, Offer},
    utils::{close_token_account, transfer_token},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(seeds = [EscrowState::SEED], bump = escrow_state.bump)]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    #[account(mut, seeds = [Offer::SEED_PREFIX, maker.key().as_ref(), escrow_account.id.to_le_bytes().as_ref()], bump = escrow_account.bump, close=funding_account)]
    pub escrow_account: Box<Account<'info, Offer>>,
    #[account(address = escrow_account.token_mint_a)]
    pub token_a_mint_account: Account<'info, Mint>,
    #[account(address = escrow_account.token_mint_b)]
    pub token_b_mint_account: Account<'info, Mint>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_b_mint_account, associated_token::authority = maker, associated_token::token_program = token_program)]
    pub maker_token_b_account: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_a_mint_account, associated_token::authority = taker, associated_token::token_program = token_program)]
    pub taker_token_a_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, associated_token::mint = token_b_mint_account, associated_token::authority = taker, associated_token::token_program = token_program)]
    pub taker_token_b_account: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_a_mint_account, associated_token::authority = escrow_state, associated_token::token_program = token_program)]
    pub escrow_token_a_fee_account: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = funding_account, associated_token::mint = token_b_mint_account, associated_token::authority = escrow_state, associated_token::token_program = token_program)]
    pub escrow_token_b_fee_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, associated_token::mint = token_a_mint_account, associated_token::authority = escrow_account, associated_token::token_program = token_program)]
    pub escrow_token_a_vault_account: Account<'info, TokenAccount>,
    /// CHECK : address of maker wallet
    #[account(address = escrow_account.maker)]
    pub maker: UncheckedAccount<'info>,
    pub taker: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TakeOffer>) -> Result<()> {
    let TakeOffer {
        escrow_state,
        escrow_account,
        maker_token_b_account,
        taker_token_a_account,
        taker_token_b_account,
        escrow_token_a_fee_account,
        escrow_token_b_fee_account,
        escrow_token_a_vault_account,
        maker,
        taker,
        token_program,
        funding_account,
        ..
    } = ctx.accounts;

    let escrow_account_signer_seeds = &[
        Offer::SEED_PREFIX,
        maker.key.as_ref(),
        &escrow_account.id.to_le_bytes(),
        &[escrow_account.bump],
    ];

    let vault_amount_a = escrow_token_a_vault_account.amount;
    let taker_amount_a_before_transfer = taker_token_a_account.amount;
    let maker_amount_b_before_transfer = maker_token_b_account.amount;

    let token_b_fee_amount = escrow_state.get_token_b_fee(vault_amount_a)?;

    transfer_token(
        token_program.to_account_info().clone(),
        taker_token_b_account.to_account_info().clone(),
        escrow_token_b_fee_account.to_account_info().clone(),
        taker.to_account_info().clone(),
        token_b_fee_amount,
        None,
    )?;

    let token_b_to_transfer_after_fee = escrow_account
        .token_b_wanted_amount
        .checked_sub(token_b_fee_amount)
        .ok_or(EscrowError::MathError)?;

    transfer_token(
        token_program.to_account_info().clone(),
        taker_token_b_account.to_account_info().clone(),
        maker_token_b_account.to_account_info().clone(),
        taker.to_account_info().clone(),
        token_b_to_transfer_after_fee,
        None,
    )?;

    let token_a_fee_amount = escrow_state.get_token_a_fee(vault_amount_a)?;

    transfer_token(
        token_program.to_account_info().clone(),
        escrow_token_a_vault_account.to_account_info().clone(),
        escrow_token_a_fee_account.to_account_info().clone(),
        escrow_account.to_account_info().clone(),
        token_a_fee_amount,
        Some(escrow_account_signer_seeds),
    )?;

    let token_a_to_transfer_after_fee = vault_amount_a
        .checked_sub(token_a_fee_amount)
        .ok_or(EscrowError::MathError)?;

    transfer_token(
        token_program.to_account_info().clone(),
        escrow_token_a_vault_account.to_account_info().clone(),
        taker_token_a_account.to_account_info().clone(),
        escrow_account.to_account_info().clone(),
        token_a_to_transfer_after_fee,
        Some(escrow_account_signer_seeds),
    )?;

    taker_token_a_account.reload()?;
    maker_token_b_account.reload()?;
    escrow_token_a_fee_account.reload()?;

    assert_eq!(
        taker_token_a_account.amount,
        taker_amount_a_before_transfer
            .checked_add(vault_amount_a)
            .ok_or(EscrowError::MathError)?
            .checked_sub(token_a_fee_amount)
            .ok_or(EscrowError::MathError)?
    );
    assert_eq!(
        maker_token_b_account.amount,
        maker_amount_b_before_transfer
            .checked_add(escrow_account.token_b_wanted_amount)
            .ok_or(EscrowError::MathError)?
            .checked_sub(token_b_fee_amount)
            .ok_or(EscrowError::MathError)?
    );

    close_token_account(
        token_program.to_account_info().clone(),
        escrow_token_a_vault_account.to_account_info().clone(),
        escrow_account.to_account_info().clone(),
        funding_account.to_account_info().clone(),
        Some(escrow_account_signer_seeds),
    )?;

    Ok(())
}
