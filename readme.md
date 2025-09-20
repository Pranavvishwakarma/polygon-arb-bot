# Polygon Arbitrage Opportunity Detector Bot

A Rust-based bot that detects potential arbitrage opportunities on the Polygon network by comparing token prices across multiple DEXes and provides a **live monitoring dashboard**.

---

## Overview

This bot periodically checks the prices of the WETH/USDC token pair on two DEXes (e.g., QuickSwap and SushiSwap) on Polygon. When a significant price difference is detected that exceeds the minimum profit threshold, it logs the potential arbitrage opportunity to a CSV file and updates a live web dashboard with simulated profit calculations.

It is **read-only** by design—no real trades are executed.

**Live Dashboard**
  Connected
  <img width="1918" height="862" alt="image" src="https://github.com/user-attachments/assets/30dd7520-de05-4479-a1e4-f40fd12fb4c2" />
  
  Not Connected
  <img width="1918" height="863" alt="image" src="https://github.com/user-attachments/assets/4d96ef76-b06b-41e0-8ae6-f8a75094f6ce" />
  <img width="1918" height="867" alt="image" src="https://github.com/user-attachments/assets/266a056c-f782-440e-8e6c-8fdbbb2e5ce6" />



---

## Features

* **Multi-DEX Price Fetching**: Queries current prices on multiple Polygon DEXes via their router contracts
* **Arbitrage Detection**: Identifies profitable opportunities by simulating trades in both directions
* **Profit Calculation**: Estimates net profit in USDC after considering a fixed gas cost
* **CSV Logging**: Stores all detected opportunities with timestamps for analysis
* **Live Web Dashboard**: Displays current opportunities, total profits, success rate, and more in real-time
* **Configurable**: Adjust all parameters via environment variables
* **Continuous Monitoring**: Runs indefinitely with configurable polling intervals
* **Graceful Shutdown**: Stops safely on `Ctrl+C`, ensuring all logs are flushed

---

## Technology Stack

* **Blockchain**: Polygon Network
* **Language**: Rust
* **DEX Integration**: Uniswap V2 Router ABI (QuickSwap, SushiSwap, etc.)
* **Tokens**: WETH, USDC
* **Libraries**: ethers-rs, tokio, csv, anyhow, rust\_decimal, axum (for web dashboard)

---

## Setup Instructions

1. **Clone the repository**

```bash
git clone <your-repo-url>
cd polygon-arb-bot
```

2. **Install Rust** (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. **Configure environment variables**
   Copy `.env.example` to `.env` and edit the values:

```bash
cp .env.example .env
```

Example `.env` configuration:

```env
# Polygon RPC (public or provider like Alchemy/QuickNode)
POLYGON_RPC=https://polygon-rpc.com

# DEX Router addresses
DEX1_ROUTER=0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff  # QuickSwap
DEX2_ROUTER=0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506  # SushiSwap

# Tokens
WETH=0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619
USDC=0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174

# Trading parameters
TRADE_AMOUNT_USDC=1000
MIN_PROFIT_USDC=5
POLL_INTERVAL_SECS=10
CSV_LOG=arb_log.csv
```

4. **Build and run the bot**

```bash
cargo build --release
cargo run --release
```

* Web dashboard will be available at [http://localhost:3000](http://localhost:3000)
* Profitable opportunities will be appended to the specified CSV file

---

## Architecture

### Core Components

| File             | Responsibility                                                |
| ---------------- | ------------------------------------------------------------- |
| `config.rs`      | Load configuration from environment variables                 |
| `dex.rs`         | Interact with DEX routers using `getAmountsOut`               |
| `arbitrage.rs`   | Contains core arbitrage detection and profit simulation logic |
| `web_monitor.rs` | Runs the live web dashboard (Axum-based)                      |
| `main.rs`        | Orchestrates polling, logging, and dashboard updates          |

### How It Works

1. Fetches current prices from both DEX routers
2. Simulates trades in both directions:

   * **DEX1 → DEX2**: Buy WETH on DEX1, sell on DEX2
   * **DEX2 → DEX1**: Buy WETH on DEX2, sell on DEX1
3. Calculates potential profit:


4. Logs profitable trades to CSV and updates live dashboard

---

## Configuration Parameters

| Parameter            | Description                 | Default      |
| -------------------- | --------------------------- | ------------ |
| `POLYGON_RPC`        | Polygon RPC endpoint        | Required     |
| `DEX1_ROUTER`        | First DEX router address    | Required     |
| `DEX2_ROUTER`        | Second DEX router address   | Required     |
| `WETH`               | WETH token address          | Required     |
| `USDC`               | USDC token address          | Required     |
| `TRADE_AMOUNT_USDC`  | Trade amount in USDC        | 1000         |
| `MIN_PROFIT_USDC`    | Minimum profit to log       | 5            |
| `POLL_INTERVAL_SECS` | Polling interval in seconds | 10           |
| `CSV_LOG`            | Output CSV filename         | arb\_log.csv |

---

## Output Format

CSV file structure:

```csv
timestamp,direction,profit_usdc
1695200000,DEX1->DEX2,12.34
1695200010,DEX2->DEX1,6.78
```

* `timestamp`: UNIX timestamp of detection
* `direction`: Arbitrage direction (DEX1→DEX2 or DEX2→DEX1)
* `profit_usdc`: Estimated profit in USDC

---

## Security Considerations

* Read-only contract calls (`getAmountsOut`) only
* No private keys or transactions are used
* Safe for testing, analysis, and educational purposes

---

## Future Enhancements

* Dynamic gas cost estimation using live gas fees
* Support for multiple token pairs
* Integration with additional DEXes and liquidity pools
* Real-time trade simulation with historical analytics
* Improved dashboard with charts and statistics

---

## License

MIT License © 2025 Pranav Vishwakarma

---

## Author

Created for Alfred Capital – Next Interview Round
