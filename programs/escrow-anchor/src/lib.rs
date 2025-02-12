use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::*;
use crate::instructions::{
    collect_fee::CollectFeeArgs, initialize::InitializeArgs, make_offer::MakeOfferArgs,
    set_fees::SetFeesArgs,
};

declare_id!("2izpriWVFuFivHicKpjJq3F7K8RKTs8qGcsvZBod7gSQ");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        initialize::handler(ctx, args)?;
        Ok(())
    }
    pub fn set_fees(ctx: Context<SetFees>, args: SetFeesArgs) -> Result<()> {
        set_fees::handler(ctx, args)?;
        Ok(())
    }
    pub fn set_manager(ctx: Context<SetManager>) -> Result<()> {
        set_manager::handler(ctx)?;
        Ok(())
    }
    pub fn collect_fee(ctx: Context<CollectFee>, args: CollectFeeArgs) -> Result<()> {
        collect_fee::handler(ctx, args)?;
        Ok(())
    }
    pub fn make_offer(ctx: Context<MakeOffer>, args: MakeOfferArgs) -> Result<()> {
        make_offer::handler(ctx, args)?;
        Ok(())
    }
    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        take_offer::handler(ctx)?;
        Ok(())
    }
    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        cancel_offer::handler(ctx)?;
        Ok(())
    }
}
