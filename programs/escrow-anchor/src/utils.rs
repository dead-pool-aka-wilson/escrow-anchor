// use anchor_lang::prelude::*;
// use anchor_spl::{associated_token, token};

// use crate::{consts::MAX_BPS_VALUE, errors::EscrowError};

// pub fn assert_is_associated_token_account(
//     token_account_address: &Pubkey,
//     owner: &Pubkey,
//     mint: &Pubkey,
// ) -> Result<()> {
//     let associated_token_account_address =
//         &associated_token::get_associated_token_address(owner, mint);

//     if token_account_address != associated_token_account_address {
//         return err!(EscrowError::TokenAccountMismatch);
//     }

//     Ok(())
// }

// pub fn assert_token_account_mint_and_owner(
//     token_account_info: &AccountInfo,
//     owner: &Pubkey,
//     mint: &Pubkey,
// ) -> Result<()> {
//     let token_account = token::state::Account::unpack(&token_account_info.data.borrow())?;
//     if token_account.mint != *mint || token_account.owner != *owner {
//         return err!(EscrowError::TokenAccountMismatch);
//     }

//     Ok(())
// }

// pub fn assert_is_bps_in_range(bps: u16) -> Result<()> {
//     if bps > MAX_BPS_VALUE {
//         return err!(EscrowError::MaxBpsValueExceeded);
//     }

//     Ok(())
// }
