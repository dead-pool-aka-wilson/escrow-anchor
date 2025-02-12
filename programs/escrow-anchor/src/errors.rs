use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Escrow state key provided does not match expected")]
    EscrowStateKeyMismatch,

    #[msg("Initial manager key provided does not match expected")]
    InitialManagerKeyMismatch,

    #[msg("Basis point value provided exceeded allowed range")]
    MaxBpsValueExceeded,

    #[msg("Manager key provided is not authorized")]
    ManagerKeyUnauthorized,

    #[msg("Manager key provided is the current manager")]
    ManagerKeyAlreadySet,

    #[msg("Offer key provided does not match expected")]
    OfferKeyMismatch,

    #[msg("Token account provided does not match expected")]
    TokenAccountMismatch,

    #[msg("Argument provided resulted in overflow")]
    MathError,
}
