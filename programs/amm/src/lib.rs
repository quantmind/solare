use anchor_lang::prelude::*;

pub mod errors;
pub mod mm;
pub mod logic;



declare_id!("Ct6fnNMhwq2vBonYjsjDZjrtaY4o4bcxppVGWz7QWM2d");


#[program]
pub mod amm {
    use super::*;

    /// Initialize a new AMM program
    pub fn initialize(ctx: Context<Initialize>, fees: mm::Fees) -> Result<()> {
        // check if the account is already initialized
        if ctx.accounts.amm.is_initialized {
            return Err(errors::AmmError::AlreadyInUse.into());
        }

        // account info key
        let ai_key = ctx.accounts.amm.to_account_info().key.to_bytes();

        let (authority, bump_seed) = Pubkey::find_program_address(
            &[&ai_key],
            ctx.program_id,
        );

        let amm = &mut ctx.accounts.amm;
        amm.fees = fees;
        amm.is_initialized = true;
        amm.pool_mint = *ctx.accounts.pool_mint.to_account_info().key;


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: Safe
    pub authority: AccountInfo<'info>,
    #[account(signer, zero)]
    pub amm: Box<Account<'info, mm::Amm>>,
}
