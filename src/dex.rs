use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;

// Abigen for UniswapV2-style router (getAmountsOut)
abigen!(
    UniswapV2Router,
    r#"[
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts)
    ]"#
);


pub async fn get_amounts_out(
    provider: Arc<Provider<Http>>,
    router_addr: Address,
    amount_in: U256,
    path: Vec<Address>,
) -> Result<Vec<U256>> {
    let router = UniswapV2Router::new(router_addr, provider);
    // call getAmountsOut(amount_in, path)
    let amounts: Vec<U256> = router
        .get_amounts_out(amount_in, path)
        .call()
        .await?;
    Ok(amounts)
}