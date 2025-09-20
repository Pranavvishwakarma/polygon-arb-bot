mod config;
mod dex;
mod arbitrage;
mod web_monitor; // Add your web_monitor module

use ethers::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use csv::WriterBuilder;
use std::fs::OpenOptions;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive}; // Added ToPrimitive
use web_monitor::{start_server, ArbData, SharedArbData};

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let cfg = config::Config::from_env();
    println!("Config: {:?}", cfg);

    // Prepare CSV log file (create if absent, append always)
    let csv_file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(&cfg.csv_log)?;
    
    let mut wtr = WriterBuilder::new()
        .has_headers(csv_file.metadata()?.len() == 0) // write headers only if file is empty
        .from_writer(csv_file);

    // Provider
    let provider = Provider::<Http>::try_from(cfg.rpc_url.as_str())?;
    let provider = Arc::new(provider);

    // Addresses
    let dex1_router = cfg.dex1_router;
    let dex2_router = cfg.dex2_router;
    let weth = cfg.weth;
    let usdc = cfg.usdc;

    // Decimal conversions
    let trade_amount = Decimal::from_f64(cfg.trade_amount_usdc).unwrap_or_else(|| Decimal::from(1000));
    let min_profit = Decimal::from_f64(cfg.min_profit_usdc).unwrap_or_else(|| Decimal::from(5));
    let gas_sim_usdc = Decimal::from_f64(5.0).unwrap(); // Fixed gas assumption

    // Shared state for live dashboard
    let arb_state: SharedArbData = Arc::new(Mutex::new(Vec::new()));

    // Start web server in background
    let web_state = arb_state.clone();
    tokio::spawn(async move {
        start_server(web_state).await;
    });

    // Graceful shutdown handling
    let shutdown = tokio::spawn(async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
        println!("Shutdown signal received...");
    });

    loop {
        // Check for shutdown signal
        if shutdown.is_finished() {
            println!("Shutting down gracefully...");
            break;
        }

        // Best direction and profit
        match arbitrage::best_direction(
            provider.clone(),
            dex1_router,
            dex2_router,
            usdc,
            weth,
            trade_amount,
            gas_sim_usdc,
        ).await {
            Ok(Some((dir, profit))) => {
                let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
                if profit >= min_profit {
                    println!("🚀 [{ts}] Arb found! direction={} profit={} USDC", dir, profit);
                    // Append to CSV
                    wtr.write_record(&[ts.to_string(), dir.clone(), profit.to_string()])?;
                    wtr.flush()?;

                    // Update web dashboard
                    {
                        let mut data = arb_state.lock().unwrap();
                        data.push(ArbData { timestamp: ts, direction: dir.clone(), profit_usdc: profit.to_f64().unwrap_or(0.0) });
                    }
                } else {
                    println!("— [{ts}] no arb (best {} profit={})", dir, profit);
                }
            }
            Ok(None) => {
                let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
                println!("— [{ts}] no valid simulation results (both directions failed)");
            }
            Err(e) => {
                eprintln!("Error during simulate: {:?}", e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(cfg.poll_interval_secs)).await;
    }

    // Final flush before exit
    wtr.flush()?;
    Ok(())
}
