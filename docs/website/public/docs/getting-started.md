---
title: Getting Started
section: Overview
order: 1
---

# Getting Started

Deploy in 5 minutes. Or less.

## What is Selendra?

EVM-compatible L1 blockchain. 1000ms blocks. Sub-2 second finality.

Started 2019. Mainnet 2022. v3.0 launched 2025.

30,000 users. Real apps. Not testnet.

## Quick Deploy

Your Solidity contracts. Your tools. Zero changes.

### Prerequisites

- Node.js 18+
- MetaMask or any web3 wallet

### Connect

```javascript
// Network Settings
RPC: https://rpc.selendra.org
Chain ID: 1961
Currency: SEL
Block Explorer: https://explorer.selendra.org
```

### Deploy

```bash
npm install @selendra/sdk

# Your existing Hardhat/Truffle config works
npx hardhat run scripts/deploy.js --network selendra
```

That's it.

## Key Numbers

- **Block Time**: 1000ms (Aura)
- **Finality**: <2s (AlephBFT)
- **Gas Limit**: 15M+ per block
- **Runtime**: v3.0 (Spec 20004)
- **Pallets**: 30
- **EVM Precompiles**: 7

## What Makes It Different

**Unified Accounts**
One account. EVM and native. No bridges.

**Native Stablecoins**
KHRt (Cambodian Riel). sUSD (DeFi). Direct bank integration.

**Account Abstraction**
Lost your keys? Social recovery. Session keys for dApps.

**Real Users**
30,000 people buying tickets, running shops, sending money home.

## Next Steps

- [Network Details](/docs/network-overview)
- [Deploy Contracts](/docs/smart-contracts)
- [Run a Node](/docs/run-a-node)
