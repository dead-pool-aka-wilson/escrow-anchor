use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct EscrowState {
    pub manager: Pubkey,
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
    pub bump: u8,
}

#[account]
#[derive(Default, InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}
