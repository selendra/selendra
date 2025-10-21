---
title: Build on Selendra
section: Developers
order: 4
---

# Build on Selendra

Start building decentralized applications on Selendra.

## Development Tools

### Smart Contracts

Selendra supports both:
- **Solidity** (via Frontier EVM)
- **ink!** (Substrate native)

### SDKs & Libraries

- **Polkadot.js**: JavaScript/TypeScript SDK
- **Ethers.js/Web3.js**: For EVM contracts
- **Substrate API**: Direct runtime interaction

## Quick Start

### Deploy an EVM Contract

```javascript
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('https://rpc.selendra.org');
const wallet = new ethers.Wallet(PRIVATE_KEY, provider);

// Deploy your contract
const factory = new ethers.ContractFactory(abi, bytecode, wallet);
const contract = await factory.deploy();
await contract.waitForDeployment();

console.log('Contract deployed:', await contract.getAddress());
```

### Using Polkadot.js

```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

const wsProvider = new WsProvider('wss://rpc.selendra.org');
const api = await ApiPromise.create({ provider: wsProvider });

// Query chain info
const chain = await api.rpc.system.chain();
const lastHeader = await api.rpc.chain.getHeader();
console.log(`Connected to ${chain} - Block #${lastHeader.number}`);
```

## Resources

- [GitHub Repository](https://github.com/selendra/selendra)
- [API Documentation](https://docs.selendra.org/api)
- [Example dApps](https://github.com/selendra/examples)
