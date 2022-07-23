use anchor_lang::prelude::*;
use anchor_spl::token::{initialize_account, InitializeAccount};

pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> ProgramResult {
    initialize_account(
        ctx.accounts.into_spl_ctx()
    ).unwrap();

    Ok(())
}


#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> CreateTokenAccount <'info> {
    pub fn into_spl_ctx(&self) -> CpiContext<'a, 'b, 'c, 'info, InitializeAccount<'info>> {
        let cpi_accounts = InitializeAccount {
            account: self.account.clone(),
            mint: self.mint.clone(),
            rent: self.rent.to_account_info(),
            authority: self.authority.clone()
        };

        let cpi_program = self.token_program.clone();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}