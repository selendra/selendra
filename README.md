# Selendra

<div align="center">

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Substrate](https://img.shields.io/badge/Substrate-Polkadot--SDK-E6007A)](https://github.com/paritytech/polkadot-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.81+-orange.svg)](https://www.rust-lang.org)
[![EVM Compatible](https://img.shields.io/badge/EVM-Compatible-brightgreen.svg)](https://ethereum.org)

**A high-performance, EVM-compatible blockchain built with Substrate**

[Website](https://selendra.org) • [Documentation](https://selendra.org/docs) • [Telegram](https://t.me/selendranetwork)

</div>

---

## 🚀 Overview

Selendra is a next-generation Layer 1 blockchain that combines the best of Ethereum compatibility with Substrate's flexibility and performance. Built on Cardinal Cryptography's AlephBFT consensus, Selendra finalises blocks in about a second, at low transaction cost.

### Key Features

- **⚡ Blazing Fast**: About one second to finality with Aura block production + AlephBFT consensus
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
│  Core Pallets (36 Total)                            │
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
| **Finality** | ~1s median, 1.4s p95 (AlephBFT, measured on mainnet 14 Aug 2026) |
| **EVM Gas Limit** | 36M gas/block |
| **Runtime Version** | 20004 deployed on mainnet, 20016 on `master` |
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
│   ├── aleph-runtime-api/      # Aleph runtime API
│   ├── elections/              # DPoS validator elections
│   ├── committee-management/   # Validator committee management
│   ├── operations/             # Administrative operations
│   ├── dynamic-evm-base-fee/   # EVM fee adjustment
│   ├── ethereum-checked/       # Checked Ethereum transactions
│   ├── xvm/                    # Cross-VM calls
│   └── unified-accounts/       # Native ↔ EVM account mapping
├── crate/                      # Supporting libraries
│   ├── finality-aleph/         # AlephBFT finality gadget
│   ├── aggregator/             # Signature aggregation
│   ├── clique/                 # Peer discovery
│   ├── rate-limiter/           # Network rate limiting
│   ├── frontier/               # Ethereum compatibility (vendored)
│   ├── selendra-client/        # Rust client library
│   ├── finalizer/              # Finalisation tooling
│   ├── fork-off/               # Fork-off-from-live state tooling
│   └── unified-accounts-cli/   # Account binding CLI
├── primitives/                 # Core primitive types
├── scripts/                    # Deployment & utility scripts
└── vendors/
    └── polkadot-sdk/           # Vendored Polkadot SDK
```

---

## 🏗️ Building from Source

### Prerequisites

- **Rust**: 1.81.0 (pinned in [rust-toolchain.toml](rust-toolchain.toml))
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

For detailed validator setup, see [Validator Guide](https://selendra.org/docs/run-nodes/run-validator)

---

## 🔌 Connecting to Selendra

### Metamask Configuration

Add Selendra to Metamask:

- **Network Name**: Selendra Mainnet
- **RPC URL**: `https://rpc.selendra.org`
- **Chain ID**: `1961`
- **Currency Symbol**: `SEL`
- **Block Explorer**: `https://explorer.selendra.org`

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
- `Authorship` - Block author tracking

### Staking & Governance
- `Staking` - Proof-of-Stake validation
- `Elections` - DPoS validator elections
- `CommitteeManagement` - Validator committee
- `Session` - Session management
- `History` - Historical session data
- `Treasury` - Community fund management (Council-approved)
- `NominationPools` - Liquid staking pools
- `Council` - Community governance collective (13 members)
- `TechnicalCommittee` - Technical governance collective (7 members)
- `Democracy` - Public referendums and proposals
- `CouncilElections` - Phragmen-based council elections
- `Preimage` - Proposal preimage storage

### EVM Integration
- `Ethereum` - Ethereum compatibility layer
- `EVM` - EVM execution environment
- `DynamicEvmBaseFee` - Dynamic EVM fee adjustment
- `UnifiedAccounts` - Native ↔ EVM account mapping
- `EthereumChecked` - Checked Ethereum transactions
- `Xvm` - Cross-VM calls

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
- `Sudo` - Superuser access (transition period, to be removed post-governance maturity)
- `SafeMode` - Emergency chain halt
- `TxPause` - Transaction filtering

---

## 🗺️ Roadmap

### History
- **2019**: Project started
- **2020**: First testnet launched
- **2022**: v1 mainnet launch
- **2025**: v3 mainnet launch (current)

### Current Status (v3.0 - October 2025)
- ✅ Full EVM compatibility via Frontier
- ✅ AlephBFT consensus (~1s finality)
- ✅ Unified accounts (native ↔ EVM)
- ✅ DPoS staking with nomination pools
- ✅ Dynamic EVM fee adjustment
- ✅ Council governance (13-member council)
- ✅ Democracy & referendum system
- ✅ Treasury with Council approval
- ✅ 36 runtime pallets
- ✅ Mainnet live and operational

### Next Steps (Q4 2025 - Q2 2026)
- 🔧 Fix critical security issues (randomness, storage bounds)
- 📦 TypeScript SDK development
- 🏛️ Transition governance (reduce sudo, increase council powers)
- 🗳️ First council elections
- 🛠️ Enhanced developer tooling
- 💱 DeFi infrastructure (DEX, oracles)
- 📚 Comprehensive documentation

### Long-Term (2026+)
- 🌉 Cross-chain bridges
- 🏦 Expanded DeFi ecosystem
- 🎯 Developer adoption in Cambodia and Southeast Asia
- 🏢 Real-world use cases (remittance, supply chain)

See [docs/design/chain-dev.md](docs/design/chain-dev.md) for detailed technical roadmap.

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

### Known Issues

- ⚠️ **Randomness**: Currently using insecure collective flip (fix in progress)
- ⚠️ **Sudo**: Superuser key present during governance transition period (removal planned after 6 months)
- ⚠️ **Contract Calls**: Wasm contracts have limited runtime call access
- ℹ️ **Governance**: Council system implemented, undergoing testing before sudo removal

See [Security Advisory](https://github.com/selendra/selendra/security/advisories) for details.

---

## 📄 License

This project is licensed under the **Apache License 2.0** - see [LICENSE](LICENSE) file.

```
Copyright (C) 2019-2025 Selendra

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
```

Polkadot-SDK client crates are GPL-3.0 WITH Classpath-exception-2.0, whose linking
exception permits distributing the combined node under Apache-2.0.

---

## 🔗 Links & Resources

### Official

- **Website**: [selendra.org](https://selendra.org)
- **Documentation**: [selendra.org/docs](https://selendra.org/docs)
- **Block Explorer**: [explorer.selendra.org](https://explorer.selendra.org)
- **Wallet Portal**: [portal.selendra.org](https://portal.selendra.org)
- **GitHub**: [github.com/selendra](https://github.com/selendra)

### Community

- **Telegram**: [t.me/selendranetwork](https://t.me/selendranetwork)
- **X**: [@selendranetwork](https://x.com/selendranetwork)

### Developer Resources

- **RPC Endpoint**: `https://rpc.selendra.org`
- **WebSocket**: `wss://rpc.selendra.org`
- **Testnet RPC**: `https://rpc-testnet.selendra.org`
- **Faucet**: [faucet.selendra.org](https://faucet.selendra.org)

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

[Join us on Telegram](https://t.me/selendranetwork) • [Follow on X](https://x.com/selendranetwork) • [Read the Docs](https://selendra.org/docs)

</div>
