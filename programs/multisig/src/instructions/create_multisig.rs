use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::error::MultisigError;
use crate::constants::VAULT_SEED;

pub fn handler(
    ctx: Context<CreateMultisig>,
    owners: Vec<Pubkey>,
    threshold: u64,
) -> Result<()> {
    require!(owners.len() >= threshold as usize, MultisigError::InvalidThreshold);
    let multisig = &mut ctx.accounts.multisig;
    multisig.owners = owners;
    multisig.threshold = threshold;
    multisig.nonce = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(init, payer = payer, space = Multisig::INIT_SPACE)]
    pub multisig: Account<'info, Multisig>,

    /// CHECK: This is just a raw SOL holder, not storing data
    #[account(
        init,
        seeds = [VAULT_SEED.as_bytes(), multisig.key().as_ref()],
        bump,
        payer = payer,
        space = 0, // system accounts holding only SOL can have 0 data
    )]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}