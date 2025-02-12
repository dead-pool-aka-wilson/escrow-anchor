use crate::errors::EscrowError;
use crate::state::EscrowState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(mut)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    /// CHECK: This is new manager of Escrow Account
    /// Anchor does not sanity check on this account
    pub new_manager: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetManager>) -> Result<()> {
    let SetManager {
        escrow_state,
        escrow_manager,
        new_manager,
    } = ctx.accounts;

    if escrow_manager.key() != escrow_state.manager {
        return Err(EscrowError::ManagerKeyUnauthorized.into());
    }

    if new_manager.key() == escrow_state.manager {
        return Err(EscrowError::ManagerKeyAlreadySet.into());
    }

    let escrow_state_info =
        EscrowState::write(escrow_state, Some(new_manager.key()), None, None, None)?;

    msg!("Updated escrow manager: {:?}", escrow_state_info);

    Ok(())
}
