# Selendra Network

<div align="center">

[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)
[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-brightgreen)](https://substrate.io/)
[![EVM Compatible](https://img.shields.io/badge/EVM-Compatible-blue)](https://ethereum.org/)

</div>

## Overview

Selendra is an enterprise-grade blockchain platform that combines EVM compatibility with a unique hybrid consensus mechanism. Built on Substrate, it offers a dual virtual machine system supporting both WebAssembly and EVM, making it ideal for enterprise adoption and developing markets.

### Key Features

- **Hybrid Consensus**: Combines Aura for block production with Aleph for finality
- **Dual VM Support**: Compatible with both EVM and WebAssembly
- **Enterprise Focus**: Built-in identity, committee management, and recovery systems
- **Fast Finality**: 1-second block time with quick transaction finality
- **Economic Model**: Dynamic fee adjustment and multi-tiered token system

## Technical Specifications

### Core Parameters
- **Block Time**: 1000ms (1 second)
- **Token (SEL)**:
  - 18 decimal places
  - Multiple denominations (MILLI, MICRO, NANO, PICO)
- **Consensus**: Up to 100 validators
- **Transaction Fees**: Dynamic adjustment based on network usage

### Enterprise Features
- **Identity Management**: Built-in system for KYC/AML compliance
- **Committee Governance**: Flexible management system
- **Multi-signature Support**: Enhanced security for enterprise users
- **Recovery Mechanisms**: Account recovery options

## Getting Started

### Prerequisites
- Rust and Cargo
- Node.js (for DApp development)
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/selendra/selendra
cd selendra

# Build the node
cargo build --release

# Run a development node
./target/release/selendra --dev
```

### Documentation

Comprehensive documentation is available in the `/docs` directory and at [docs.selendra.org](https://docs.selendra.org)

#### Key Documentation
- [Tokenomics Design](docs/TOKENOMICS.md) - Detailed token economics and incentive structures
- [Network Improvements](docs/IMPROVEMENTS.md) - Proposed enhancements and future developments
- [Technical Docs](https://docs.selendra.org) - API references and technical guides

## Use Cases

### Enterprise Solutions
- Supply chain management
- Digital identity systems
- Corporate governance
- Asset tokenization

### Financial Services
- Cross-border payments
- DeFi applications
- Microfinance solutions

### Development Focus
- Smart contract deployment (EVM/Wasm)
- DApp development
- Enterprise integration

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

Selendra is licensed under either of the following, at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- GNU General Public License v3.0 ([LICENSE-GPL](LICENSE-GPL))
=======
## Overview

Selendra Network is an EVM-compatible Layer 1 blockchain built on Substrate with AlephBFT consensus for finality and a Proof-of-Stake security model. Selendra aims to serve both business and general users across multiple use cases, with particular focus on DeFi, Real World Assets (RWA), Loyalty programs, and Privacy features.


## Key Features

Selendra project is inspired by the excellent work of many growing projects in the Polkadot ecosystem and many other blockchain developers around the world. Our progress in the past, the present and the future is only possible thanks to the open sources software community, framework, and tools.

Special thanks to:
- The Substrate and Polkadot ecosystem for providing the foundational framework
- The global blockchain development community for their continuous innovations
- All contributors and supporters who help make Selendra better

This is a work in progress, and we will continue to update information as we progress further. For more details about our token economy, please refer to our [Tokenomics Design](docs/TOKENOMICS.md) document.

## Contact

- Website: [selendra.org](https://selendra.org)
- Email: [dev@selendra.org](mailto:dev@selendra.org)
- Twitter: [@SelendraNetwork](https://twitter.com/SelendraNetwork)

---

<div align="center">

Built with ❤️ for a decentralized future.

</div>

- **EVM Compatibility**: Run Ethereum smart contracts on Selendra
- **AlephBFT Consensus**: Provides fast finality with Proof-of-Stake security
- **Privacy Features**: Zero-knowledge proofs and confidential transactions
- **Cross-Chain Compatibility**: Designed for interoperability
- **SEL Token**: Native utility token for gas fees, staking, and governance

## Codebase Structure

The Selendra Network codebase follows the typical Substrate node architecture:

```bash
selendra/
├── node/ # Selendra node implementation
├── runtime/ # Runtime logic and pallets
├── pallets/ # Custom pallets
│ ├── staking/ # PoS staking logic
│ ├── evm/ # Ethereum Virtual Machine integration
│ └── privacy/ # Privacy features implementation
├── primitives/ # Core types and traits
├── client/ # Client implementation
└── scripts/ # Utility scripts
```

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable)
- LLVM and Clang
- Additional development libraries

### Installation Steps

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Initialize your Rust environment:
```bash
source $HOME/.cargo/env
```

3. Add Wasm target:
```bash
rustup target add wasm32-unknown-unknown
```

4. Clone the repository:
```bash
git clone https://github.com/selendra/selendra.git
cd selendra
```

5. Build the node:
```bash
cargo build --release
```

6. Run the node:
```bash
./target/release/selendra-node --dev
```

## Running a Selendra RPC Node

For running a production RPC node, Selendra provides an automated setup script:

```bash
# Download with wget
wget https://github.com/selendra/selendra-rpc/raw/main/selendra-rpc-setup.sh -O selendra-rpc-setup.sh

# Make it executable
chmod +x selendra-rpc-setup.sh

# Run with sudo
sudo ./selendra-rpc-setup.sh
```

For more information, see our [RPC node documentation](https://github.com/selendra/selendra-rpc).

## Development Resources

- [Block Explorer](https://explorer.selendra.org)

## Contributing

We welcome contributions to Selendra Network! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgements

We are grateful and would like to thank the following projects and individuals for their contributions to open source knowledge and so our team could build Selendra Network:

- [AlephBFT](https://github.com/aleph-network/aleph-bft)
- [Substrate](https://github.com/paritytech/substrate)
- [Ethereum](https://github.com/ethereum/ethereum)
- [Polkadot](https://github.com/paritytech/polkadot)
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)

## License

Selendra Network is released under the GPL-3.0 License.

