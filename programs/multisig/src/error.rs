use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigError {
    #[msg("Invalid threshold - must not exceed the number of owners.")]
    InvalidThreshold,
    #[msg("The signer is not an owner of the multisig.")]
    NotOwner,
    #[msg("The transaction has already been executed.")]
    AlreadyExecuted,
    #[msg("This account has already approved the transaction.")]
    AlreadyApproved,
    #[msg("Not enough approvals to execute the transaction.")]
    NotEnoughApprovals,
}
