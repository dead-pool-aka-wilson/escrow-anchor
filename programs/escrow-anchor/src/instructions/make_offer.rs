use crate::state::Offer;
use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
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
    #[account(init, space=8+Offer::INIT_SPACE, payer = funding_account, seeds = [Offer::SEED_PREFIX,maker.key().as_ref(), args.id.to_le_bytes().as_ref() ], bump)]
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

pub fn handler(ctx: Context<MakeOffer>, args: MakeOfferArgs) -> Result<()> {
    let MakeOffer {
        escrow_account,
        token_a_mint_account,
        token_b_mint_account,
        maker_token_a_account,
        escrow_token_a_vault_account,
        maker,
        token_program,
        ..
    } = ctx.accounts;

    let offer = Offer::write(
        escrow_account,
        Some(args.id),
        Some(maker.key()),
        Some(token_a_mint_account.key()),
        Some(token_b_mint_account.key()),
        Some(args.token_b_wanted_amount),
        Some(ctx.bumps.escrow_account),
    )?;

    transfer_token(
        token_program.to_account_info().clone(),
        maker_token_a_account.to_account_info().clone(),
        escrow_token_a_vault_account.to_account_info().clone(),
        maker.to_account_info().clone(),
        args.token_a_offered_amount,
        None,
    )?;

    escrow_token_a_vault_account.reload()?;
    let vault_token_amount = escrow_token_a_vault_account.amount;

    assert_eq!(vault_token_amount, args.token_a_offered_amount);

    msg!("Offer created successfully : {:?}", offer);

    Ok(())
}
