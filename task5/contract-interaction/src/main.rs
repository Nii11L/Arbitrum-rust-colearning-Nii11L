mod contract;

use contract::{
    get_contract,
    display_token_info,
    get_balance,
};
use ethers::prelude::*;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=========================================================");
    println!("    Arbitrum Sepolia ERC20 Contract Interaction");
    println!("=========================================================\n");

    // Get the contract
    println!("📜 Connecting to ERC20 contract...\n");
    let (contract, contract_address) = get_contract().await?;

    // Display token information
    let token_info = display_token_info(&contract, contract_address).await?;
    println!("{}\n", token_info);

    // Query balance for a specific address if provided
    let query_address_str = env::var("QUERY_ADDRESS").ok();
    if let Some(address_str) = query_address_str {
        println!("💰 Querying balance for address: {}\n", address_str);

        match H160::from_str(&address_str) {
            Ok(address) => {
                match get_balance(&contract, address).await {
                    Ok(balance) => {
                        println!("---------------------------------------------------------");
                        println!("Balance Query Result:");
                        println!("---------------------------------------------------------");
                        println!("Address: {}", address_str);
                        println!("Balance: {} tokens", balance);
                        println!("---------------------------------------------------------\n");
                    }
                    Err(e) => {
                        println!("⚠️  Failed to query balance: {}\n", e);
                    }
                }
            }
            Err(_) => {
                println!("⚠️  Invalid address format: {}\n", address_str);
            }
        }
    } else {
        println!("💡 Tip: Set QUERY_ADDRESS in .env to check token balance for a specific address\n");
    }

    // Display additional examples
    println!("📚 What you can do with this contract:");
    println!("   ✓ Read token name");
    println!("   ✓ Read token symbol");
    println!("   ✓ Read token decimals");
    println!("   ✓ Read total supply");
    println!("   ✓ Query balance for any address");
    println!("   ✓ View contract on Arbiscan: https://sepolia.arbiscan.io/token/{:#x}", contract_address);
    println!();

    println!("✨ Contract interaction completed successfully!");
    println!("🎉 Task completed!\n");

    Ok(())
}
