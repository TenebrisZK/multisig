use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::Transaction;

pub fn handler(
    ctx: Context<ProposeTransaction>,
    to: Pubkey,
    lamports: u64,
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let multisig = &mut ctx.accounts.multisig;
    transaction.multisig = multisig.key();
    transaction.to = to;
    transaction.lamports = lamports;
    transaction.did_execute = false;
    transaction.approvals = vec![];
    multisig.nonce += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct ProposeTransaction<'info> {
    #[account(init, payer = proposer, space = Transaction::INIT_SPACE)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}