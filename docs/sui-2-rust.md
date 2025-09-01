# Tides Swap SDK (Rust)

A Rust SDK for interacting with the Tides Protocol on Sui blockchain to execute token swaps and trades.

## Overview

The Tides Swap SDK provides a high-level interface for:

- Getting RFQ (Request for Quote) trading quotes from the Market Makers of the Tides Protocol
- Executing token swaps with atomicity and output guarantees
- Building Sui transactions with proper price feed updates
- Managing minimum take amounts and validity periods

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tides-sui-swap-sdk = "0.1.0"
```

## Quick Start

### 1. Basic Token Swap (Exact Input)

```rust
use tides_sui_swap_sdk::TidesClient;
use sui_sdk_types::TypeTag;
use sui_transaction_builder::TransactionBuilder;
use std::str::FromStr;

#[tokio::main]
async fn basic_swap_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Tides hub
    let mut client = TidesClient::connect("https://sui-hub-tip.tides.xyz").await?;

    let input_usdc_amount = 1_000_000; // 1 USDC (6 decimals)

    // Get quote for swapping USDC to SUI
    let quote = client.quote_exact_in(
        TypeTag::from_str("0xa::usdc::USDC")?,
        input_usdc_amount,
        TypeTag::from_str("0x2::sui::SUI")?,
        None, // Accept any amount of SUI; optional
        Some(1767243600000), // Use custom quote expiration, in Unix ms
        Some(1767232540000), // Use custom deadline for rfq response
    ).await?;

    if let Some(quote) = quote {
        println!("Expected output: {}", quote.output_ceiling());

        // Setup transaction
        let mut tx = TransactionBuilder::new();

        // Create input coin (USDC to swap)
        let input_coin = make_coin_with_amount(quote.input(), quote.input_amount());

        // Create coin for Pyth oracle fees (paid in SUI), it's 1 MIST or 10^-9 SUI.
        let pyth_fee_coin = make_coin_with_amount(
            &TypeTag::from_str("0x2::sui::SUI")?,
            quote.pyth_price_fees()
        );

        // Apply swap to transaction
        quote.apply_swap_to_tx_and_transfer_coin(
            &mut tx,
            input_coin,
            pyth_fee_coin,
            recipient_address,
            None, // clock; optional
        )?;

        // Execute transaction
        // ... submit tx to Sui network
        println!("Swap completed!");
    } else {
        println!("No valid quote available");
    }

    Ok(())
}
```

### 2. Exact Output Swap

Request a quote by specifying exactly how much output token you want to receive. The quote will calculate the required input amount.

```rust
async fn exact_out_swap_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Tides hub
    let mut client = TidesClient::connect("https://sui-hub-tip.tides.xyz").await?;

    let desired_sui_amount = 1_000_000_000; // 1 SUI (9 decimals)

    // Get quote for receiving exact amount of SUI
    let quote = client.quote_exact_out(
        TypeTag::from_str("0x2::sui::SUI")?,
        desired_sui_amount,
        TypeTag::from_str("0xa::usdc::USDC")?,
        Some(10_000_000), // Max USDC willing to spend; optional
        None, // Use default quote expiration
        None, // Use default rfq deadline
    ).await?;

    if let Some(quote) = quote {
        println!("USDC input required: {}", quote.input_amount());

        // Setup transaction
        let mut tx = TransactionBuilder::new();

        // Create input coin with calculated amount
        let input_coin = make_coin_with_amount(quote.input(), quote.input_amount());

        // Create coin for Pyth oracle fees (paid in SUI)
        let pyth_fee_coin = make_coin_with_amount(
            &TypeTag::from_str("0x2::sui::SUI")?,
            quote.pyth_price_fees()
        );

        // Apply swap to transaction
        quote.apply_swap_to_tx_and_transfer_coin(
            &mut tx,
            input_coin,
            pyth_fee_coin,
            recipient_address,
            None, // clock; optional
        )?;

        // Execute transaction
        // ... submit tx to Sui network
        println!("Swap completed!");
    } else {
        println!("No valid quote available");
    }

    Ok(())
}
```

### 3. Partial Quote Execution

Execute only a portion of a quote. Useful for aggregators or when you want to limit position size. Quote can be partially consumed as long as it meets the minimum output floor.

```rust
async fn partial_swap_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Tides hub
    let mut client = TidesClient::connect("https://sui-hub-tip.tides.xyz").await?;

    let full_usdc_amount = 1_000_000; // 1 USDC (6 decimals)

    // Get quote for full amount
    let quote = client.quote_exact_in(
        TypeTag::from_str("0xa::usdc::USDC")?,
        full_usdc_amount,
        TypeTag::from_str("0x2::sui::SUI")?,
        None,
        Some(1767243600000),
        None,
    ).await?;

    if let Some(quote) = quote {
        println!("Minimum output floor: {}", quote.output_floor());
        println!("Minimum input floor: {}", quote.input_floor());

        // Use only half of the quote
        let partial_usdc_amount = full_usdc_amount / 2;

        // Calculate output for partial amount
        if let Some(partial_output_amount) = quote.calculate_output_amount(partial_usdc_amount) {
            println!("Partial output: {}", partial_output_amount);

            // Setup transaction
            let mut tx = TransactionBuilder::new();

            // Create input coin with partial amount
            let input_coin = make_coin_with_amount(quote.input(), partial_usdc_amount);

            // Create coin for Pyth oracle fees (paid in SUI)
            let pyth_fee_coin = make_coin_with_amount(
                &TypeTag::from_str("0x2::sui::SUI")?,
                quote.pyth_price_fees()
            );

            // Apply partial swap to transaction
            quote.apply_swap_to_tx_and_transfer_coin(
                &mut tx,
                input_coin,
                pyth_fee_coin,
                recipient_address,
                None, // clock; optional
            )?;

            // Execute transaction
            // ... submit tx to Sui network
            println!("Partial swap completed!");
        } else {
            println!("Partial amount doesn't meet minimum floor");
        }
    } else {
        println!("No valid quote available");
    }

    Ok(())
}
```

## Core Concepts

### Request for Quote (RFQ) Trading

Tides Protocol uses **RFQ-based trading** with time-limited quotes. Each quote has:

- A fixed exchange rate for the duration of the quote
- A validity period after which the quote expires
- Minimum and maximum take amounts that define trading bounds

### Input and Output Bounds

- **Input Amount**: Maximum amount that can be provided as input to the RFQ
- **Output Amount**: Maximum amount that can be taken from the RFQ (depends on initial input provided)

- **Input Floor**: Minimum input amount needed for an RFQ to be valid
- **Output Floor**: Minimum amount that can be taken from the RFQ (minimum take amount), or how you'd get by providing input floor.

- **Expiry Time**: Unix timestamp after which the RFQ becomes invalid

```rust
// Check RFQ validity and bounds
let min_input = quote.input_floor();
let min_output = quote.output_floor();
let expiry = quote.expiry_time_ms();
println!("RFQ expires at: {} (Unix ms)", expiry);
println!("Minimum input needed: {} SUI", min_input as f64 / 1e9);
println!("Minimum takeable amount: {} USDC", min_output as f64 / 1e6);

// Check if a custom input amount would work
let custom_input = 500_000_000; // 500 SUI
if custom_input >= min_input {
    if let Some(output) = quote.calculate_output_amount(custom_input) {
        println!("500 SUI would yield {} USDC", output as f64 / 1e6);
    }
} else {
    println!("500 SUI is below minimum input of {} SUI", min_input as f64 / 1e9);
}
```

## API Reference

### TidesClient

#### `connect(endpoint: &'static str) -> Result<Self>`

Creates a new client connected to the Tides Hub gRPC service.

#### `quote_exact_in(...) -> Result<Option<Quote>>`

Gets a quote for swapping an exact input amount to the output token.

**Parameters:**

- `input`: Input token type
- `input_amount`: Exact amount of input tokens to swap
- `output`: Output token type
- `min_output_amount`: Optional minimum output amount (slippage protection)
- `recipient`: Address to receive the output

#### `quote_exact_out(...) -> Result<Option<Quote>>`

Gets a quote for receiving an exact output amount from the input token.

**Parameters:**

- `output`: Output token type
- `output_amount`: Exact amount of output tokens desired
- `input`: Input token type
- `max_input_amount`: Optional maximum input amount willing to spend
- `recipient`: Address to receive the output

### Quote

#### Transaction Building

##### `apply_swap_to_tx_and_transfer_coin(...) -> Result<()>`

Executes the swap and transfers output directly to recipient.

##### `apply_swap_to_tx(...) -> Result<Argument>`

Executes the swap and returns output balance for custom handling.

#### Quote Information

##### `input() -> &TypeTag` / `output() -> &TypeTag`

Returns the input/output token types.

##### `input_amount() -> u64`

The exact input amount this quote was calculated for.

##### `input_floor() -> u64`

The minimum input amount needed for this RFQ to be valid.

##### `output_floor() -> u64` / `output_ceiling() -> u64`

Minimum and maximum output amounts that can be taken from this RFQ.

##### `expiry_time_ms() -> u64`

The expiry time of this RFQ in Unix milliseconds. The quote becomes invalid after this timestamp.

##### `pyth_price_fees() -> u64`

Total SUI fees needed for Pyth price feed updates.

#### Calculations

##### `calculate_output_amount(input_amount: u64) -> Option<u64>`

Calculates output for a custom input amount using this RFQ's exchange rate. Returns `None` if input is outside valid range.

## Error Handling

The SDK uses `anyhow::Result` for error handling. Common error scenarios:

- **Connection failures**: Hub service unreachable
- **No trade routes**: No available path between tokens
- **Invalid amounts**: Input/output amounts outside valid ranges
- **Transaction building**: Invalid arguments or object references
- **Quote expiration**: Quote expired before execution
- **RFQ timeout**: No Quote found before `rfqTimeout` elapsed

```rust
match client.quote_exact_in(...).await {
    Ok(Some(quote)) => {
        // Process quote
    },
    Ok(None) => {
        println!("No trade route available for this pair");
    },
    Err(e) => {
        eprintln!("Error getting quote: {}", e);
    }
}
```

## Requirements

- Rust 1.70+
- Access to a Tides Hub gRPC service
- Sui SDK types for transaction building

## License

This project is licensed under the Business Source License 1.1 - see the [LICENSE](../LICENSE) file for details.
