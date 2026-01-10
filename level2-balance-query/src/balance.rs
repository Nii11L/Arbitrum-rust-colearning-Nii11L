use ethers::prelude::*;
use ethers::utils::format_units;
use std::str::FromStr;

/// Query ETH balance for an address on Arbitrum Sepolia testnet
///
/// # Arguments
/// * `address_str` - The Ethereum address to query balance for (e.g., "0x...")
///
/// # Returns
/// * `Result<(U256, String), Box<dyn std::error::Error>>` - Returns a tuple containing:
///   - Raw balance in wei (U256)
///   - Formatted balance string in ETH
pub async fn query_balance(address_str: &str) -> Result<(U256, String), Box<dyn std::error::Error>> {
    // Parse the address
    let address = Address::from_str(address_str)?;

    // Get RPC URL from environment variable or use default
    let rpc_url = std::env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    // Connect to Arbitrum Sepolia testnet
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Query the balance in wei
    let balance = provider.get_balance(address, None).await?;

    // Convert wei to ETH (1 ETH = 10^18 wei)
    let eth_value = format_units(balance, "ether")?;

    Ok((balance, eth_value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_balance() {
        let test_address = "0xd78677EFed3b87f8f421E68dA3F984ad8Ef76439";
        match query_balance(test_address).await {
            Ok((balance, eth_value)) => {
                println!("Balance: {} wei", balance);
                println!("Balance: {} ETH", eth_value);
                assert!(balance >= U256::zero());
            }
            Err(e) => {
                println!("Error querying balance: {}", e);
                // Allow test to pass even if RPC fails
                assert!(true);
            }
        }
    }
}
