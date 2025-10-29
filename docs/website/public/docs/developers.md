---
title: Build on Selendra
section: Developers
order: 4
---

# Build on Selendra

5 minutes to deploy. Zero code changes.

## Why Build Here

**It just works**
Your Solidity code. MetaMask. Hardhat. Remix. All of it.

**Real users waiting**
30,000 people. Buying tickets. Running shops. Not trading.

**Low friction**
No testnet dance. Deploy straight to mainnet. Low fees.

## Quick Start

### Install

```bash
npm install -g @selendra/cli
```

### Deploy

```bash
selendra init my-dapp
cd my-dapp
selendra deploy --network mainnet
```

Done.

## Development Tools

### Smart Contracts

**Solidity (EVM)**
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
  solidity: '0.8.20'
}
```

**ink! (Wasm)**
```bash
cargo install cargo-contract
cargo contract new my-contract
cargo contract build
cargo contract deploy
```

### SDK

**@selendra/sdk**
TypeScript SDK. Full type safety.

```bash
npm install @selendra/sdk
```

```typescript
import { SelendraApi } from '@selendra/sdk'

const api = new SelendraApi('wss://rpc.selendra.org')
await api.connect()

// Get balance
const balance = await api.getBalance(address)

// Transfer
await api.transfer(to, amount)

// Deploy contract
const contract = await api.deployContract(abi, bytecode)
```

### Wallets

**MetaMask**
Add network:
- Network Name: Selendra
- RPC: https://rpc.selendra.org
- Chain ID: 1961
- Currency: SEL

**Polkadot.js Extension**
For native Substrate transactions.

## What Makes It Different

**Unified Accounts**
One account. Works with EVM and native runtime.

```typescript
// Same account for both
const evmAddress = '0x1234...'
const nativeAddress = 'selendra1234...'

// Linked automatically
api.linkAccounts(evmAddress, nativeAddress)
```

**Account Abstraction**
Social recovery. Session keys. No 12-word panic.

```typescript
// Setup recovery
await api.setupRecovery({
  guardians: [friend1, friend2, friend3],
  threshold: 2
})

// Lost keys? Guardians recover
await api.initiateRecovery(lostAccount, newAccount)
```

**Native Stablecoins**
KHRt. sUSD. Direct bank integration.

```typescript
// Get KHRt from bank
const khqr = await api.generateKHQR(amount)
// User pays via bank app
// KHRt appears in account

// Cash out to bank
await api.burnKHRt(amount, bankAccount)
```

## Developer Experience

**Instant Testnet Faucet**
No signup. No email. Just get tokens.

```bash
selendra faucet 0x1234...
```

**Web Playground**
Browser IDE. Deploy in 5 minutes.

https://playground.selendra.org

**10+ Templates**
Production-ready. Copy and deploy.

```bash
selendra create --template erc20
selendra create --template nft
selendra create --template defi-staking
```

**Auto-Generated Docs**
Write code. Docs generate automatically.

```bash
selendra docs generate
```

## Examples

**Token Contract**
```solidity
// Your existing ERC20. Works unchanged.
contract Token is ERC20 {
    constructor() ERC20("MyToken", "MTK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}
```

**NFT Marketplace**
```bash
git clone https://github.com/selendra/nft-marketplace
cd nft-marketplace
npm install
npm run deploy
```

**DeFi Protocol**
```bash
git clone https://github.com/selendra/defi-template
cd defi-template
npm install
npm run deploy
```

## Resources

- GitHub: https://github.com/selendra
- SDK Docs: https://docs.selendra.org/sdk
- Examples: https://github.com/selendra/examples
- Discord: https://discord.gg/selendra

## Get Help

**Discord Developer Channel**
Ask questions. Get answers. Fast.

**Office Hours**
Every Thursday 3pm UTC.
Live coding. Q&A.

**GitHub Issues**
Bug? Report it.
https://github.com/selendra/selendra/issues
