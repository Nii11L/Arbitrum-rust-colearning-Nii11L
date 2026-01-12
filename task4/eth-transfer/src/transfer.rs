use ethers::prelude::*;
use ethers::utils::{format_units, parse_ether};
use std::env;
use std::str::FromStr;

/// Validate an Ethereum address format
///
/// # Arguments
/// * `address_str` - The address string to validate
///
/// # Returns
/// * `Result<H160, Box<dyn std::error::Error>>` - Valid address or error
pub fn validate_address(address_str: &str) -> Result<H160, Box<dyn std::error::Error>> {
    // Remove 0x prefix if present
    let clean_address = address_str.strip_prefix("0x").unwrap_or(address_str);

    // Check if address is valid length (40 hex chars + optional 0x = 42)
    if clean_address.len() != 40 {
        return Err(format!("Invalid address length: {}", address_str).into());
    }

    // Try to parse the address
    let address = H160::from_str(address_str)
        .map_err(|_| format!("Invalid address format: {}", address_str))?;

    Ok(address)
}

/// Get the current gas price from Arbitrum Sepolia testnet
///
/// # Returns
/// * `Result<U256, Box<dyn std::error::Error>>` - Current gas price in wei
pub async fn get_gas_price() -> Result<U256, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let gas_price = provider.get_gas_price().await?;

    Ok(gas_price)
}

/// Create a wallet from private key environment variable
///
/// # Returns
/// * `Result<LocalWallet, Box<dyn std::error::Error>>` - Wallet with private key
pub fn get_wallet() -> Result<LocalWallet, Box<dyn std::error::Error>> {
    let private_key_str = env::var("PRIVATE_KEY")
        .map_err(|_| "PRIVATE_KEY environment variable not set".to_string())?;

    // Remove 0x prefix if present
    let clean_key = private_key_str.strip_prefix("0x").unwrap_or(&private_key_str);

    // Parse private key
    let private_key = H256::from_str(clean_key)
        .map_err(|_| "Invalid private key format".to_string())?;

    // Create wallet (will use chain ID from provider later)
    let wallet = LocalWallet::from_bytes(&private_key.0)
        .map_err(|_| "Failed to create wallet from private key".to_string())?;

    Ok(wallet)
}

/// Get ETH balance of an address
///
/// # Arguments
/// * `address` - The address to check balance for
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Balance formatted in ETH
pub async fn get_balance(address: H160) -> Result<String, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let balance = provider.get_balance(address, None).await?;
    let balance_eth = format_units(balance, "ether")?;

    Ok(balance_eth)
}

/// Transfer ETH from sender to receiver on Arbitrum Sepolia
///
/// # Arguments
/// * `from_address` - Sender address
/// * `to_address` - Receiver address
/// * `amount_ether` - Amount to transfer in ETH (as string, e.g., "0.001")
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Transaction hash
pub async fn transfer_eth(
    from_address: H160,
    to_address: H160,
    amount_ether: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Get wallet and connect with chain ID
    let wallet = get_wallet()?;
    let chain_id = provider.get_chainid().await?;

    // Arbitrum Sepolia chain ID is 421614
    if chain_id.as_u64() != 421614 {
        return Err(format!("Not connected to Arbitrum Sepolia. Chain ID: {}", chain_id).into());
    }

    let wallet = wallet.with_chain_id(chain_id.as_u64());

    // Verify the wallet address matches the sender
    let wallet_address = wallet.address();
    if wallet_address != from_address {
        return Err(format!(
            "Wallet address {:?} does not match sender address {:?}",
            wallet_address, from_address
        )
        .into());
    }

    // Get current block to find base fee for EIP-1559
    let block = provider.get_block(BlockNumber::Latest).await?
        .ok_or("Failed to get block")?;

    // Get base fee from the block (Arbitrum Sepolia uses EIP-1559)
    let base_fee = block.base_fee_per_gas.unwrap_or_else(|| U256::from(10_000_000u64));

    // Calculate gas price: must be higher than base_fee
    // Add 20% buffer + 0.01 Gwei tip to ensure transaction is accepted
    let max_priority_fee = U256::from(10_000_000u64); // 0.01 Gwei tip
    let gas_price = base_fee + (base_fee / 5) + max_priority_fee; // base_fee + 20% + tip

    // Standard gas limit for ETH transfer
    let gas_limit = U256::from(100_000); // Use 100k to be safe on L2

    // Parse amount
    let amount_wei = parse_ether(amount_ether)?;

    // Build transaction with gas_price (will be converted to EIP-1559 by the network)
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount_wei)
        .from(from_address)
        .gas(gas_limit)
        .gas_price(gas_price);

    // Send transaction
    println!("Sending transaction...");
    println!("Base fee: {} Gwei", format_units(base_fee, "gwei")?);
    println!("Gas price: {} Gwei (base_fee + 20% + 0.01 Gwei)", format_units(gas_price, "gwei")?);
    println!("Gas limit: {}", gas_limit);
    let client = SignerMiddleware::new(provider.clone(), wallet);
    let pending_tx = client.send_transaction(tx, None).await?;

    // Get the transaction hash - in ethers v2, PendingTransaction implements Deref to TxHash
    let tx_hash = *pending_tx;

    Ok(format!("{:#x}", tx_hash))
}

/// Get transaction receipt by hash
///
/// # Arguments
/// * `tx_hash` - Transaction hash
///
/// # Returns
/// * `Result<Option<TransactionReceipt>, Box<dyn std::error::Error>>` - Transaction receipt
pub async fn get_transaction_receipt(
    tx_hash: &str,
) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Parse transaction hash
    let tx_hash = H256::from_str(tx_hash)?;

    let receipt = provider.get_transaction_receipt(tx_hash).await?;

    Ok(receipt)
}

/// Estimate gas fee for a transfer
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Formatted gas fee info
pub async fn estimate_transfer_fee() -> Result<String, Box<dyn std::error::Error>> {
    let gas_price = get_gas_price().await?;
    let gas_limit = U256::from(21_000);
    let gas_fee = gas_price * gas_limit;

    let gwei_value = format_units(gas_fee, "gwei")?;
    let eth_value = format_units(gas_fee, "ether")?;

    Ok(format!(
        "Gas Price: {} Gwei\nGas Limit: {} units\nEstimated Fee: {} Gwei ({} ETH)",
        format_units(gas_price, "gwei")?,
        gas_limit,
        gwei_value,
        eth_value
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_addresses() {
        let valid_addresses = vec![
            "0xd78677EFed3b87f8f421E68dA3F984ad8Ef76439",
            "0x7292dD72151DaCFBbE76305db1C8Ab1928E922E4",
            "d78677EFed3b87f8f421E68dA3F984ad8Ef76439", // without 0x prefix
        ];

        for addr in valid_addresses {
            assert!(validate_address(addr).is_ok(), "Should validate: {}", addr);
        }
    }

    #[test]
    fn test_validate_invalid_addresses() {
        let invalid_addresses = vec![
            "0x123", // too short
            "0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG", // invalid hex
            "not_an_address", // invalid format
        ];

        for addr in invalid_addresses {
            assert!(validate_address(addr).is_err(), "Should reject: {}", addr);
        }
    }

    #[test]
    fn test_parse_ether_amount() {
        // Test parsing valid amounts
        let amount1 = parse_ether("1.0").unwrap();
        let amount2 = parse_ether("0.001").unwrap();

        assert!(amount1 > U256::zero());
        assert!(amount2 > U256::zero());
        assert!(amount1 > amount2);
    }
}
