use anchor_lang::prelude::*;
use crate::constants::MAX_WALLET_OWNERS;

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(MAX_WALLET_OWNERS)]
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u64,
}