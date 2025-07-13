use anchor_lang::prelude::*;
use crate::error::MultisigError;
use crate::state::{TransactionInfo, Multisig};
use crate::constants::VAULT_SEED;
use anchor_lang::system_program::{transfer, Transfer};


pub fn handler(ctx: Context<ExecuteTransaction>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let transaction_info = &mut ctx.accounts.transaction_info;
    let vault = &ctx.accounts.vault;
    let recipient = &ctx.accounts.to;

    require!(!transaction_info.did_execute, MultisigError::AlreadyExecuted);
    require!(
        transaction_info.approvals.len() as u64 >= multisig.threshold,
        MultisigError::NotEnoughApprovals
    );

    let mkey= multisig.key();
    let signer_seeds = &[
        b"vault", 
        mkey.as_ref(), 
        &[ctx.bumps.vault]
        ];

    transfer(
        CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: vault.to_account_info(),
            to: recipient.to_account_info(),
        },
        &[signer_seeds],
        ),
        transaction_info.lamports,
    )?;

    transaction_info.did_execute = true;

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub transaction_info: Account<'info, TransactionInfo>,

    /// CHECK: This is the PDA vault account for the multisig (holds the SOL)
    #[account(
        mut,
        seeds = [VAULT_SEED.as_bytes(), multisig.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    /// CHECK: Recipient (no validation needed, just receives lamports)
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}