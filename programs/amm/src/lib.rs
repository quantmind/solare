use anchor_lang::prelude::*;

pub mod errors;
pub mod mm;

declare_id!("Ct6fnNMhwq2vBonYjsjDZjrtaY4o4bcxppVGWz7QWM2d");

#[program]
pub mod amm {
    use super::*;

    /// Initialize the AMM program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // check if the account is already initialized
        if ctx.accounts.amm.is_initialized {
            return Err(errors::AmmError::AlreadyInUse.into());
        }

        let amm = &mut ctx.accounts.amm;
        amm.is_initialized = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: Safe
    pub authority: AccountInfo<'info>,
    #[account(signer, zero)]
    pub amm: Box<Account<'info, Amm>>,
}

#[account]
pub struct Amm {
    pub initializer_key: Pubkey,
    /// Is the swap initialized, with data written to it
    pub is_initialized: bool,
}
