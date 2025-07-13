pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("87AkacnvRbjAq6SYsNey4jv3eY7YzJKd7VzriVPryyhp");

#[program]
pub mod multisig {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
    pub fn create_multisig(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u64) -> Result<()> {
        create_multisig::handler(ctx, owners, threshold)
    }
    pub fn propose_transaction(ctx: Context<ProposeTransaction>, to: Pubkey, lamports: u64) -> Result<()> {
        propose_transaction::handler(ctx, to, lamports)
    }
    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        approve_transaction::handler(ctx)
    }
}
