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

## Acknowledgement

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
