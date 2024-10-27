use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Colleteral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump: u8,
    pub bum_sol_account: u8,
    pub is_initialized: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,          // authority of the this program config account
    pub mint_account: Pubkey,       // the stablecoin mint address, which is a PDA
    pub liquidation_threshold: u64, // determines how much extra collateral is required
    pub liquidation_bonus: u64,     // % bonus lamports to liquidator for liquidating an account
    pub min_health_factor: u64, // minimum health factor, if below min then Collateral account can be liquidated
    pub bump: u8,               // store bump seed for this config account
    pub bump_mint_account: u8,  // store bump seed for th
}
