use crate::errors::EscrowError;
use crate::state::EscrowState;
use crate::utils::assert_is_bps_in_range;
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Default)]
pub struct SetFeesArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}

#[derive(Accounts)]
pub struct SetFees<'info> {
    #[account(mut, seeds = [EscrowState::SEED], bump = escrow_state.bump)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
}

pub fn handler(ctx: Context<SetFees>, args: SetFeesArgs) -> Result<()> {
    assert_is_bps_in_range(args.maker_fee_bps)?;
    assert_is_bps_in_range(args.taker_fee_bps)?;

    let SetFees {
        escrow_state,
        escrow_manager,
    } = ctx.accounts;

    if escrow_manager.key() != escrow_state.manager {
        return Err(EscrowError::ManagerKeyUnauthorized.into());
    }

    let escrow_state_info = EscrowState::write(
        escrow_state,
        None,
        Some(args.maker_fee_bps),
        Some(args.taker_fee_bps),
        None,
    )?;

    msg!("Updated escrow fee : {:?}", escrow_state_info);

    Ok(())
}
