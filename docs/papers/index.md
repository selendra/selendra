# Selendra Network Developer Documentation

Welcome to the comprehensive guide for developers looking to build on the Selendra Network. This documentation covers everything from setting up nodes to deploying smart contracts and integrating cross-chain functionality.

## Introduction to Selendra for Developers

Selendra is an EVM-compatible Layer 1 blockchain built on Substrate with AlephBFT consensus. It features:

- **1-second block time**: Ultra-fast transaction finality
- **High throughput**: Designed for 5,000 TPS under optimal conditions
- **EVM compatibility**: Full support for Ethereum tools and applications
- **WebAssembly contracts**: Support for ink! smart contracts
- **Privacy features**: Planned infrastructure for confidential transactions
- **Cambodia-focused**: Built with Southeast Asian enterprise needs in mind

### Technical Architecture

```
selendra/
├── node/                # Selendra node implementation
├── runtime/             # Runtime logic and pallets
├── pallets/             # Custom pallets
│   ├── staking/         # PoS staking logic
│   ├── evm/             # Ethereum Virtual Machine integration
│   └── privacy/         # Privacy features implementation (future)
├── primitives/          # Core types and traits
├── client/              # Client implementation
└── scripts/             # Utility scripts
```

### Network Information

- **Chain ID**: 1961
- **Currency Symbol**: SEL
- **Decimals**: 18
- **Block Time**: 1 second
- **Consensus**: AlephBFT + Proof-of-Stake

## Documentation Sections

- [Setting Up a Selendra RPC Node](node-setup.md#setting-up-a-selendra-rpc-node)
- [Running a Validator Node](node-setup.md#running-a-validator-node)
- [Building a Wallet](wallet-integration.md)
- [Smart Contract Development](smart-contracts.md)
- [Cross-Chain Integration with Selendra Bridge](bridge-integration.md)
- [Dapp Development](dapp-development.md)
- [Resources and Support](resources.md) 