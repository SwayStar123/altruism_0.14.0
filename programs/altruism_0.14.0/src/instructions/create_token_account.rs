use anchor_lang::prelude::*;
use anchor_spl::token::{initialize_account, InitializeAccount, Mint};

pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> ProgramResult {
    initialize_account(
        ctx.accounts.into_spl_ctx()
    ).unwrap();

    let auth_key = ctx.accounts.authority.key();

    ctx.accounts.vault.authority = auth_key;
    ctx.accounts.vault.bump = Pubkey::find_program_address(&[b"vault", auth_key.as_ref()], ctx.program_id).1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        space = 41 + 8,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: ProgramAccount<'info, Vault>,
    pub system_program: AccountInfo<'info>,

    pub account: AccountInfo<'info>,
    // TODO: Figure out how to verify this is the same mint as the one initialized in initialize.rs
    pub mint: CpiAccount<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    #[account(executable)]
    pub token_program: AccountInfo<'info>,
}

// 32 + 8 + 1 = 41
#[account]
pub struct Vault {
    pub authority: Pubkey, // 32
    pub staked_balance: u64, // 8
    pub bump: u8, // 1
}

impl<'a, 'b, 'c, 'info> CreateTokenAccount <'info> {
    pub fn into_spl_ctx(&self) -> CpiContext<'a, 'b, 'c, 'info, InitializeAccount<'info>> {
        let cpi_accounts = InitializeAccount {
            account: self.account.clone(),
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
            authority: self.authority.clone()
        };

        let cpi_program = self.token_program.clone();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}