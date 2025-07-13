use anchor_lang::prelude::*;
use crate::constants::MAX_WALLET_OWNERS;

#[account]
#[derive(InitSpace)]
pub struct TransactionInfo {
    pub multisig: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
    #[max_len(MAX_WALLET_OWNERS)]
    pub approvals: Vec<Pubkey>,
    pub did_execute: bool,
}