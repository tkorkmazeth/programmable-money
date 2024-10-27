use anchor_lang::prelude::*;

mod constants;
mod error;
mod state;
use constants::*;
use instructions::*;
use state::*;
mod instructions;

declare_id!("3TxyVAd9fi8YfDjuwbVKZNhddDqr69SYCBCvTmpt5umX");

#[program]
pub mod stable_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
