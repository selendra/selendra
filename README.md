## 🙋‍♀️ Selendra Network

## Overview

Selendra Network is an EVM-compatible Layer 1 blockchain built on Substrate with AlephBFT consensus for finality and a Proof-of-Stake security model. Selendra aims to serve both business and general users across multiple use cases, with particular focus on DeFi, Real World Assets (RWA), Loyalty programs, and Privacy features.

## Key Features

- **Hybrid Consensus**: Combines Aura for block production with Aleph for finality
- **Dual VM Support**: Compatible with both EVM and WebAssembly
- **Enterprise Focus**: Built-in identity, committee management, and recovery systems
- **Fast Finality**: 1-second block time with quick transaction finality
- **Economic Model**: Dynamic fee adjustment and multi-tiered token system

## Codebase Structure

The Selendra Network codebase follows the typical Substrate node architecture:

```bash
selendra/
├── bin/ # Binary implementations
│ ├── node/ # Selendra node implementation
│ ├── runtime/ # Runtime logic and pallets
│ ├── client-runtime-api/ # Client runtime API
│ └── chain-bootstrapper/ # Chain bootstrapping utilities
├── pallets/ # Custom pallets
│ ├── aleph/ # AlephBFT consensus integration
│ ├── committee-management/ # Validator committee management
│ ├── elections/ # Validator elections
│ ├── operations/ # Administrative operations
│ ├── dynamic-evm-base-fee/ # EVM fee adjustment
│ └── privacy/ # Privacy-enabled smart contracts (Planned)
├── primitives/ # Core types and traits
├── crate/ # Core crates
│ ├── finality-aleph/ # AlephBFT finality implementation
│ ├── clique/ # Clique consensus
│ ├── aggregator/ # Block aggregation
│ └── rate-limiter/ # Rate limiting utilities
├── scripts/ # Utility scripts
├── docs/ # Documentation
├── vendors/ # Third-party dependencies
│ └── frontier/ # Ethereum compatibility layer (EVM & WASM)
└── target/ # Build artifacts (EVM & WASM runtimes)
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
### With Docker + Domain + SSL (recommended):
```bash
curl -sSL https://raw.githubusercontent.com/selendra/selendra/main/scripts/deploy_rpc_node_ubuntu.sh -o deploy-selendra-rpc

```

```bash
./deploy-selendra-rpc rpc.yourdomain.com your-email@gmail.com
```

* Replace `rpc.yourdomain.com` → your domain (must point to the server’s IP).
* Replace `your-email@gmail.com` → for Let’s Encrypt SSL.

---

## 4. One-line quick deployment

If you want to skip downloading:

```bash
curl -sSL https://raw.githubusercontent.com/selendra/selendra/main/scripts/deploy_rpc_node_ubuntu.sh | bash -s -- rpc.yourdomain.com your-email@gmail.com
```

---

✅ Done!
After completion, your Selendra RPC node will be running at:

```
https://rpc.yourdomain.com
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

<div align="center">

Built with ❤️ for a decentralized future.

</div>
