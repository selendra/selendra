# Selendra

<div align="center">

[![License](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](LICENSE)
[![Substrate](https://img.shields.io/badge/Substrate-Polkadot--SDK-E6007A)](https://substrate.io)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![EVM Compatible](https://img.shields.io/badge/EVM-Compatible-brightgreen.svg)](https://ethereum.org)

**A high-performance, EVM-compatible blockchain built with Substrate**

[Website](https://selendra.org) • [Documentation](https://docs.selendra.org) • [Discord](https://discord.gg/selendra) • [Telegram](https://t.me/selendra)

</div>

---

## 🚀 Overview

Selendra is a next-generation Layer 1 blockchain that combines the best of Ethereum compatibility with Substrate's flexibility and performance. Built on Cardinal Cryptography's AlephBFT consensus, Selendra delivers sub-second finality, low transaction costs, and enterprise-grade reliability.

### Key Features

- **⚡ Blazing Fast**: Sub-second finality with Aura block production + AlephBFT consensus
- **🔗 EVM Compatible**: Full Ethereum compatibility via Frontier - deploy Solidity contracts seamlessly
- **🔐 Unified Accounts**: Native ↔ EVM account mapping for superior user experience
- **💰 Low Fees**: Optimized transaction costs with dynamic EVM base fee adjustment
- **🎯 Enterprise Ready**: Advanced staking, governance tools, and institutional features
- **🛠️ Developer Friendly**: Substrate pallets + EVM smart contracts in one platform

### Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Selendra Network                 │
├─────────────────────────────────────────────────────┤
│  Consensus Layer                                    │
│  ├─ Aura (Block Production, 1s slots)               │
│  └─ AlephBFT (Byzantine Fault Tolerant Finality)    │
├─────────────────────────────────────────────────────┤
│  Execution Environments                             │
│  ├─ EVM (Ethereum Virtual Machine via Frontier)     │
│  ├─ WASM (WebAssembly Contracts via pallet-contracts)│
│  └─ Native Pallets (Substrate Runtime Logic)        │
├─────────────────────────────────────────────────────┤
│  Core Pallets (27 Total)                            │
│  ├─ Staking & Governance (DPoS, Treasury, Council)  │
│  ├─ EVM Integration (Ethereum, Dynamic Fees)        │
│  ├─ Unified Accounts (Native ↔ EVM Mapping)         │
│  └─ Utilities (Multisig, Proxy, Identity, Vesting)  │
└─────────────────────────────────────────────────────┘
```

### Network Specifications

| Property | Value |
|----------|-------|
| **Chain ID** | 1961 (Mainnet) |
| **Block Time** | ~1 second |
| **Finality** | Sub-second (AlephBFT) |
| **EVM Gas Limit** | ~15M gas/block |
| **Runtime Version** | v20004 (v2.0.4) |
| **Native Token** | SEL |
| **Decimals** | 18 |
| **Consensus** | Aura + AlephBFT |

---

## 📦 Repository Structure

```
selendra/
├── bin/                        # Executables
│   ├── node/                   # Selendra node implementation
│   ├── runtime/                # Selendra runtime (WASM + native)
│   ├── chain-bootstrapper/     # Network bootstrapping tool
│   └── client-runtime-api/     # Client-side runtime APIs
├── pallets/                    # Custom Substrate pallets
│   ├── aleph/                  # AlephBFT integration
│   ├── elections/              # DPoS validator elections
│   ├── committee-management/   # Validator committee management
│   ├── operations/             # Administrative operations
│   ├── dynamic-evm-base-fee/   # EVM fee adjustment
│   └── unified-accounts/       # Native ↔ EVM account mapping
├── crate/                      # Supporting libraries
│   ├── finality-aleph/         # AlephBFT finality gadget
│   ├── aggregator/             # Signature aggregation
│   ├── clique/                 # Peer discovery
│   └── rate-limiter/           # Network rate limiting
├── primitives/                 # Core primitive types
├── scripts/                    # Deployment & utility scripts
└── vendors/                    # Vendored dependencies
    ├── selendra-client/        # Rust client library (v3.16.0)
    ├── frontier/               # Ethereum compatibility (submodule)
    └── bind_account/           # Account binding utilities
```

---

## 🏗️ Building from Source

### Prerequisites

- **Rust**: 1.75.0 or later (see [rust-toolchain.toml](rust-toolchain.toml))
- **OS**: Linux (recommended), macOS, or WSL2 on Windows
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Disk**: 50GB+ free space

### Installation

1. **Install Rust & Dependencies**

   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install build dependencies (Ubuntu/Debian)
   sudo apt update
   sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev protobuf-compiler

   # Install build dependencies (macOS)
   brew install openssl cmake protobuf
   ```

2. **Clone the Repository**

   ```bash
   git clone https://github.com/selendra/selendra.git
   cd selendra

   # Initialize submodules (Frontier, etc.)
   git submodule update --init --recursive
   ```

3. **Build the Node**

   ```bash
   # Development build (faster compilation)
   cargo build --release

   # Production build (optimized for performance)
   cargo build --profile production
   ```

   The compiled binary will be at `target/release/selendra-node` (or `target/production/selendra-node`)

---

## 🚀 Running a Node

### Development Node (Single Node Testnet)

```bash
# Run a local development chain
./target/release/selendra-node \
  --dev \
  --tmp \
  --rpc-cors=all \
  --rpc-methods=unsafe
```

Access the node:
- RPC Endpoint: `http://localhost:9944`
- Ethereum RPC: `http://localhost:9933` (Web3/Metamask compatible)

### Mainnet Node

```bash
# Sync with Selendra mainnet
./target/release/selendra-node \
  --chain=mainnet \
  --pruning=archive \
  --rpc-cors=all \
  --rpc-external \
  --ws-external \
  --rpc-methods=safe \
  --name="MySelendraNode"
```

### Validator Node

```bash
# Run a validator (requires staking)
./target/release/selendra-node \
  --chain=mainnet \
  --validator \
  --name="MyValidator" \
  --rpc-methods=safe \
  --prometheus-external
```

For detailed validator setup, see [Validator Guide](https://docs.selendra.org/validators)

---

## 🔌 Connecting to Selendra

### Metamask Configuration

Add Selendra to Metamask:

- **Network Name**: Selendra Mainnet
- **RPC URL**: `https://rpc.selendra.org`
- **Chain ID**: `1961`
- **Currency Symbol**: `SEL`
- **Block Explorer**: `https://scan.selendra.org`

### Polkadot.js Apps

Connect via [Polkadot.js Apps](https://polkadot.js.org/apps):
- Custom Endpoint: `wss://rpc.selendra.org`

### Using TypeScript/JavaScript

```typescript
// Coming soon: @selendra/sdk
// For now, use @polkadot/api or web3.js

// Option 1: Polkadot.js API (for Substrate calls)
import { ApiPromise, WsProvider } from '@polkadot/api';

const provider = new WsProvider('wss://rpc.selendra.org');
const api = await ApiPromise.create({ provider });

// Query balance
const balance = await api.query.system.account('ACCOUNT_ADDRESS');
console.log('Balance:', balance.data.free.toHuman());

// Option 2: Web3.js (for EVM calls)
import Web3 from 'web3';

const web3 = new Web3('https://rpc.selendra.org');
const balance = await web3.eth.getBalance('0xEVM_ADDRESS');
console.log('Balance:', web3.utils.fromWei(balance, 'ether'), 'SEL');
```

---

## 🛠️ Development

### Smart Contract Deployment

#### Solidity (EVM)

1. **Using Hardhat**

   ```javascript
   // hardhat.config.js
   module.exports = {
     networks: {
       selendra: {
         url: 'https://rpc.selendra.org',
         chainId: 1961,
         accounts: [process.env.PRIVATE_KEY]
       }
     },
     solidity: "0.8.20"
   };
   ```

   ```bash
   npx hardhat deploy --network selendra
   ```

2. **Using Remix IDE**
   - Set Metamask to Selendra network
   - Compile contract in Remix
   - Deploy via "Injected Provider - Metamask"

#### ink! (WASM)

```bash
# Install cargo-contract
cargo install cargo-contract

# Create new contract
cargo contract new my_contract
cd my_contract

# Build contract
cargo contract build

# Deploy via Polkadot.js Apps
# 1. Upload WASM & metadata.json
# 2. Instantiate contract
```

### Runtime Development

Modify or add pallets in `pallets/`:

```bash
# Create a new pallet
cargo new --lib pallets/my-pallet

# Add to workspace in Cargo.toml
# [workspace]
# members = [
#   ...
#   "pallets/my-pallet"
# ]

# Include in runtime (bin/runtime/src/lib.rs)
construct_runtime!(
    pub struct Runtime {
        ...
        MyPallet: pallet_my_pallet,
    }
);
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific pallet tests
cargo test -p pallet-elections

# Run runtime tests
cargo test -p selendra-runtime

# Run with output
cargo test -- --nocapture
```

---

## 🌐 EVM Precompiles

Selendra provides custom precompiles for accessing Substrate functionality from EVM:

| Address | Function | Description |
|---------|----------|-------------|
| `0x0000...0001` | ECRecover | Ethereum signature recovery |
| `0x0000...0002` | SHA256 | SHA-256 hashing |
| `0x0000...0003` | RIPEMD160 | RIPEMD-160 hashing |
| `0x0000...0004` | Identity | Data copy |
| `0x0000...0005` | ModExp | Modular exponentiation |
| `0x0000...0400` | SHA3FIPS256 | SHA3-256 (FIPS) |
| `0x0000...0401` | ECRecoverPublicKey | Public key recovery |

**Coming Soon:**
- `0x0402` - Oracle Price Feeds
- `0x0403` - Staking Interface
- `0x0404` - Governance Interface
- `0x0405` - Unified Accounts

---

## 📊 Runtime Pallets

### Core System Pallets
- `System` - Core blockchain functionality
- `Aura` - Block production (Authority Round)
- `Aleph` - AlephBFT finality consensus
- `Timestamp` - Block timestamps
- `Balances` - Native token management
- `TransactionPayment` - Fee handling
- `Scheduler` - Delayed/scheduled calls

### Staking & Governance
- `Staking` - Proof-of-Stake validation
- `Elections` - DPoS validator elections
- `CommitteeManagement` - Validator committee
- `Session` - Session management
- `History` - Historical session data
- `Treasury` - Community fund management
- `NominationPools` - Liquid staking pools

### EVM Integration
- `Ethereum` - Ethereum compatibility layer
- `EVM` - EVM execution environment
- `DynamicEvmBaseFee` - Dynamic EVM fee adjustment
- `UnifiedAccounts` - Native ↔ EVM account mapping

### Smart Contracts
- `Contracts` - WASM smart contracts (ink!)

### Utilities
- `Utility` - Batch calls, multi-operations
- `Multisig` - Multi-signature accounts
- `Proxy` - Proxy accounts & delegation
- `Identity` - On-chain identity
- `Vesting` - Token vesting schedules

### Administrative
- `Operations` - Admin operations
- `Sudo` - Superuser access (temporary, being phased out)
- `SafeMode` - Emergency chain halt
- `TxPause` - Transaction filtering

---

## 🗺️ Roadmap

### Current Status (v2.0.4)
- ✅ Full EVM compatibility via Frontier
- ✅ AlephBFT consensus (sub-second finality)
- ✅ Unified accounts (native ↔ EVM)
- ✅ DPoS staking with nomination pools
- ✅ Dynamic EVM fee adjustment

### Phase 1: Foundation & Security (Q4 2025 - Q1 2026)
- 🔧 Fix critical security issues (randomness, storage bounds)
- 📦 Basic TypeScript SDK (core functionality)
- 🏛️ Governance preparation (Council setup)
- 📚 Essential documentation (quickstart, API reference)

### Phase 2: Developer Experience (Q2 2026)
- 🛠️ Enhanced SDK & tooling (Hardhat plugin, examples)
- 💱 Deploy proven DEX (Uniswap V2 fork)
- 🔮 Basic oracle integration (Chainlink price feeds)
- 💰 Launch small grants program ($500K initial)

### Phase 3: Ecosystem Growth (Q3-Q4 2026)
- 🌉 Ethereum bridge (via LayerZero or proven solution)
- 🏦 Add lending protocol (if DEX successful)
- 🔗 Stablecoin support (bridged USDC/USDT)
- 📈 Community building & partnerships

### Long-Term Goals (2027+)
- ⚡ Performance optimizations
- 🎯 Advanced features (based on market demand)
- 🏢 Enterprise adoption (if opportunity emerges)

See [Product Roadmap](selendra-product-and-chain-dev.md) for detailed plans.

---

## 🤝 Contributing

We welcome contributions! Here's how to get involved:

### Development Workflow

1. **Fork & Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/selendra.git
   cd selendra
   git remote add upstream https://github.com/selendra/selendra.git
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/my-awesome-feature
   ```

3. **Make Changes & Test**
   ```bash
   cargo test --workspace
   cargo fmt
   cargo clippy
   ```

4. **Submit Pull Request**
   - Push to your fork
   - Open PR against `master` branch
   - Include clear description & tests
   - Link related issues

### Contribution Areas

- 🐛 **Bug Fixes**: Report or fix bugs
- ✨ **Features**: Propose new functionality
- 📖 **Documentation**: Improve guides & docs
- 🧪 **Testing**: Add test coverage
- 🔍 **Security**: Report vulnerabilities (security@selendra.org)
- 🌍 **Translations**: Translate documentation

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Pass `cargo clippy` with no warnings
- Write tests for new functionality
- Document public APIs with `///` comments

---

## 🔒 Security

### Reporting Vulnerabilities

**DO NOT** open public issues for security vulnerabilities.

- **Email**: security@selendra.org
- **PGP Key**: [Download](https://selendra.org/security/pgp-key.asc)
- **Bug Bounty**: Up to $500K via [Immunefi](https://immunefi.com/selendra) *(coming soon)*

### Security Audits

- **Status**: In progress
- **Firms**: CertiK, Trail of Bits
- **Scope**: Runtime, custom pallets, EVM integration
- **Reports**: Published at [selendra.org/security](https://selendra.org/security)

### Known Issues

- ⚠️ **Randomness**: Currently using insecure collective flip (fix in progress)
- ⚠️ **Sudo**: Superuser key present (being phased out with governance)
- ⚠️ **Contract Calls**: Wasm contracts have limited runtime call access

See [Security Advisory](https://github.com/selendra/selendra/security/advisories) for details.

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0** - see [LICENSE](LICENSE) file.

```
Copyright (C) 2024 Selendra

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
```

---

## 🔗 Links & Resources

### Official

- **Website**: [selendra.org](https://selendra.org)
- **Documentation**: [docs.selendra.org](https://docs.selendra.org)
- **Block Explorer**: [scan.selendra.org](https://scan.selendra.org)
- **GitHub**: [github.com/selendra](https://github.com/selendra)

### Community

- **Discord**: [discord.gg/selendra](https://discord.gg/selendra)
- **Telegram**: [t.me/selendra](https://t.me/selendra)
- **Twitter**: [@selendra](https://twitter.com/selendra)
- **Forum**: [forum.selendra.org](https://forum.selendra.org)

### Developer Resources

- **RPC Endpoint**: `https://rpc.selendra.org`
- **WebSocket**: `wss://rpc.selendra.org`
- **Testnet RPC**: `https://rpc-testnet.selendra.org`
- **Faucet**: [faucet.selendra.org](https://faucet.selendra.org) *(coming soon)*

### Technical

- **Substrate Docs**: [docs.substrate.io](https://docs.substrate.io)
- **Polkadot.js**: [polkadot.js.org](https://polkadot.js.org)
- **Frontier**: [github.com/paritytech/frontier](https://github.com/paritytech/frontier)
- **AlephBFT**: [github.com/Cardinal-Cryptography/AlephBFT](https://github.com/Cardinal-Cryptography/AlephBFT)

---

## 🙏 Acknowledgments

Selendra is built on the shoulders of giants:

- **Cardinal Cryptography** - AlephBFT consensus & Polkadot SDK fork
- **Parity Technologies** - Substrate framework & Frontier EVM
- **Polkadot** - Shared security model & ecosystem
- **Ethereum Foundation** - EVM specification & tooling
- **Community Contributors** - Bug reports, features, and support

Special thanks to all validators, developers, and community members making Selendra possible.

---

<div align="center">

**Built with ❤️ by the Selendra Team**

[Join our Discord](https://discord.gg/selendra) • [Follow on Twitter](https://twitter.com/selendra) • [Read the Docs](https://docs.selendra.org)

</div>
