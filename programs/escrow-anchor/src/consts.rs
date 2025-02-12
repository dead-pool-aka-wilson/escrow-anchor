use anchor_lang::prelude::*;

pub const MAX_BPS_VALUE: u16 = 10_000;

#[cfg(not(feature = "localnet"))]
pub const INITIAL_MANAGER: Pubkey = pubkey!("TH1S1SNoTAVAL1DPUBKEYUSETHE1MPLS1D1NSTEAD11");

#[cfg(feature = "localnet")]
pub const INITIAL_MANAGER: Pubkey = pubkey!("GRaji5MQuLfPPYX1x9RK8Eb34QzCsawegMjmv3561nAG");

#[cfg(feature = "localnet")]
pub const INITIAL_MANAGER_KEYPAIR: [u8; 64] = [
    225, 36, 24, 115, 153, 63, 218, 187, 113, 14, 20, 214, 38, 240, 197, 73, 225, 49, 209, 116,
    135, 218, 130, 216, 200, 199, 123, 63, 46, 156, 175, 159, 229, 43, 99, 157, 153, 13, 92, 91,
    114, 10, 209, 117, 130, 3, 60, 193, 20, 166, 97, 167, 91, 95, 189, 176, 42, 5, 137, 51, 83,
    183, 10, 61,
];
