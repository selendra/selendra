## 🙋‍♀️ Selendra Network

## Overview

Selendra Network is an EVM-compatible Layer 1 blockchain built on Substrate with AlephBFT consensus for finality and a Proof-of-Stake security model. Selendra aims to serve both business and general users across multiple use cases, with particular focus on DeFi, Real World Assets (RWA), Loyalty programs, and Privacy features.

## Key Features

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
