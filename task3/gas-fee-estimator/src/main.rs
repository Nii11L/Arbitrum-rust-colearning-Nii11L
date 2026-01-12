mod gas;

use gas::{get_basic_transfer_gas_limit, get_gas_price_info, estimate_gas_fee};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=========================================================");
    println!("         Arbitrum Sepolia Gas Fee Estimator");
    println!("=========================================================\n");

    // Get and display current gas price
    println!("📊 Fetching current gas price from Arbitrum Sepolia...\n");
    let gas_price_info = get_gas_price_info().await?;
    println!("{}\n", gas_price_info);

    // Get gas limit for basic transfer
    let gas_limit = get_basic_transfer_gas_limit();
    println!("⛽ Gas Limit for Basic Transfer: {} units\n", gas_limit);

    // Estimate gas fee for basic transfer
    println!("💰 Estimating gas fee for basic ETH transfer...\n");
    match estimate_gas_fee(gas_limit).await {
        Ok((gas_fee, formatted)) => {
            println!("---------------------------------------------------------");
            println!("Gas Fee Estimation Results:");
            println!("---------------------------------------------------------");
            println!("{}", formatted);
            println!("---------------------------------------------------------");
            println!("Raw Gas Fee: {} wei\n", gas_fee);
        }
        Err(e) => {
            eprintln!("Error estimating gas fee: {}", e);
            return Err(e);
        }
    }

    // Display formula explanation
    println!("📝 Gas Fee Calculation Formula:");
    println!("   Gas Fee = Gas Price × Gas Limit");
    println!("   Where:");
    println!("   - Gas Price: The cost per unit of gas (in wei)");
    println!("   - Gas Limit: Maximum amount of gas units for the transaction");
    println!("   - Basic ETH transfer typically requires 21,000 gas units\n");

    println!("✨ Gas fee estimation completed successfully!");
    println!("🎉 Task completed!\n");

    Ok(())
}
