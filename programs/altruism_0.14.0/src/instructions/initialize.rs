use anchor_lang::prelude::*;
use anchor_spl::token::{initialize_mint, InitializeMint};

pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    initialize_mint(
        ctx.accounts.into_spl_ctx(),
        9,
        ctx.accounts.authority.key,
        Some(ctx.accounts.authority.key)
    ).unwrap();

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub mint: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> Initialize<'info> {
    pub fn into_spl_ctx(&self) -> CpiContext<'a, 'b, 'c, 'info, InitializeMint<'info>> {
        let cpi_accounts = InitializeMint {
            mint: self.mint.clone(),
            rent: self.rent.to_account_info(),
        };

        let cpi_program = self.token_program.clone();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}