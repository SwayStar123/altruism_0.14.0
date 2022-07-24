use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn};

use marinade_onchain_helper::{cpi_context_accounts::MarinadeLiquidUnstake, cpi_util};

use crate::Vault;

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
    msg!("Depositing: {}", amount);
    let cpi_ctx = ctx.accounts.into_marinade_ctx();
    let data = marinade_finance::instruction::Deposit { lamports: amount };
    // call Marinade
    cpi_util::invoke_signed(cpi_ctx, data)?;

    mint_to(ctx.accounts.into_spl_ctx(), amount)?;

    Ok(())
}


#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    #[account(mut)]
    pub msol_mint: AccountInfo<'info>,
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: ProgramAccount<'info, Vault>,
    pub msol_mint_authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    // accounts added are: Marinade main program ID
    #[account(address = marinade_finance::ID)]
    pub marinade_finance_program: AccountInfo<'info>,

    pub token_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> Deposit<'info> {
    pub fn into_spl_ctx(&self) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo { 
            mint: self.mint.clone(),
            to: self.token_account.clone(),
            authority: self.authority.clone()
        };
    
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    pub fn into_marinade_ctx(&self) -> CpiContext<'a, 'b, 'c, 'info, MarinadeLiquidUnstake<'info>> {
        let cpi_accounts = MarinadeDeposit {
            state: self.state.clone(),
            msol_mint: self.msol_mint.clone(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.clone(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.clone(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.clone(),
            reserve_pda: self.reserve_pda.clone(),
            transfer_from: self.authority.clone(),
            mint_to: self.vault.to_account_info(),
            msol_mint_authority: self.msol_mint_authority.clone(),
            system_program: self.system_program.clone(),
            token_program: self.token_program.clone(),
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}