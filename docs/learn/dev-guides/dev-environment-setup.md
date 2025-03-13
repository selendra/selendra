# Setting Up Your Selendra Development Environment

This guide will help you set up a complete development environment for building applications on Selendra.

## Prerequisites

Before you begin, make sure you have:

- **Operating System**: Linux, macOS, or Windows with WSL
- **Node.js**: v14.0.0 or higher
- **Yarn**: v1.22.0 or higher
- **Git**: Latest version
- **Docker**: For running a local Selendra node (optional but recommended)

## Step 1: Install Development Tools

### Node.js and Yarn

```bash
# Using NVM (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
nvm install 16
nvm use 16

# Install Yarn
npm install -g yarn
```

### Rust (for WebAssembly development)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

## Step 2: Clone Selendra Templates

```bash
# For EVM (Solidity) development
git clone https://github.com/selendra/selendra-evm-starter.git
cd selendra-evm-starter
yarn install

# For WASM (ink!) development
git clone https://github.com/selendra/selendra-ink-starter.git
cd selendra-ink-starter
yarn install
```

## Step 3: Set Up a Local Selendra Node

The easiest way to run a local Selendra node is using Docker:

```bash
docker pull selendrachain/selendra:latest
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external
```

This will start a development node with the following endpoints:
- WebSocket: ws://127.0.0.1:9944
- HTTP: http://127.0.0.1:9933

## Step 4: Configure Development Tools

### Hardhat Configuration (for EVM development)

Create a `hardhat.config.js` file in your project:

```javascript
module.exports = {
  solidity: "0.8.17",
  networks: {
    selendraLocal: {
      url: "http://127.0.0.1:9933",
      accounts: [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" // Dev account
      ],
      chainId: 1994
    },
    selendraTestnet: {
      url: "https://testnet.selendra.org",
      accounts: ["YOUR_PRIVATE_KEY"], // Replace with your account's private key
      chainId: 1994
    }
  }
};
```

### Substrate/Polkadot.js Configuration

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function connectToSelendra() {
  // Connect to local node
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider: wsProvider });
  
  // Display chain information
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  
  console.log(`Connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
  return api;
}

connectToSelendra();
```

## Step 5: Set Up Browser Extensions

For testing your dApps, install:

1. **MetaMask**: For EVM compatibility
   - Add Selendra network with ChainID 1994
   - RPC URL: http://127.0.0.1:9933 (local) or https://testnet.selendra.org (testnet)

2. **Polkadot.js Extension**: For native Substrate functionality
   - Add local development node: 127.0.0.1:9944
   - Add testnet: wss://testnet.selendra.org

## Troubleshooting Common Issues

- **Connection refused errors**: Ensure your node is running and ports are correctly mapped
- **Gas fee errors**: Make sure your account has sufficient SEL for transactions
- **Contract deployment failures**: Verify your contract compiles correctly and follows EVM standards
- **WebAssembly errors**: Ensure you're using the correct Rust and ink! versions

## Next Steps

Now that your development environment is set up, you can:

- Build your first dApp following our [First dApp Guide](./first-dapp.md)
- Explore EVM development with [EVM Smart Contracts Guide](./evm-contracts.md)
- Learn about WebAssembly with [WASM Contracts Guide](./wasm-contracts.md) 