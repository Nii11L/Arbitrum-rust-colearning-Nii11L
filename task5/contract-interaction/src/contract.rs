use ethers::prelude::*;
use std::env;
use std::fs;
use std::str::FromStr;

/// ERC20 Token Contract interface
///
/// This module provides functions to interact with ERC20 tokens
/// on the Arbitrum Sepolia testnet using ethers-rs.

/// Load ERC20 ABI from file
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - ABI JSON string
pub fn load_erc20_abi() -> Result<String, Box<dyn std::error::Error>> {
    let abi_path = "abi.json";
    let abi_json = fs::read_to_string(abi_path)
        .map_err(|_| format!("Failed to read ABI file: {}", abi_path))?;
    Ok(abi_json)
}

/// Get RPC URL from environment variable or use default
///
/// # Returns
/// * `String` - RPC URL
pub fn get_rpc_url() -> String {
    env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string())
}

/// Get contract address from environment variable or use default
///
/// # Returns
/// * `Result<H160, Box<dyn std::error::Error>>` - Contract address
pub fn get_contract_address() -> Result<H160, Box<dyn std::error::Error>> {
    let address_str = env::var("CONTRACT_ADDRESS")
        .unwrap_or_else(|_| "0x812dd1c3eb07bb1f5f93540350ef9af838ab0528".to_string());
    let address = H160::from_str(&address_str)
        .map_err(|_| format!("Invalid contract address: {}", address_str))?;
    Ok(address)
}

/// Get contract instance
///
/// # Returns
/// * Contract instance and address
pub async fn get_contract() -> Result<(Contract<Provider<Http>>, H160), Box<dyn std::error::Error>> {
    let rpc_url = get_rpc_url();
    let contract_address = get_contract_address()?;

    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Load ABI
    let abi_json = load_erc20_abi()?;

    // Parse ABI
    let abi: ethers::abi::Abi = serde_json::from_str(&abi_json)
        .map_err(|_| "Failed to parse ABI JSON".to_string())?;

    // Create contract instance
    let contract = Contract::new(contract_address, abi, provider.into());

    Ok((contract, contract_address))
}

/// Query ERC20 token name
///
/// # Arguments
/// * `contract` - The contract instance
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Token name
pub async fn get_token_name(contract: &Contract<Provider<Http>>) -> Result<String, Box<dyn std::error::Error>> {
    let name: String = contract
        .method("name", ())?
        .call()
        .await?;
    Ok(name)
}

/// Query ERC20 token symbol
///
/// # Arguments
/// * `contract` - The contract instance
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Token symbol
pub async fn get_token_symbol(contract: &Contract<Provider<Http>>) -> Result<String, Box<dyn std::error::Error>> {
    let symbol: String = contract
        .method("symbol", ())?
        .call()
        .await?;
    Ok(symbol)
}

/// Query ERC20 token decimals
///
/// # Arguments
/// * `contract` - The contract instance
///
/// # Returns
/// * `Result<u8, Box<dyn std::error::Error>>` - Token decimals
pub async fn get_token_decimals(contract: &Contract<Provider<Http>>) -> Result<u8, Box<dyn std::error::Error>> {
    let decimals: U256 = contract
        .method("decimals", ())?
        .call()
        .await?;
    Ok(decimals.as_u32() as u8)
}

/// Query ERC20 token total supply
///
/// # Arguments
/// * `contract` - The contract instance
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Total supply formatted with decimals
pub async fn get_total_supply(contract: &Contract<Provider<Http>>) -> Result<String, Box<dyn std::error::Error>> {
    let total_supply: U256 = contract
        .method("totalSupply", ())?
        .call()
        .await?;

    let decimals = get_token_decimals(contract).await?;

    // Calculate the divisor for decimals
    let divisor = U256::from(10).pow(U256::from(decimals));
    let whole_part = total_supply / divisor;
    let fractional_part = total_supply % divisor;

    Ok(format!("{}.{:0width$}",
        whole_part,
        fractional_part.as_u128(),
        width = decimals as usize
    ))
}

/// Query ERC20 token balance for an address
///
/// # Arguments
/// * `contract` - The contract instance
/// * `address` - The address to query balance for
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Balance formatted with token decimals
pub async fn get_balance(
    contract: &Contract<Provider<Http>>,
    address: H160,
) -> Result<String, Box<dyn std::error::Error>> {
    let balance: U256 = contract
        .method("balanceOf", address)?
        .call()
        .await?;

    let decimals = get_token_decimals(contract).await?;

    // Manually format balance with decimals
    let divisor = U256::from(10).pow(U256::from(decimals));
    let whole_part = balance / divisor;
    let fractional_part = balance % divisor;

    Ok(format!("{}.{:0width$}",
        whole_part,
        fractional_part.as_u128(),
        width = decimals as usize
    ))
}

/// Display token information
///
/// # Arguments
/// * `contract` - The contract instance
/// * `contract_address` - The contract address
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Formatted token information
pub async fn display_token_info(
    contract: &Contract<Provider<Http>>,
    contract_address: H160,
) -> Result<String, Box<dyn std::error::Error>> {
    let name = get_token_name(contract).await?;
    let symbol = get_token_symbol(contract).await?;
    let decimals = get_token_decimals(contract).await?;
    let total_supply = get_total_supply(contract).await?;

    let info = format!(
        "=========================================================\n\
         Token Information:\n\
         =========================================================\n\
         Contract Address: {}\n\
         Name:              {}\n\
         Symbol:            {}\n\
         Decimals:          {}\n\
         Total Supply:      {} {}\n\
         =========================================================",
        format!("{:#x}", contract_address),
        name,
        symbol,
        decimals,
        total_supply,
        symbol
    );

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_erc20_abi() {
        let abi = load_erc20_abi();
        assert!(abi.is_ok(), "Should load ABI file");
    }

    #[test]
    fn test_get_rpc_url() {
        let url = get_rpc_url();
        assert!(!url.is_empty(), "RPC URL should not be empty");
    }

    #[test]
    fn test_get_contract_address() {
        let address = get_contract_address();
        assert!(address.is_ok(), "Should get valid contract address");
    }
}
