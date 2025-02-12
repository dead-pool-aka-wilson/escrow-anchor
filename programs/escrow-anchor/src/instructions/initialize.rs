use anchor_lang::prelude::*;

#[derive(Default, AnchorDeserialize, AnchorSerialize, Debug)]
pub struct InitializeArgs {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}
