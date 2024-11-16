use crate::{
    error::CustomError, Collateral, Config, FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT,
};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn check_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(
        health_factor >= config.min_health_factor,
        CustomError::BelowMinimumHealthFactor
    );
    Ok(())
}

// Calcuate health factor for a given Collateral account
pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;

    let collateral_adjusted_for_liquidation_threshold =
        (collateral_value_in_usd * config.liquidation_threshold) / 100;

    msg!(
        "Minted Amount : {:.9}",
        collateral.amount_minted as f64 / 1e9
    );

    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }

    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;

    msg!("Health Factor : {}", health_factor);
    Ok(health_factor)
}

fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    msg!("*** CONVERT USD TO SOL ***");
    msg!("SOL/USD Price : {:.9}", price_in_usd as f64 / 1e9);
    msg!("SOL Amount    : {:.9}", *amount_in_lamports as f64 / 1e9);
    msg!("USD Value     : {:.9}", amount_in_usd as f64 / 1e9);
    // msg!("Price exponent?: {}", price.exponent);

    Ok(amount_in_usd as u64)
}

// Given USD amount, return lamports based on current SOL price
pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_lamports = ((*amount_in_usd as u128) * (LAMPORTS_PER_SOL as u128)) / price_in_usd;

    msg!("*** CONVERT SOL TO USD ***");
    msg!("SOL/USD Price : {:.9}", price_in_usd as f64 / 1e9);
    msg!("USD Amount    : {:.9}", *amount_in_usd as f64 / 1e9);
    msg!("SOL Value     : {:.9}", amount_in_lamports as f64 / 1e9);

    Ok(amount_in_lamports as u64)
}
