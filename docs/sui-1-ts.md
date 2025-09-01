# Tides Swap SDK (TypeScript)

A TypeScript SDK for interacting with the Tides Protocol on Sui blockchain to execute token swaps and trades.

## Overview

The Tides Swap SDK provides a high-level interface for:

- Getting RFQ (Request for Quote) trading quotes from the Market Makers of the Tides Protocol
- Executing token swaps with atomicity and output guarantees
- Building Sui transactions with proper price feed updates
- Managing minimum take amounts and validity periods
- Encoding and decoding quotes for wire transmission and persistence

## Installation

```bash
npm install @mysten/sui @tides-protocol/sui-swap-sdk
```

## Quick Start

### 1. Basic Token Swap (Exact Input)

```typescript
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";
import { Transaction, coinWithBalance } from "@mysten/sui/transactions";
import { TidesClient } from "@tides-protocol/sui-swap-sdk";

async function basicSwapExample(): Promise<void> {
  // Connect to Tides hub
  const tidesClient = await TidesClient.connect("https://sui-hub-tip.tides.xyz");

  const inputUsdcAmount = 1_000_000n; // 1 USDC (6 decimals)

  try {
    // Get quote for swapping USDC to SUI
    const quote = await tidesClient.getSwapExactInQuote({
      inputType: "0xa::usdc::USDC",
      inputAmount: inputUsdcAmount,
      outputType: "0x2::sui::SUI",
      minOutputAmount: 0n, // Accept any amount of SUI; optional
      quoteExpiry: new Date(1767243600000), // Use custom quote expiration
      rfqTimeout: new Date(1767232540000), // Use custom deadline for rfq response
    });

    if (!quote) {
      console.log("No valid quote available");
      return;
    }

    console.log("Expected output:", quote.getOutputAmount());
    console.log("Quote:", quote.getSummary());

    // Setup Sui client and transaction
    const client = new SuiClient({ url: getFullnodeUrl("testnet") });
    const keypair = Ed25519Keypair.generate(); // Replace with your keypair
    const address = keypair.getPublicKey().toSuiAddress();

    const tx = new Transaction();
    tx.setSender(address);

    // Create input coin (USDC to swap)
    const inputCoin = coinWithBalance({
      balance: inputUsdcAmount,
      type: "0xa::usdc::USDC",
    })(tx);

    // Create coin for Pyth oracle fees (paid in SUI), it's 1 MIST or 10^-9 SUI.
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
    const result = await client.signAndExecuteTransaction({
      signer: keypair,
      transaction: tx,
      options: {
        showEffects: true,
        showObjectChanges: true,
      },
    });

    console.log("Swap completed:", result.digest);
  } catch (error) {
    console.error("Swap failed:", error);
  }
}
```

### 2. Exact Output Swap

Request a quote by specifying exactly how much output token you want to receive. The quote will calculate the required input amount.

```typescript
async function exactOutSwapExample(): Promise<void> {
  // Connect to Tides hub
  const tidesClient = await TidesClient.connect("https://sui-hub-tip.tides.xyz");

  const desiredSuiAmount = 1_000_000_000n; // 1 SUI (9 decimals)

  try {
    // Get quote for receiving exact amount of SUI
    const quote = await tidesClient.getSwapExactOutQuote({
      outputType: "0x2::sui::SUI",
      outputAmount: desiredSuiAmount,
      inputType: "0xa::usdc::USDC",
      maxInputAmount: 10_000_000n, // Max USDC willing to spend; optional
      quoteExpiry: undefined, // Use default quote expiration
      rfqTimeout: undefined, // Use default rfq deadline
    });

    if (!quote) {
      console.log("No valid quote available");
      return;
    }

    console.log("USDC input required:", quote.getInputAmount());

    // Setup Sui client and transaction
    const client = new SuiClient({ url: getFullnodeUrl("testnet") });
    const keypair = Ed25519Keypair.generate(); // Replace with your keypair
    const address = keypair.getPublicKey().toSuiAddress();

    const tx = new Transaction();
    tx.setSender(address);

    // Create input coin with calculated amount
    const inputCoin = coinWithBalance({
      balance: quote.getInputAmount(),
      type: "0xa::usdc::USDC",
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
    const result = await client.signAndExecuteTransaction({
      signer: keypair,
      transaction: tx,
      options: {
        showEffects: true,
        showObjectChanges: true,
      },
    });

    console.log("Swap completed:", result.digest);
  } catch (error) {
    console.error("Swap failed:", error);
  }
}
```

### 3. Partial Quote Execution

Execute only a portion of a quote. Useful for aggregators or when you want to limit position size. Quote can be partially consumed as long as it meets the minimum output floor.

```typescript
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";
import { Transaction, coinWithBalance } from "@mysten/sui/transactions";
import { TidesClient } from "@tides-protocol/sui-swap-sdk";

async function partialSwapExample(): Promise<void> {
  // Connect to Tides hub
  const tidesClient = await TidesClient.connect("https://sui-hub-tip.tides.xyz");

  const fullUsdcAmount = 1_000_000n; // 1 USDC (6 decimals)

  try {
    // Get quote for full amount
    const quote = await tidesClient.getSwapExactInQuote({
      inputType: "0xa::usdc::USDC",
      inputAmount: fullUsdcAmount,
      outputType: "0x2::sui::SUI",
      minOutputAmount: 0n,
    });

    if (!quote) {
      console.log("No valid quote available");
      return;
    }

    console.log("Quote:", quote.getSummary());
    console.log("Minimum output floor:", quote.getOutputFloor());
    console.log("Minimum input floor:", quote.getInputFloor());

    // Use only half of the quote
    const partialUsdcAmount = fullUsdcAmount / 2n;

    // Calculate output for partial amount
    const partialOutputAmount = quote.calculateOutputAmount(partialUsdcAmount);
    if (!partialOutputAmount) {
      console.log("Partial amount doesn't meet minimum floor");
      return;
    }

    console.log("Partial output:", partialOutputAmount);

    // Setup Sui client and transaction
    const client = new SuiClient({ url: getFullnodeUrl("testnet") });
    const keypair = Ed25519Keypair.generate(); // Replace with your keypair
    const address = keypair.getPublicKey().toSuiAddress();

    const tx = new Transaction();
    tx.setSender(address);

    // Create input coin with partial amount
    const inputCoin = coinWithBalance({
      balance: partialUsdcAmount,
      type: "0xa::usdc::USDC",
    })(tx);

    // Create coin for Pyth oracle fees (paid in SUI)
    const pythFeeCoin = coinWithBalance({
      balance: quote.getPythPriceFees(),
    })(tx);

    // Apply partial swap to transaction
    quote.applySwapToTxAndTransferCoin(tx, {
      inputCoin,
      pythFeeCoin,
      recipient: address,
    });

    // Execute transaction
    const result = await client.signAndExecuteTransaction({
      signer: keypair,
      transaction: tx,
      options: {
        showEffects: true,
        showObjectChanges: true,
      },
    });

    console.log("Partial swap completed:", result.digest);
  } catch (error) {
    console.error("Swap failed:", error);
  }
}
```

## Core Concepts

### Request for Quote (RFQ) Trading

Tides Protocol uses **RFQ-based trading** with time-limited quotes provided by Market Makers. Each quote has:

- A fixed exchange rate for the duration of the quote
- A validity period after which the quote expires
- Minimum and maximum take amounts that define trading bounds

### Input and Output Bounds

- **Input Amount**: Maximum amount that can be provided as input to the RFQ
- **Output Amount**: Maximum amount that can be taken from the RFQ (depends on initial input provided)

- **Input Floor**: Minimum input amount needed for an RFQ to be valid
- **Output Floor**: Minimum amount that can be taken from the RFQ (minimum take amount), or how you'd get by profiving input floor.

- **Expiry Time**: Unix timestamp after which the RFQ becomes invalid

```typescript
// Check RFQ validity and bounds
const minInput = quote.getInputFloor();
const minOutput = quote.getOutputFloor();
const expiry = quote.getExpiryTimeMs();
console.log(`RFQ expires at: ${expiry} (Unix ms)`);
console.log(`Minimum input needed: ${Number(minInput) / 1e9} SUI`);
console.log(`Minimum takeable amount: ${Number(minOutput) / 1e6} USDC`);

// Check if a custom input amount would work
const customInput = 500_000_000n; // 500 SUI
if (customInput >= minInput) {
  const output = quote.calculateOutputAmount(customInput);
  if (output !== null) {
    console.log(`500 SUI would yield ${Number(output) / 1e6} USDC`);
  }
} else {
  console.log(
    `500 SUI is below minimum input of ${Number(minInput) / 1e9} SUI`
  );
}

// Check if quote is still valid
if (quote.isValid()) {
  console.log("Quote is still valid");
} else {
  console.log("Quote has expired");
}
```

## Error Handling

The SDK uses standard TypeScript error handling with try-catch blocks. Common error scenarios:

- **Connection failures**: Hub service unreachable
- **No trade routes**: No available path between tokens
- **Invalid amounts**: Input/output amounts outside valid ranges
- **Transaction building**: Invalid arguments or object references
- **Quote expiration**: Quote expired before execution
- **RFQ timeout**: No Quote found before `rfqTimeout` elapsed

```typescript
try {
  const quote = await client.getSwapExactInQuote({
    inputType: "0x2::sui::SUI",
    inputAmount: 1000000000n,
    outputType: "0xa::usdc::USDC",
    minOutputAmount: 0n,
  });

  if (!quote) {
    console.log("No trade route available for this pair");
    return;
  }
} catch (error) {
  console.error("Error getting quote:", error);
}
```

## Wire Encoding & Quote Transmission

The SDK supports encoding quotes for wire transmission, allowing you to send quotes over a network.

### Example: Quote Transmission

```typescript
// Server-side: Send quote to client
async function sendQuoteToClient(
  client: TidesClient,
  websocket: WebSocket
): Promise<void> {
  const quote = await client.getSwapExactInQuote({
    inputType: "0x2::sui::SUI",
    inputAmount: 1_000_000_000n,
    outputType: "0xa::usdc::USDC",
  });

  if (quote) {
    const wireData = client.encodeQuoteToBytes(quote);
    websocket.send(
      JSON.stringify({
        type: "quote",
        data: Array.from(wireData),
        inputType: "0x2::sui::SUI",
        outputType: "0xa::usdc::USDC",
      })
    );
  }
}

// Client-side: Receive and reconstruct quote
function handleQuoteMessage(client: TidesClient, message: any): void {
  if (message.type === "quote") {
    const wireData = new Uint8Array(message.data);
    const quote = client.decodeQuoteFromBytes(
      message.inputType,
      message.outputType,
      wireData
    );

    if (quote && quote.isValid()) {
      console.log("Received valid quote:", quote.getSummary());
      // Process the quote
    }
  }
}
```

## License

This project is licensed under the Business Source License 1.1 - see the [LICENSE](../LICENSE) file for details.
