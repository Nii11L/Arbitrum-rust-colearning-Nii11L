mod transfer;

use transfer::{
    validate_address,
    get_balance,
    transfer_eth,
    estimate_transfer_fee,
    get_transaction_receipt,
};

use ethers::types::U64;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=========================================================");
    println!("         Arbitrum Sepolia ETH Transfer");
    println!("=========================================================\n");

    // Define addresses
    let sender_addr_str = "0xd78677EFed3b87f8f421E68dA3F984ad8Ef76439";
    let receiver_addr_str = "0x7292dD72151DaCFBbE76305db1C8Ab1928E922E4";

    println!("📋 Transaction Details:");
    println!("   From: {}", sender_addr_str);
    println!("   To:   {}", receiver_addr_str);
    println!("   Network: Arbitrum Sepolia (Chain ID: 421614)\n");

    // Validate addresses
    println!("🔍 Validating addresses...");
    let sender_address = validate_address(sender_addr_str)?;
    let receiver_address = validate_address(receiver_addr_str)?;
    println!("   ✅ Both addresses are valid\n");

    // Check sender balance
    println!("💰 Checking sender balance...");
    let sender_balance = get_balance(sender_address).await?;
    println!("   Sender Balance: {} ETH\n", sender_balance);

    // Parse balance to check if sufficient
    let sender_balance_float: f64 = sender_balance.parse()
        .map_err(|_| "Failed to parse balance")?;

    // Amount to transfer
    let transfer_amount = "0.0001"; // 0.0001 ETH

    // Check if balance is sufficient
    if sender_balance_float < 0.001 {
        println!("⚠️  Warning: Sender balance is very low ({} ETH)", sender_balance);
        println!("   You may need more ETH to cover gas fees.\n");
    }

    println!("💸 Transfer Amount: {} ETH\n", transfer_amount);

    // Estimate gas fee
    println!("⛽ Estimating gas fee...");
    let gas_info = estimate_transfer_fee().await?;
    println!("   {}", gas_info);
    println!();

    // Confirm before proceeding
    println!("⚠️  Ready to send transaction:");
    println!("   From:    {}", sender_addr_str);
    println!("   To:      {}", receiver_addr_str);
    println!("   Amount:  {} ETH", transfer_amount);
    println!();
    println!("   Press Ctrl+C to cancel, or wait 3 seconds to proceed...");
    thread::sleep(Duration::from_secs(3));
    println!();

    // Execute transfer
    println!("🚀 Executing transfer...\n");
    match transfer_eth(sender_address, receiver_address, transfer_amount).await {
        Ok(tx_hash) => {
            println!("---------------------------------------------------------");
            println!("✅ Transaction Submitted Successfully!");
            println!("---------------------------------------------------------");
            println!("Transaction Hash: {}", tx_hash);
            println!("---------------------------------------------------------");
            println!();
            println!("🔗 View on Arbitrum Sepolia Explorer:");
            println!("   https://sepolia.arbiscan.io/tx/{}", tx_hash);
            println!();

            // Wait for transaction confirmation
            println!("⏳ Waiting for transaction confirmation (up to 60 seconds)...");
            let mut retries = 0;
            let max_retries = 12; // 12 * 5 = 60 seconds

            while retries < max_retries {
                thread::sleep(Duration::from_secs(5));

                match get_transaction_receipt(&tx_hash).await {
                    Ok(Some(receipt)) => {
                        println!();
                        println!("---------------------------------------------------------");
                        println!("✅ Transaction Confirmed!");
                        println!("---------------------------------------------------------");
                        println!("Block Number: {}", receipt.block_number.unwrap_or_default());
                        println!("Gas Used: {}", receipt.gas_used.unwrap_or_default());
                        println!("Status: {}", if receipt.status.unwrap_or_default() == U64::from(1) {
                            "Success ✓"
                        } else {
                            "Failed ✗"
                        });
                        println!("---------------------------------------------------------");
                        println!();

                        // Check final balances
                        println!("📊 Final Balances:");
                        let new_sender_balance = get_balance(sender_address).await?;
                        let new_receiver_balance = get_balance(receiver_address).await?;
                        println!("   Sender:   {} ETH", new_sender_balance);
                        println!("   Receiver: {} ETH", new_receiver_balance);
                        println!();

                        println!("🎉 Transfer completed successfully!");
                        break;
                    }
                    Ok(None) => {
                        retries += 1;
                        print!(".");
                        // Flush stdout to show the dots
                        use std::io::Write;
                        std::io::stdout().flush().ok();
                    }
                    Err(e) => {
                        println!();
                        println!("⚠️  Error checking receipt: {}", e);
                        println!("   Transaction may still be pending. Check the explorer link above.");
                        break;
                    }
                }
            }

            if retries >= max_retries {
                println!();
                println!("⏱️  Transaction not confirmed within 60 seconds.");
                println!("   It may still be processing. Check the explorer link above.");
            }
        }
        Err(e) => {
            eprintln!("❌ Transaction failed: {}", e);
            println!();
            println!("Troubleshooting tips:");
            println!("1. Check that PRIVATE_KEY is correctly set in .env file");
            println!("2. Ensure the sender address has sufficient ETH balance");
            println!("3. Verify the RPC endpoint is accessible");
            println!("4. Check that you're using the correct private key for the sender address");
            return Err(e);
        }
    }

    Ok(())
}
