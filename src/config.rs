use dotenv::dotenv;
use ethers::types::Address;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub dex1_router: Address,
    pub dex2_router: Address,
    pub weth: Address,
    pub usdc: Address,
    pub trade_amount_usdc: f64,
    pub min_profit_usdc: f64,
    pub poll_interval_secs: u64,
    pub csv_log: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let trade_amount_usdc = env::var("TRADE_AMOUNT_USDC")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .expect("TRADE_AMOUNT_USDC must be a number");
        let min_profit_usdc = env::var("MIN_PROFIT_USDC")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("MIN_PROFIT_USDC must be a number");
        let poll_interval_secs = env::var("POLL_INTERVAL_SECS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("POLL_INTERVAL_SECS must be a number");
        
        Config {
            rpc_url: env::var("POLYGON_RPC").expect("POLYGON_RPC must be set"),
            dex1_router: env::var("DEX1_ROUTER")
                .expect("DEX1_ROUTER must be set")
                .parse()
                .expect("DEX1_ROUTER must be a valid Ethereum address"),
            dex2_router: env::var("DEX2_ROUTER")
                .expect("DEX2_ROUTER must be set")
                .parse()
                .expect("DEX2_ROUTER must be a valid Ethereum address"),
            weth: env::var("WETH")
                .expect("WETH must be set")
                .parse()
                .expect("WETH must be a valid Ethereum address"),
            usdc: env::var("USDC")
                .expect("USDC must be set")
                .parse()
                .expect("USDC must be a valid Ethereum address"),
            trade_amount_usdc,
            min_profit_usdc,
            poll_interval_secs,
            csv_log: env::var("CSV_LOG").unwrap_or_else(|_| "arb_log.csv".to_string()),
        }
    }
}