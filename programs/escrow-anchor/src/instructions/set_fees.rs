use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Default)]
pub struct SetFeesArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}
