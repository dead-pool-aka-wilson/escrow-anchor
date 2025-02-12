use crate::state::EscrowState;
use anchor_lang::prelude::*;

#[derive(Default, AnchorDeserialize, AnchorSerialize, Debug)]
pub struct InitializeArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = funding_account, space = 8 + EscrowState::INIT_SPACE, seeds = [b"state"], bump)]
    pub escrow_state: Account<'info, EscrowState>,
    pub escrow_manager: Signer<'info>,
    #[account(mut)]
    pub funding_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// impl<'info> Initialize<'info> {
//     pub fn process(&self) -> Result<()> {
//         Ok(())
//     }
// }
