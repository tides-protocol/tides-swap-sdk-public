# Tides Swap SDK

**Integrate atomic RFQ trading into your app.**

SDKs for aggregators, wallets, and frontends to connect with Tides - a fully atomic RFQ protocol. Get competitive quotes from market makers and execute swaps directly on-chain.

## How integration works

1. **Your app requests quotes** using our SDK for any token pair
2. **Market makers compete** and return competitive prices
3. **Your users execute swaps** atomically on-chain

## Why integrate Tides RFQs

**Traditional RFQs** are async nightmares, that involve transactions from both the user and the MM to settle a single trade and can't be composed with other DeFi operations.

**Tides RFQs** execute atomically on-chain with expiration timestamps. For your users:

- **Guaranteed execution** at the quoted price
- **Composable** with other DeFi operations in the same transaction
- **No slippage** beyond what they agreed to
- **Better UX** than any other RFQ system

Better prices than AMMs, better UX than traditional RFQs. Your users win, you win.

## Get Started

- [**📚 SDK Overview**](./docs/sdk-overview.md) - Choose the right SDK for your stack!
