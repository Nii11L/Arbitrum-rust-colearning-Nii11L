mod balance;

use balance::query_balance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=========================================================");
    println!("         Arbitrum Sepolia Balance Query");
    println!("=========================================================\n");

    // The address to query
    let target_address = "0xd78677EFed3b87f8f421E68dA3F984ad8Ef76439";

    println!("Target Address: {}\n", target_address);

    // Query the balance
    match query_balance(target_address).await {
        Ok((balance, eth_value)) => {
            println!("---------------------------------------------------------");
            println!("Balance Query Results:");
            println!("---------------------------------------------------------");
            println!("  Raw Balance (wei): {} wei", balance);
            println!("  Formatted Balance: {} ETH", eth_value);
            println!("---------------------------------------------------------");
            println!("\nQuery completed successfully!");
        }
        Err(e) => {
            eprintln!("Error querying balance: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
