use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    // 0.
    // The account cannot be initialized because it is already being used.
    #[msg("Swap account already in use")]
    AlreadyInUse,
}
