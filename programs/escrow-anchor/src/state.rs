use crate::consts::MAX_BPS_VALUE;
use crate::errors::EscrowError;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace, Debug)]
pub struct EscrowState {
    pub manager: Pubkey,
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
    pub bump: u8,
}

impl EscrowState {
    pub const SEED: &'static [u8] = b"state";

    pub fn write(
        escrow_state: &mut Account<'_, EscrowState>,
        manager: Option<Pubkey>,
        maker_fee_bps: Option<u16>,
        taker_fee_bps: Option<u16>,
        bump: Option<u8>,
    ) -> Result<Self> {
        manager.map(|m| escrow_state.manager = m);
        maker_fee_bps.map(|fee| escrow_state.maker_fee_bps = fee);
        taker_fee_bps.map(|fee| escrow_state.taker_fee_bps = fee);
        bump.map(|b| escrow_state.bump = b);

        Ok(Self {
            manager: escrow_state.manager,
            maker_fee_bps: escrow_state.maker_fee_bps,
            taker_fee_bps: escrow_state.taker_fee_bps,
            bump: escrow_state.bump,
        })
    }

    /// Calculate token A (offer token) fee amount.
    ///
    /// The fee is to be levied **from the amount transferred from vault to taker**.
    pub fn get_token_a_fee(&self, amount: u64) -> Result<u64> {
        u128::from(amount)
            .checked_mul(u128::from(self.taker_fee_bps))
            .and_then(|v| v.checked_div(u128::from(MAX_BPS_VALUE)))
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(EscrowError::MathError.into())
    }

    /// Calculate token B (ask token) fee amount.
    ///
    /// The fee is to be levied **from the amount transferred from taker to maker**.
    pub fn get_token_b_fee(&self, amount: u64) -> Result<u64> {
        u128::from(amount)
            .checked_mul(u128::from(self.maker_fee_bps))
            .and_then(|v| v.checked_div(u128::from(MAX_BPS_VALUE)))
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(EscrowError::MathError.into())
    }
}

#[account]
#[derive(Default, InitSpace, Debug)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}

impl Offer {
    pub const SEED_PREFIX: &'static [u8] = b"offer";

    pub fn write(
        offer: &mut Account<'_, Offer>,
        id: Option<u64>,
        maker: Option<Pubkey>,
        token_mint_a: Option<Pubkey>,
        token_mint_b: Option<Pubkey>,
        token_b_wanted_amount: Option<u64>,
        bump: Option<u8>,
    ) -> Result<Self> {
        id.map(|i| offer.id = i);
        maker.map(|m| offer.maker = m);
        token_mint_a.map(|tma| offer.token_mint_a = tma);
        token_mint_b.map(|tmb| offer.token_mint_b = tmb);
        token_b_wanted_amount.map(|tbwa| offer.token_b_wanted_amount = tbwa);
        bump.map(|b| offer.bump = b);

        Ok(Self {
            id: offer.id,
            maker: offer.maker,
            token_mint_a: offer.token_mint_a,
            token_mint_b: offer.token_mint_b,
            token_b_wanted_amount: offer.token_b_wanted_amount,
            bump: offer.bump,
        })
    }
}
