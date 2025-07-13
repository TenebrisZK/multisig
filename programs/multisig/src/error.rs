use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigError {
    #[msg("Invalid threshold - must not exceed the number of owners.")]
    InvalidThreshold,
}
