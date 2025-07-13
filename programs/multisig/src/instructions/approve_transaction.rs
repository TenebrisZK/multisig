use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::Transaction;
use crate::error::MultisigError;

pub fn handler(ctx: Context<ApproveTransaction>) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let multisig = &ctx.accounts.multisig;
    let signer = &ctx.accounts.signer;

    require!(
        multisig.owners.contains(&signer.key()),
        MultisigError::NotOwner
    );
    require!(!transaction.did_execute, MultisigError::AlreadyExecuted);
    require!(
        !transaction.approvals.contains(&signer.key()),
        MultisigError::AlreadyApproved
    );
    transaction.approvals.push(signer.key());
    Ok(())
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
}