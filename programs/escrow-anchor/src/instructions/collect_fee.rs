use crate::state::EscrowState;
use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, CloseAccount, Mint, Token, TokenAccount},
};

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct CollectFeeArgs {
    pub should_close_fee_account: bool,
}

#[derive(Accounts)]
pub struct CollectFee<'info> {
    #[account(seeds = [EscrowState::SEED], bump = escrow_state.bump)]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(mut, address = escrow_state.manager)]
    pub escrow_manager: Signer<'info>,
    pub token_mint_account: Account<'info, Mint>,
    #[account(mut, associated_token::mint = token_mint_account , associated_token::authority = escrow_state)]
    pub escrow_fee_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = escrow_manager, associated_token::mint = token_mint_account, associated_token::authority = escrow_manager)]
    pub manager_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectFee>, args: CollectFeeArgs) -> Result<()> {
    let CollectFee {
        escrow_state,
        escrow_manager,
        escrow_fee_account,
        manager_token_account,
        token_program,
        ..
    } = ctx.accounts;

    let fee_amount = escrow_fee_account.amount;

    let escrow_state_signer_seeds = &[EscrowState::SEED, &[escrow_state.bump]];

    if fee_amount != 0 {
        transfer_token(
            token_program.to_account_info().clone(),
            escrow_fee_account.to_account_info().clone(),
            manager_token_account.to_account_info().clone(),
            escrow_state.to_account_info().clone(),
            fee_amount,
            Some(escrow_state_signer_seeds),
        )?;
    }

    if args.should_close_fee_account {
        close_account(
            CpiContext::new(
                token_program.to_account_info().clone(),
                CloseAccount {
                    account: escrow_fee_account.to_account_info().clone(),
                    authority: escrow_state.to_account_info().clone(),
                    destination: escrow_manager.to_account_info().clone(),
                },
            )
            .with_signer(&[escrow_state_signer_seeds]),
        )?;
    }

    Ok(())
}
