use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct CollectFeeArgs {
    pub should_close_fee_account: bool,
}
