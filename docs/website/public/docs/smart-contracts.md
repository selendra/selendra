---
title: Smart Contracts
section: Developers
order: 5
---

# Smart Contracts on Selendra

Deploy and interact with smart contracts on Selendra.

## EVM Contracts (Solidity)

Selendra is fully EVM-compatible through Frontier. Deploy your existing Ethereum contracts without modification.

### Deploy with Hardhat

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    selendra: {
      url: 'https://rpc.selendra.org',
      accounts: [PRIVATE_KEY],
      chainId: TBD
    }
  },
  solidity: '0.8.20'
}
```

### Deploy with Remix

1. Open [Remix IDE](https://remix.ethereum.org)
2. Connect to Selendra network via MetaMask
3. Compile and deploy your contract

## ink! Contracts (Substrate Native)

Use ink! for native Substrate smart contracts with lower gas costs.

### Install cargo-contract

```bash
cargo install cargo-contract --force
```

### Create a Contract

```bash
cargo contract new flipper
cd flipper
cargo contract build
```

### Deploy

```bash
cargo contract instantiate \
  --constructor new \
  --args true \
  --suri //Alice
```

## Best Practices

- Always audit contracts before mainnet deployment
- Test thoroughly on testnet first
- Use established libraries (OpenZeppelin, etc.)
- Implement upgrade patterns when necessary
