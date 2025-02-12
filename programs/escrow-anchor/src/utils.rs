use crate::{consts::MAX_BPS_VALUE, errors::EscrowError};
use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, transfer, CloseAccount, Transfer};

pub fn assert_is_bps_in_range(bps: u16) -> Result<()> {
    if bps > MAX_BPS_VALUE {
        return err!(EscrowError::MaxBpsValueExceeded);
    }

    Ok(())
}

pub fn transfer_token<'info>(
    token_program: AccountInfo<'info>,
    from_token_account: AccountInfo<'info>,
    to_token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    signer_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    // 기본 CPI 컨텍스트 구성
    let cpi_ctx = CpiContext::new(
        token_program,
        Transfer {
            from: from_token_account,
            to: to_token_account,
            authority,
        },
    );

    match signer_seeds {
        Some(seeds) => {
            let seeds = [seeds];
            transfer(cpi_ctx.with_signer(&seeds), amount)?;
        }
        None => {
            transfer(cpi_ctx, amount)?;
        }
    }

    Ok(())
}

pub fn close_token_account<'info>(
    token_program: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    signer_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        token_program,
        CloseAccount {
            account: token_account,
            authority,
            destination,
        },
    );

    match signer_seeds {
        Some(seeds) => {
            let seeds = [seeds];
            close_account(cpi_ctx.with_signer(&seeds))?;
        }
        None => {
            close_account(cpi_ctx)?;
        }
    }

    Ok(())
}
