use ethers::prelude::*;
use ethers::providers::Provider;
use std::sync::Arc;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::dex;

/// Convert human USDC (like 1000.23) -> U256 (USDC has 6 decimals)
pub fn usdc_to_wei(amount_usdc: Decimal) -> U256 {
    let scaled = amount_usdc * Decimal::from_i128_with_scale(1_000_000, 0); // * 1e6
    // Round down to avoid accidental overspend
    let as_i128 = scaled.trunc().to_i128().expect("overflow converting to i128");
    // U256 from u128
    let as_u128 = as_i128 as u128;
    U256::from(as_u128)
}

/// Convert U256 (6 decimals) -> Decimal human USDC
pub fn u256_to_usdc(value: U256) -> Decimal {
    let as_u128 = value.as_u128();
    // Use from_i128_with_scale instead of from_u128
    Decimal::from_i128_with_scale(as_u128 as i128, 0) / Decimal::from_i128_with_scale(1_000_000, 0)
}

/// Run simulation:
/// - Buy WETH on DEX A using `trade_amount_usdc` (path = [USDC, WETH]) -> get weth_received
/// - Sell that weth on DEX B (path = [WETH, USDC]) -> get usdc_received
/// Profit = usdc_received - trade_amount_usdc - gas_usdc
pub async fn simulate_trade_usdc(
    provider: Arc<Provider<ethers::providers::Http>>,
    dex_buy_router: Address,
    dex_sell_router: Address,
    usdc_addr: Address,
    weth_addr: Address,
    trade_amount_usdc: Decimal,
    gas_usdc: Decimal,
) -> Result<(String, Decimal)> {
    // amount_in for buy: trade_amount_usdc in USDC base units
    let amount_in_buy = usdc_to_wei(trade_amount_usdc);

    // path [USDC, WETH]
    let path_buy = vec![usdc_addr, weth_addr];
    let amounts_buy = dex::get_amounts_out(provider.clone(), dex_buy_router, amount_in_buy, path_buy).await?;
    // amounts_buy[0] == amount_in_buy, amounts_buy[1] == weth_received (in wei)

    let weth_received = amounts_buy[1];

    // Now sell weth_received on DEX sell: path [WETH, USDC]
    let path_sell = vec![weth_addr, usdc_addr];
    let amounts_sell = dex::get_amounts_out(provider.clone(), dex_sell_router, weth_received, path_sell).await?;
    let usdc_received = amounts_sell[1];

    let usdc_received_dec = u256_to_usdc(usdc_received);
    let profit = usdc_received_dec - trade_amount_usdc - gas_usdc;

    Ok(("BUY_ON_A_SELL_B".to_string(), profit))
}

/// Helper that tries both directions and returns best (A->B or B->A)
/// Returns Ok(None) if both directions fail, Ok(Some((direction, profit))) if at least one works
pub async fn best_direction(
    provider: Arc<Provider<ethers::providers::Http>>,
    dex_a: Address,
    dex_b: Address,
    usdc_addr: Address,
    weth_addr: Address,
    trade_amount_usdc: Decimal,
    gas_usdc: Decimal,
) -> Result<Option<(String, Decimal)>> {
    // A buy -> B sell
    let res_ab = simulate_trade_usdc(provider.clone(), dex_a, dex_b, usdc_addr, weth_addr, trade_amount_usdc, gas_usdc).await;
    // B buy -> A sell
    let res_ba = simulate_trade_usdc(provider.clone(), dex_b, dex_a, usdc_addr, weth_addr, trade_amount_usdc, gas_usdc).await;

    let profit_ab = match res_ab {
        Ok((dir, p)) => Some((dir, p)),
        Err(e) => {
            eprintln!("A->B simulation failed: {:?}", e);
            None
        }
    };
    
    let profit_ba = match res_ba {
        Ok((dir, p)) => Some((dir, p)),
        Err(e) => {
            eprintln!("B->A simulation failed: {:?}", e);
            None
        }
    };

    match (profit_ab, profit_ba) {
        (Some((dir_ab, profit_ab)), Some((dir_ba, profit_ba))) => {
            if profit_ab >= profit_ba {
                Ok(Some((format!("A->B ({})", dir_ab), profit_ab)))
            } else {
                Ok(Some((format!("B->A ({})", dir_ba), profit_ba)))
            }
        }
        (Some(profit), None) | (None, Some(profit)) => Ok(Some(profit)),
        (None, None) => Ok(None),
    }
}