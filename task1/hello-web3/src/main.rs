use ethers::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("🚀 Hello Web3 - Arbitrum Sepolia Connection Program\n");

    // Get RPC URL from environment variable, or use default Arbitrum Sepolia public RPC
    let rpc_url = env::var("ARBITRUM_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

    println!("📡 Connecting to Arbitrum Sepolia testnet...");
    println!("   RPC URL: {}\n", rpc_url);

    // Connect to Ethereum node (Arbitrum Sepolia)
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Get chain ID
    println!("⛓️  Getting chain information...");
    let chain_id = provider.get_chainid().await?;

    // Arbitrum Sepolia chain ID is 421614
    println!("   Chain ID: {}", chain_id);
    if chain_id.as_u64() == 421614 {
        println!("   ✅ Successfully connected to Arbitrum Sepolia testnet!\n");
    } else {
        println!("   ⚠️  Warning: Chain ID is not Arbitrum Sepolia (421614)\n");
    }

    // Get latest block number
    println!("📦 Getting latest block information...");
    let block_number = provider.get_block_number().await?;
    println!("   Latest block number: #{}\n", block_number);

    // Get network version
    println!("🌐 Getting network information...");
    let net_version = provider.get_net_version().await?;
    println!("   Network version: {}\n", net_version);

    // Get block details (optional)
    println!("📊 Getting latest block details...");
    if let Some(block) = provider.get_block(block_number).await? {
        println!("   Block hash: {:?}", block.hash.unwrap());
        println!("   Timestamp: {}", block.timestamp);
        println!("   Transaction count: {}\n", block.transactions.len());
    }

    println!("✨ Hello Web3! Successfully connected to Arbitrum Sepolia testnet!");
    println!("🎉 Task completed!\n");

    Ok(())
}
