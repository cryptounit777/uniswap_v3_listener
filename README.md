# Ethereum Transaction Tracker

This is a small excerpt from an Ethereum transaction monitoring tool written in Rust, which was developed during an educational cycle. It tracks and analyzes transactions related to a specific smart contract.

## Features

- 🔍 Real-time transaction monitoring
- 🎯 Transaction filtering by contract address
- 💰 Tracking ETH and ERC20 token transfers
- 📊 Transaction sorting by amount
- 🔍 Detailed transaction information

## Prerequisites

- Rust (latest stable version)
- Docker (optional)
- Infura API key

## Installation

1. Clone the repository:

```bash
git clone https://github.com/cryptounit777/uniswap_v3_listener.git
cd ethereum-transaction-tracker
```

2. Set up environment variables:

```bash
export TARGET_CONTRACT_ADDRESS=0x...  # Target contract address
```

## Usage

### Local Run

```bash
cargo run
```

### Docker Run

```bash
docker-compose up
```

## Functionality

### Transaction Monitoring

The program connects to the Ethereum network via Infura WebSocket and monitors all incoming transactions in real-time. For each transaction, it performs:

1. Verification against target contract address
2. Collection of detailed transaction information
3. Token transfer analysis (if it's an ERC20 transaction)

### Transaction Information

For each monitored transaction, the following information is displayed:

- Basic Information:
  - Transaction hash
  - From address
  - To address
  - Amount in ETH
  
- Technical Details:
  - Gas price (Gwei)
  - Gas used
  - Nonce
  - Block number
  - Transaction index
  - Chain ID

- Token Information (for ERC20):
  - Token address
  - Transfer amount

### Error Handling

The program includes typed error handling for the following cases:

```rust
#[derive(Error, Debug)]
enum TrackerError {
    WebSocketConnection(WsClientError),
    TransactionRetrieval(ProviderError),
    TransactionParsing(String),
}
```

## Code Structure

Main components:

- `main()`: Entry point and main monitoring loop
- `print_transaction_info()`: Formatted transaction information output
- `extract_token_info()`: Token transfer information extraction
- Error handling through `TrackerError`

## Limitations

- Program stops after collecting 5 transactions (for demonstration purposes)
- Only standard ERC20 token transfer function is supported
