# Selendra

> The blockchain infrastructure for Cambodia.

## What is Selendra?

Selendra is a Substrate-based blockchain providing dual EVM and WASM execution environments. Designed for real-world applications with instant finality and sub-cent transaction costs.

- **Substrate-based**: Built on Polkadot SDK for forkless upgrades and modularity
- **Dual VM**: EVM for Solidity contracts, WASM for ink! smart contracts
- **1-second block time**: Fast confirmation with AlephBFT finality
- **Sub-cent transactions**: Typical transaction cost under $0.001

## Quick Start

### Connect MetaMask

| Field | Value |
|-------|-------|
| Network | Selendra Mainnet |
| RPC URL | https://rpc.selendra.org |
| Chain ID | 1961 |
| Currency | SEL |
| Explorer | https://explorer.selendra.org |

### Resources

- **Block Explorer**: https://explorer.selendra.org
- **Faucet (testnet)**: https://faucet.selendra.org
- **Documentation**: https://selendra.org/docs

## Key Features

| Feature | Description |
|---------|-------------|
| **EVM Compatible** | Deploy Solidity contracts, use MetaMask, Hardhat, Foundry |
| **WASM Support** | Build with ink! for gas-efficient smart contracts |
| **Instant Finality** | AlephBFT finality in seconds, not minutes |
| **Sub-cent Gas** | Typical transaction < $0.001 |
| **Forkless Upgrades** | Runtime upgrades without chain splits |

## Built for Cambodia

Cambodia is uniquely positioned for blockchain adoption:

- **Dollarized economy**: USD is the de facto currency
- **KHQR standard**: National payment system with 20+ banks integrated
- **Unbanked population**: 80% lack access to traditional banking
- **Tourism**: 7M+ annual visitors needing payment solutions
- **Digital transformation**: Government actively modernizing services

## Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Foundry (for Solidity contracts)

### Build from Source

```bash
git clone https://github.com/selendra/selendra.git
cd selendra
cargo build --release
```

The compiled binary will be at `target/release/selendra-node`

### Run a Node

```bash
# Development node (single node testnet)
./target/release/selendra-node --dev --tmp

# Mainnet node
./target/release/selendra-node \
  --chain=mainnet \
  --pruning=archive \
  --rpc-cors=all \
  --rpc-external
```

### Smart Contracts

#### Solidity (EVM)

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

#### ink! (WASM)

```bash
cargo install cargo-contract
cargo contract new my_contract
cd my_contract
cargo contract build
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific pallet tests
cargo test -p pallet-elections
```

## Network Specifications

| Property | Value |
|----------|-------|
| Chain ID | 1961 (Mainnet) |
| Block Time | ~1 second |
| Finality | Sub-second (AlephBFT) |
| Native Token | SEL |
| Decimals | 18 |
| Consensus | Aura + AlephBFT |

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Apache 2.0 - see [LICENSE](LICENSE) file for details.

## Links

| Resource | URL |
|----------|-----|
| Website | https://selendra.org |
| Documentation | https://selendra.org/docs |
| Block Explorer | https://explorer.selendra.org |
| Discord | https://discord.gg/selendra |
| Twitter | https://twitter.com/selendranetwork |
