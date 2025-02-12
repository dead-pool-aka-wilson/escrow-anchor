use anchor_lang::prelude::*;

use crate::{
    consts::INITIAL_MANAGER, errors::EscrowError, state::EscrowState, utils::assert_is_bps_in_range,
};

#[derive(Default, AnchorDeserialize, AnchorSerialize, Debug)]
pub struct InitializeArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = funding_account, space = 8 + EscrowState::INIT_SPACE, seeds = [EscrowState::SEED], bump)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
    // Check the range of bps values in args
    assert_is_bps_in_range(args.maker_fee_bps)?;
    assert_is_bps_in_range(args.taker_fee_bps)?;

    let Initialize {
        escrow_state,
        escrow_manager,
        ..
    } = ctx.accounts;

    // Ensure the initial manager pubkey is correct
    if escrow_manager.key() != INITIAL_MANAGER {
        return Err(EscrowError::InitialManagerKeyMismatch.into());
    }

    let escrow_state_info = EscrowState::write(
        escrow_state,
        Some(escrow_manager.key()),
        Some(args.maker_fee_bps),
        Some(args.taker_fee_bps),
        Some(ctx.bumps.escrow_state),
    )?;

    msg!("Initialized escrow state: {:?}", escrow_state_info);

    Ok(())
}
