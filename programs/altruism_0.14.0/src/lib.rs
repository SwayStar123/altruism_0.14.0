use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

#[program]
pub mod altruism_0_14_0 {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        instructions::initialize::initialize(ctx)
    }
}
