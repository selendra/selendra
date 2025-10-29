---
title: Network Overview
section: Overview
order: 2
---

# Network Overview

Production blockchain. Live since 2022. v3.0 since 2025.

## Core Specifications

**Consensus**
- Aura: Produces blocks every 1000ms
- AlephBFT: Finalizes in under 2 seconds
- BFT security guarantees

**Runtime**
- Spec Version: 20004
- Spec Name: `selendra`
- Framework: Cardinal Cryptography's Polkadot SDK (aleph-v1.6.0)
- State Version: 2

**EVM**
- Chain ID: 1961
- Gas Limit: ~15M per block
- Framework: Frontier
- Precompiles: 7 (5 standard + 2 custom)

## Runtime Pallets (30 Total)

**Core System**
- frame_system
- pallet_aura (block production)
- pallet_aleph (finality)
- pallet_timestamp
- pallet_balances
- pallet_transaction_payment
- pallet_scheduler

**Staking & Governance**
- pallet_staking
- pallet_session
- pallet_elections (custom DPoS)
- pallet_committee_management
- pallet_treasury
- pallet_nomination_pools

**EVM Integration**
- pallet_ethereum
- pallet_evm
- pallet_dynamic_evm_base_fee
- pallet_unified_accounts

**Smart Contracts**
- pallet_contracts (ink! Wasm)

**Utility**
- pallet_utility
- pallet_multisig
- pallet_identity
- pallet_vesting
- pallet_proxy

## EVM Precompiles

**Standard (Ethereum)**
- 0x01: ECRecover
- 0x02: Sha256
- 0x03: Ripemd160
- 0x04: Identity
- 0x05: Modexp

**Custom (Selendra)**
- 0x0400 (1024): Sha3FIPS256
- 0x0401 (1025): ECRecoverPublicKey

## Network Parameters

**Block Production**
- MILLISECS_PER_BLOCK: 1000
- MAX_BLOCK_WEIGHT: 400ms of compute

**Deposits**
- Contract storage: 0.00004 SEL per byte
- Existential: 0.0000005 SEL

**Staking**
- Bonding Duration: 14 eras
- Sessions per Era: 6
- Session Period: 900 blocks (15 minutes)

## Endpoints

**Mainnet**
- RPC: `https://rpc.selendra.org`
- WebSocket: `wss://rpc.selendra.org`
- Explorer: `https://explorer.selendra.org`

**Testnet**
- RPC: `https://rpc-testnet.selendra.org`
- WebSocket: `wss://rpc-testnet.selendra.org`

## What's Next

v4.0 Development (2026):
- Additional EVM precompiles (Staking, Governance, Oracles)
- Enhanced account abstraction
- LayerZero cross-chain bridge
- Native DEX (10x cheaper than EVM)
- Full governance (remove sudo)
