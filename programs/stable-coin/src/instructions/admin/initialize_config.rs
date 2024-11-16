use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::{
    Config, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR,
    SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};
