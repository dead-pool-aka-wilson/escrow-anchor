use anchor_lang::prelude::*;

pub mod consts;
pub mod instructions;
pub mod state;

use crate::instructions::*;
use crate::instructions::{
    initialize::InitializeArgs, make_offer::MakeOfferArgs, set_fees::SetFeesArgs,
};

declare_id!("4feFvLpc3CaHWLsxD954DCL9mjAdSj61jZD85jag8A32");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn set_fees(ctx: Context<SetFees>, args: SetFeesArgs) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn set_manager(ctx: Context<SetManager>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn collect_fee(ctx: Context<CollectFee>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn make_offer(ctx: Context<MakeOffer>, args: MakeOfferArgs) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
