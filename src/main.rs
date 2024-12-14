use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::types::{Address, Transaction, U256};
use ethers::utils::hex;
use futures_util::stream::StreamExt;
use thiserror::Error;
use std::env;
use tokio;

// Typed errors for improved diagnostics
#[derive(Error, Debug)]
enum TrackerError {
    #[error("WebSocket connection error: {0}")]
    WebSocketConnection(#[from] WsClientError),

    #[error("Transaction retrieval error: {0}")]
    TransactionRetrieval(#[from] ethers::providers::ProviderError),

    #[error("Transaction data parsing error: {0}")]
    TransactionParsing(String),
}

#[tokio::main]
async fn main() -> Result<(), TrackerError> {
    let infura_ws_url = "wss://mainnet.infura.io/ws/v3/";

    // Get the contract address from the environment variable
    let input = env::var("TARGET_CONTRACT_ADDRESS")
        .map_err(|_| TrackerError::TransactionParsing("Environment variable TARGET_CONTRACT_ADDRESS is not set".to_string()))?;

    // Parse the contract address from the environment variable
    let target_contract_address: Address = input
        .parse()
        .map_err(|_| TrackerError::TransactionParsing(format!("Invalid address format: {}", input)))?;

    // Create a WebSocket provider for connecting to Ethereum through Infura
    let ws = Ws::connect(infura_ws_url).await?;
    let provider = Provider::new(ws);

    // Subscribe to new transactions in the network
    let mut stream = provider.subscribe_pending_txs().await?;
    let mut transactions: Vec<Transaction> = Vec::new();

    println!("Waiting for new transactions...");
    while let Some(tx_hash) = stream.next().await {
        match provider.get_transaction(tx_hash).await {
            Ok(Some(tx)) => {
                // Filter transactions to track only those interacting with the specified contract
                if let Some(to_address) = tx.to {
                    // Output for debugging, indicating that the transaction is being analyzed
                    println!("Analyzing transaction with address: {:?}", to_address);

                    if to_address == target_contract_address {
                        transactions.push(tx);

                        // Example: stop after collecting 5 transactions for demonstration
                        if transactions.len() >= 5 {
                            break;
                        }
                    }
                }
            }
            Ok(None) => continue, // Transaction not found
            Err(e) => {
                eprintln!("Error retrieving transaction: {:?}", e);
                continue;
            }
        }
    }

    // Sort transactions by amount (`value`)
    transactions.sort_by(|a, b| a.value.cmp(&b.value));

    // Output sorted transactions
    println!(
        "Sorted transactions related to contract {}:",
        target_contract_address
    );
    for tx in &transactions {
        print_transaction_info(tx);
    }

    Ok(())
}

fn print_transaction_info(tx: &Transaction) {
    println!("==================== Transaction Details ====================");

    // Basic transaction information
    println!("Transaction Hash: {:?}", tx.hash);
    println!("From Address: {:?}", tx.from);
    if let Some(to) = tx.to {
        println!("To Address: {:?}", to);
    } else {
        println!("To Address: None (Contract Creation)");
    }
    println!("Value Transferred (ETH): {:?}", ethers::utils::format_units(tx.value, "ether").unwrap_or_else(|_| "N/A".to_string()));

    // Additional transaction details
    println!("Gas Price (Gwei): {:?}", ethers::utils::format_units(tx.gas_price.unwrap_or_default(), "gwei").unwrap_or_else(|_| "N/A".to_string()));
    println!("Gas Used: {:?}", tx.gas);
    println!("Nonce: {:?}", tx.nonce);

    // New additional information
    if let Some(block_number) = tx.block_number {
        println!("Block Number: {:?}", block_number);
    } else {
        println!("Block Number: Pending");
    }
    if let Some(transaction_index) = tx.transaction_index {
        println!("Transaction Index in Block: {:?}", transaction_index);
    } else {
        println!("Transaction Index: Pending");
    }
    if let Some(block_hash) = tx.block_hash {
        println!("Block Hash: {:?}", block_hash);
    } else {
        println!("Block Hash: Pending");
    }
    println!("Chain ID: {:?}", tx.chain_id.unwrap_or_default());

    // Token transfer information
    match extract_token_info(tx) {
        Ok(Some(token_info)) => {
            println!("Token Transfer Detected:");
            println!("  Token Address: {}", token_info.token);
            println!("  Token Amount: {}", ethers::utils::format_units(token_info.amount, "ether").unwrap_or_else(|_| "N/A".to_string()));
        }
        Ok(None) => println!("Token Information: Not available"),
        Err(e) => eprintln!("Error parsing token data: {:?}", e),
    }

    println!("============================================================");
}

// Structure for storing token information
struct TokenInfo {
    token: String,
    amount: U256,
}

// Function to extract token information from a transaction (if applicable)
fn extract_token_info(tx: &Transaction) -> Result<Option<TokenInfo>, TrackerError> {
    // Check if this is a token interaction transaction (e.g., ERC20)
    // In this case, we check by the function signature `transfer` (0xa9059cbb)
    let data = &tx.input;
    {
        if data.0.starts_with(&[0xa9, 0x05, 0x9c, 0xbb]) && data.0.len() == 68 {
            // Extract the recipient address and token amount
            let token_address = hex::encode(&data.0[16..36]);
            let token_amount = U256::from_big_endian(&data.0[36..68]);

            // Return token information
            return Ok(Some(TokenInfo {
                token: format!("0x{}", token_address),
                amount: token_amount,
            }));
        }
    }
    Ok(None)
}
