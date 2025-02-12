use crate::errors::EscrowError;
use crate::state::Offer;
use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, CloseAccount, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(mut, seeds = [Offer::SEED_PREFIX, maker.key().as_ref(), escrow_account.id.to_le_bytes().as_ref()], bump = escrow_account.bump, close = funding_account)]
    pub escrow_account: Account<'info, Offer>,
    #[account(address = escrow_account.token_mint_a)]
    pub token_a_mint_account: Account<'info, Mint>,
    #[account(init_if_needed, payer=funding_account, associated_token::mint = token_a_mint_account, associated_token::authority = maker)]
    pub maker_token_a_account: Account<'info, TokenAccount>,
    #[account(mut,associated_token::mint = token_a_mint_account, associated_token::authority = escrow_account)]
    pub escrow_token_a_vault_account: Account<'info, TokenAccount>,
    #[account(address = escrow_account.maker)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CancelOffer>) -> Result<()> {
    let CancelOffer {
        escrow_account,
        maker_token_a_account,
        escrow_token_a_vault_account,
        maker,
        funding_account,
        token_program,
        ..
    } = ctx.accounts;

    let escrow_account_signer_seeds = &[
        Offer::SEED_PREFIX,
        maker.key.as_ref(),
        &escrow_account.id.to_le_bytes(),
        &[escrow_account.bump],
    ];

    let vault_amount_a = escrow_token_a_vault_account.amount;
    let maker_token_a_amount_before_transfer = maker_token_a_account.amount;

    transfer_token(
        token_program.to_account_info().clone(),
        escrow_token_a_vault_account.to_account_info().clone(),
        maker_token_a_account.to_account_info().clone(),
        escrow_account.to_account_info().clone(),
        vault_amount_a,
        Some(escrow_account_signer_seeds),
    )?;

    maker_token_a_account.reload()?;

    assert_eq!(
        maker_token_a_account.amount,
        maker_token_a_amount_before_transfer
            .checked_add(vault_amount_a)
            .ok_or(EscrowError::MathError)?
    );

    let close_account_cpi = CpiContext::new(
        token_program.to_account_info().clone(),
        CloseAccount {
            account: escrow_token_a_vault_account.to_account_info().clone(),
            authority: escrow_account.to_account_info().clone(),
            destination: funding_account.to_account_info().clone(),
        },
    );

    close_account(close_account_cpi.with_signer(&[escrow_account_signer_seeds]))?;

    Ok(())
}
