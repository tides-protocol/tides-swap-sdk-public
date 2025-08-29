# Cross-Language Example: TypeScript Frontend + Rust Backend

This example shows how to use both SDKs together in a typical frontend-backend architecture where:

- **Rust backend** handles quote requests and returns encoded quotes
- **TypeScript frontend** decodes quotes and builds transactions

## Backend (Rust)

```rust
use tides_sui_swap_sdk::TidesClient;
use sui_sdk_types::TypeTag;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use axum::{extract::Query, response::Json, routing::get, Router};

#[derive(Deserialize)]
struct QuoteRequest {
    input_type: String,
    input_amount: u64,
    output_type: String,
    min_output_amount: Option<u64>,
    quote_expiry_ms: Option<u64>,
    rfq_timeout_ms: Option<u64>,
}

#[derive(Serialize)]
struct QuoteResponse {
    quote_data: Option<Vec<u8>>, // Encoded quote bytes
    error: Option<String>,
    // Note: input/output types not needed since frontend already knows what it requested
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/quote", get(get_quote_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_quote_handler(Query(params): Query<QuoteRequest>) -> Json<QuoteResponse> {
    // Connect to Tides hub, NOTE: the client should ideally be long-lived.
    let mut client = match TidesClient::connect("https://hub.tides.xyz").await {
        Ok(client) => client,
        Err(e) => return Json(QuoteResponse {
            quote_data: None,
            error: Some(format!("Failed to connect: {}", e)),
        }),
    };

    // Parse token types
    let input_type = match TypeTag::from_str(&params.input_type) {
        Ok(t) => t,
        Err(e) => return Json(QuoteResponse {
            quote_data: None,
            error: Some(format!("Invalid input type: {}", e)),
        }),
    };

    let output_type = match TypeTag::from_str(&params.output_type) {
        Ok(t) => t,
        Err(e) => return Json(QuoteResponse {
            quote_data: None,
            error: Some(format!("Invalid output type: {}", e)),
        }),
    };

    // Get quote from Tides
    match client.quote_exact_in(
        input_type,
        params.input_amount,
        output_type,
        params.min_output_amount,
        params.quote_expiry_ms,
        params.rfq_timeout_ms,
    ).await {
        Ok(Some(quote)) => {
            // Encode quote for wire transmission
            match client.encode_quote_to_bytes(&quote) {
                Ok(encoded_quote) => Json(QuoteResponse {
                    quote_data: Some(encoded_quote),
                    error: None,
                }),
                Err(e) => Json(QuoteResponse {
                    quote_data: None,
                    error: Some(format!("Failed to encode quote: {}", e)),
                }),
            }
        },
        Ok(None) => Json(QuoteResponse {
            quote_data: None,
            error: None, // No quote is healthy, not an error
        }),
        Err(e) => Json(QuoteResponse {
            quote_data: None,
            error: Some(format!("Failed to get quote: {}", e)),
        }),
    }
}
```

## Frontend (TypeScript)

```typescript
import { TidesClient } from "@tides-protocol/sui-swap-sdk";
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";
import { Transaction, coinWithBalance } from "@mysten/sui/transactions";

interface QuoteResponse {
  quote_data?: number[]; // Encoded quote bytes as array
  error?: string;
}

async function executeSwapFromBackend() {
  // Create Tides client for decoding
  const tidesClient = new TidesClient("https://hub.tides.xyz");

  const inputAmount = 1_000_000; // 1 USDC
  const inputType = "0xa::usdc::USDC";
  const outputType = "0x2::sui::SUI";

  try {
    // 1. Request quote from Rust backend
    const response = await fetch(
      "http://localhost:3000/quote?" +
        new URLSearchParams({
          input_type: inputType,
          input_amount: inputAmount.toString(),
          output_type: outputType,
          min_output_amount: "0", // Accept any amount; optional
        })
    );

    const quoteResponse: QuoteResponse = await response.json();

    if (quoteResponse.error) {
      console.error("Backend error:", quoteResponse.error);
      return;
    }

    if (!quoteResponse.quote_data) {
      console.log("No quote available from market makers");
      return;
    }

    // 2. Decode quote from backend response
    const encodedQuote = new Uint8Array(quoteResponse.quote_data);
    const quote = tidesClient.decodeQuoteFromBytes(
      inputType,
      outputType,
      encodedQuote
    );

    if (!quote) {
      console.error("Failed to decode quote from backend");
      return;
    }

    console.log("Decoded quote from backend:", quote.getSummary());
    console.log("Expected output:", quote.getOutputAmount());

    // 3. Build and execute transaction
    const suiClient = new SuiClient({ url: getFullnodeUrl("testnet") });
    const keypair = Ed25519Keypair.generate(); // Replace with your keypair
    const address = keypair.getPublicKey().toSuiAddress();

    const tx = new Transaction();
    tx.setSender(address);

    // Create input coin (USDC to swap)
    const inputCoin = coinWithBalance({
      balance: inputAmount,
      type: inputType,
    })(tx);

    // Create coin for Pyth oracle fees (paid in SUI)
    const pythFeeCoin = coinWithBalance({
      balance: quote.getPythPriceFees(),
    })(tx);

    // Apply swap to transaction
    quote.applySwapToTxAndTransferCoin(tx, {
      inputCoin,
      pythFeeCoin,
      recipient: address,
    });

    // Execute transaction
    const result = await suiClient.signAndExecuteTransaction({
      signer: keypair,
      transaction: tx,
      options: { showEffects: true },
    });

    console.log("Swap completed:", result.digest);
  } catch (error) {
    console.error("Frontend error:", error);
  }
}

// Run the example
executeSwapFromBackend();
```

## Key Points

### Encoding/Decoding

- **Rust**: `encode_quote_to_bytes()` → `decode_quote_from_bytes()`
- **TypeScript**: `encodeQuoteToBytes()` → `decodeQuoteFromBytes()`
- Both use the same underlying protobuf format for compatibility

### Architecture Benefits

- **Backend**: Ability to setup custom middlewares, such as connection pooling, quote caching and rate limiting
- **Frontend**: Lightweight, focuses on UI and transaction building

### Error Handling

- Backend validates inputs and returns structured errors
- Frontend handles both network and decoding errors
- Quote expiry is preserved across encoding/decoding

This pattern allows you to centralize quote management while keeping transaction building on the client side for security.
