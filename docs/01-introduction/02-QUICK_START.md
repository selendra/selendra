# Quick Start Guide

> Available in:
> - English (Current)
> - [ភាសាខ្មែរ](02-QUICK_START.kh.md)
> - [ไทย](02-QUICK_START.th.md)
> - [Tiếng Việt](02-QUICK_START.vi.md)
> - [Bahasa Indonesia](02-QUICK_START.id.md)

## Regional Support

### Cambodia
- Technical Support (Khmer): [t.me/selendra_dev_kh](https://t.me/selendra_dev_kh)
- Local Documentation: [docs.selendra.org/kh](https://docs.selendra.org/kh)
- Community Chat: [t.me/selendra_cambodia](https://t.me/selendra_cambodia)

### Southeast Asia
- Vietnam Support: [t.me/selendra_vietnam](https://t.me/selendra_vietnam)
- Thailand Support: [t.me/selendra_thailand](https://t.me/selendra_thailand)
- Indonesia Support: [t.me/selendra_indonesia](https://t.me/selendra_indonesia)

### Enterprise Support
- Cambodia: [enterprise.selendra.org/kh](https://enterprise.selendra.org/kh)
- Southeast Asia: [enterprise.selendra.org/sea](https://enterprise.selendra.org/sea)

## Installation

### 1. Install Dependencies
```bash
# Update package list
sudo apt update

# Install build dependencies
sudo apt install -y \
    build-essential \
    clang \
    curl \
    git \
    libssl-dev \
    llvm \
    pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add Wasm target
rustup target add wasm32-unknown-unknown

# Install additional tools
cargo install cargo-watch cargo-edit
```

### 2. Build Selendra
```bash
# Clone repository
git clone https://github.com/selendra/selendra
cd selendra

# Build node
cargo build --release

# Check installation
./target/release/selendra --version
```

## Running a Node

### 1. Start Local Node
```bash
# Development chain
./target/release/selendra --dev

# Connect to testnet
./target/release/selendra --chain testnet

# Connect to mainnet
./target/release/selendra --chain mainnet
```

### 2. Node Configuration
```yaml
# config.yaml
base-path: /data/selendra
chain: mainnet

# Network configuration
port: 30333
rpc-port: 9933
ws-port: 9944
rpc-external: true
ws-external: true

# Validator configuration
validator: true
name: "my-node"
```

## Development Setup

### Regional Development Resources

#### Cambodia
- Local Development Workshops: [events.selendra.org/kh/dev](https://events.selendra.org/kh/dev)
- Khmer Language Resources: [dev.selendra.org/kh](https://dev.selendra.org/kh)
- University Programs: [edu.selendra.org/kh](https://edu.selendra.org/kh)

#### Southeast Asia
- Regional Developer Portal: [dev.selendra.org/sea](https://dev.selendra.org/sea)
- Local Meetups: [events.selendra.org/sea](https://events.selendra.org/sea)
- Translation Contributions: [github.com/selendra/translations](https://github.com/selendra/translations)

### 1. Smart Contract Development
```bash
# Install contract toolkit
cargo install cargo-contract

# Create new contract
cargo contract new my_contract
cd my_contract

# Build contract
cargo contract build

# Run tests
cargo test
```

### 2. Local Development Chain
```bash
# Start local chain
./target/release/selendra --dev

# Purge chain data
./target/release/selendra purge-chain --dev
```

## Wallet Setup

### 1. Create Account
```typescript
import { Keyring } from '@polkadot/api';
import { mnemonicGenerate } from '@polkadot/util-crypto';

// Generate mnemonic
const mnemonic = mnemonicGenerate();

// Create account
const keyring = new Keyring({ type: 'sr25519' });
const account = keyring.addFromMnemonic(mnemonic);

console.log('Address:', account.address);
console.log('Mnemonic:', mnemonic);
```

### 2. Connect to Node
```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function connect() {
    // Create provider
    const provider = new WsProvider('ws://localhost:9944');
    
    // Create API instance
    const api = await ApiPromise.create({ provider });
    
    // Get chain info
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    
    console.log(`Connected to ${chain} using ${nodeName} v${nodeVersion}`);
    
    return api;
}
```

## Basic Operations

### Regional Considerations

#### Supported Payment Methods
- Cambodia: ABA, ACLEDA, Wing, Pi Pay
- Vietnam: Momo, VNPay, ZaloPay
- Thailand: PromptPay, TrueMoney
- Indonesia: GoPay, OVO, DANA

#### Compliance Requirements
- KYC verification for each region
- Local regulatory compliance
- Regional transaction limits

### 1. Transfer Tokens
```typescript
async function transfer(api: ApiPromise, from: KeyringPair, to: string, amount: number) {
    // Create transfer
    const transfer = api.tx.balances.transfer(to, amount);
    
    // Sign and send
    const hash = await transfer.signAndSend(from);
    
    console.log('Transfer hash:', hash.toHex());
}
```

### 2. Query Chain State
```typescript
async function queryState(api: ApiPromise) {
    // Get latest block
    const lastHeader = await api.rpc.chain.getHeader();
    console.log('Latest block:', lastHeader.number.toNumber());
    
    // Get account balance
    const account = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const { data: balance } = await api.query.system.account(account);
    console.log('Balance:', balance.free.toString());
    
    // Get validator set
    const validators = await api.query.session.validators();
    console.log('Validators:', validators.toString());
}
```

## Smart Contract Deployment

### 1. Deploy Contract
```typescript
async function deployContract(api: ApiPromise, deployer: KeyringPair) {
    // Upload code
    const { code, abi } = await getContractArtifacts();
    const uploadTx = api.tx.contracts.uploadCode(code, null);
    const { codeHash } = await uploadTx.signAndSend(deployer);
    
    // Instantiate contract
    const instantiateTx = api.tx.contracts.instantiate(
        1000000000000,  // endowment
        1000000000,     // gas limit
        codeHash,
        abi.constructors[0].toU8a([]),  // constructor
        null            // salt
    );
    
    const { contract } = await instantiateTx.signAndSend(deployer);
    
    console.log('Contract address:', contract.address.toString());
}
```

### 2. Interact with Contract
```typescript
async function interactWithContract(
    api: ApiPromise,
    caller: KeyringPair,
    contractAddress: string
) {
    // Get contract
    const contract = new ContractPromise(
        api,
        abi,
        contractAddress
    );
    
    // Call contract method
    const { result, output } = await contract.query.getValue(
        caller.address,
        { value: 0, gasLimit: -1 }
    );
    
    console.log('Contract value:', output.toString());
    
    // Execute contract transaction
    const tx = await contract.tx.setValue(
        { value: 0, gasLimit: -1 },
        123
    );
    
    await tx.signAndSend(caller);
}
```

## Network Monitoring

### 1. Subscribe to Events
```typescript
async function subscribeEvents(api: ApiPromise) {
    // Subscribe to new blocks
    await api.rpc.chain.subscribeNewHeads((header) => {
        console.log('New block:', header.number.toNumber());
    });
    
    // Subscribe to system events
    await api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            console.log('Event:', event.section, event.method);
        });
    });
}
```

### 2. Monitor Node Status
```typescript
async function monitorNode(api: ApiPromise) {
    // Get node info
    const [peers, health] = await Promise.all([
        api.rpc.system.peers(),
        api.rpc.system.health()
    ]);
    
    console.log('Connected peers:', peers.length);
    console.log('Node health:', health.toString());
    
    // Monitor sync status
    await api.rpc.system.syncState((syncState) => {
        console.log('Sync status:', syncState.currentBlock.toString());
    });
}
```

## Next Steps

1. Explore [Developer Documentation](../04-guides/developer/INDEX.md)
2. Learn about [Network Architecture](03-ARCHITECTURE.md)
3. Understand [Tokenomics](../03-tokenomics/01-OVERVIEW.md)
4. Join the [Community](https://t.me/selendra_official)
