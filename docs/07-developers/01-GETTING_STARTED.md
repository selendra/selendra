# Getting Started with Selendra Development

## Overview

This guide will help you set up your development environment and start building on Selendra. We'll cover:
- Development environment setup
- Local node deployment
- Smart contract development
- Testing and deployment
- Best practices

## Prerequisites

- Node.js v16.0.0 or later
- Rust v1.60.0 or later
- Docker v20.10.0 or later
- Git

## Environment Setup

### 1. Install Selendra CLI
```bash
# Install via npm
npm install -g @selendra/cli

# Verify installation
selendra --version
```

### 2. Install Development Dependencies
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# Install substrate dependencies
sudo apt update
sudo apt install -y cmake pkg-config libssl-dev git clang libclang-dev

# Install node dependencies
npm install -g yarn
```

### 3. Clone Selendra Node
```bash
# Clone repository
git clone https://github.com/selendra/selendra
cd selendra

# Install dependencies
cargo build --release
```

## Local Development Chain

### 1. Start Local Node
```bash
# Start development chain
./target/release/selendra --dev

# Start with persistent data
./target/release/selendra --dev --base-path /tmp/selendra-data
```

### 2. Configure Network
```toml
# config.toml
[network]
chain = "dev"
endpoint = "ws://127.0.0.1:9944"

[account]
seed = "//Alice"
```

### 3. Connect to Node
```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function connect() {
    // Create provider
    const provider = new WsProvider('ws://127.0.0.1:9944');
    
    // Create API instance
    const api = await ApiPromise.create({ provider });
    
    // Get chain info
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    
    console.log(`Connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    
    return api;
}
```

## Smart Contract Development

### 1. Project Setup
```bash
# Create new project
selendra init my-dapp
cd my-dapp

# Install dependencies
yarn install
```

### 2. Contract Structure
```
my-dapp/
├── contracts/
│   ├── lib.rs
│   └── Cargo.toml
├── tests/
│   └── contract.test.ts
├── scripts/
│   ├── deploy.ts
│   └── interact.ts
└── package.json
```

### 3. Basic Contract
```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod greeter {
    #[ink(storage)]
    pub struct Greeter {
        greeting: String,
    }
    
    impl Greeter {
        #[ink(constructor)]
        pub fn new(greeting: String) -> Self {
            Self { greeting }
        }
        
        #[ink(message)]
        pub fn greet(&self) -> String {
            self.greeting.clone()
        }
        
        #[ink(message)]
        pub fn set_greeting(&mut self, greeting: String) {
            self.greeting = greeting;
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_greeting() {
            let greeter = Greeter::new("Hello".into());
            assert_eq!(greeter.greet(), "Hello");
        }
    }
}
```

### 4. Build & Test
```bash
# Build contract
cargo +nightly contract build

# Run tests
cargo test
yarn test
```

## Development Tools

### 1. Selendra SDK
```typescript
import { Selendra } from '@selendra/sdk';

async function initSDK() {
    const sdk = new Selendra({
        nodeUrl: 'ws://127.0.0.1:9944',
        networkType: 'development'
    });
    
    await sdk.init();
    return sdk;
}
```

### 2. Contract Interaction
```typescript
async function interactWithContract() {
    const sdk = await initSDK();
    
    // Deploy contract
    const contract = await sdk.deployContract({
        name: 'Greeter',
        constructorArgs: ['Hello Selendra']
    });
    
    // Call contract
    const result = await contract.query.greet();
    console.log('Greeting:', result.toString());
    
    // Send transaction
    await contract.tx.setGreeting('New Greeting')
        .signAndSend(account);
}
```

### 3. Event Handling
```typescript
async function handleEvents() {
    const sdk = await initSDK();
    
    // Subscribe to events
    sdk.api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            
            if (event.section === 'contracts') {
                console.log('Contract event:', event.data.toString());
            }
        });
    });
}
```

## Best Practices

### 1. Security Guidelines

#### Contract Security
```rust
// Use safe math operations
use ink_env::DefaultEnvironment;
use ink_prelude::collections::HashMap;

#[ink(storage)]
pub struct Token {
    balances: HashMap<AccountId, Balance>,
    total_supply: Balance,
}

impl Token {
    // Check for overflows
    pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
        let from_balance = self.balance_of(self.env().caller());
        ensure!(from_balance >= value, Error::InsufficientBalance);
        
        // Safe math operations
        self.balances.insert(self.env().caller(), from_balance - value);
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + value);
        
        Ok(())
    }
}
```

#### Access Control
```rust
#[ink(storage)]
pub struct Contract {
    owner: AccountId,
    admins: HashMap<AccountId, bool>,
}

impl Contract {
    #[ink(message)]
    pub fn admin_only(&self) -> Result<()> {
        ensure!(
            self.admins.contains_key(&self.env().caller()),
            Error::NotAuthorized
        );
        Ok(())
    }
}
```

### 2. Gas Optimization

#### Storage Optimization
```rust
#[ink(storage)]
pub struct OptimizedContract {
    // Use compact encoding for small numbers
    #[ink(storage_field)]
    counter: u32,
    
    // Use hashmaps for sparse data
    #[ink(storage_field)]
    balances: HashMap<AccountId, Balance>,
}
```

#### Computation Optimization
```rust
impl OptimizedContract {
    // Cache frequently accessed values
    #[ink(message)]
    pub fn complex_operation(&self) -> Result<()> {
        let caller = self.env().caller();
        let balance = self.balances.get(&caller)
            .ok_or(Error::NoBalance)?;
            
        // Batch operations
        if balance >= 100 {
            self.do_batch_operation(caller, balance)?;
        }
        
        Ok(())
    }
}
```

### 3. Testing Guidelines

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[ink::test]
    fn test_transfer() {
        let mut contract = Contract::new();
        let accounts = ink_env::test::default_accounts::<Environment>();
        
        // Test transfer
        assert_ok!(contract.transfer(accounts.bob, 100));
        assert_eq!(contract.balance_of(accounts.bob), 100);
    }
}
```

#### Integration Tests
```typescript
import { ApiPromise } from '@polkadot/api';
import { Contract } from '@selendra/sdk';

describe('Contract Integration', () => {
    let api: ApiPromise;
    let contract: Contract;
    
    beforeAll(async () => {
        api = await initAPI();
        contract = await deployContract();
    });
    
    it('should handle complex scenario', async () => {
        // Setup
        await setupTestData();
        
        // Execute
        const result = await contract.tx.complexOperation()
            .signAndSend(account);
            
        // Verify
        expect(result.status.isFinalized).toBe(true);
        
        const state = await contract.query.getState();
        expect(state.toNumber()).toBe(expectedValue);
    });
});
```

## Deployment

### 1. Mainnet Deployment
```typescript
async function deployToMainnet() {
    const sdk = await initSDK({
        nodeUrl: 'wss://mainnet.selendra.org',
        networkType: 'mainnet'
    });
    
    // Deploy contract
    const contract = await sdk.deployContract({
        name: 'MyContract',
        constructorArgs: [...args],
        gasLimit: 1000000
    });
    
    console.log('Contract deployed at:', contract.address);
    return contract;
}
```

### 2. Contract Verification
```typescript
async function verifyContract() {
    const sdk = await initSDK();
    
    // Verify contract
    await sdk.verifyContract({
        address: contractAddress,
        name: 'MyContract',
        compiler: 'rust-contract',
        version: '3.3.0',
        constructorArgs: [...args],
        source: './contracts/lib.rs'
    });
}
```

### 3. Monitoring
```typescript
async function monitorContract() {
    const sdk = await initSDK();
    
    // Monitor events
    sdk.api.query.system.events((events) => {
        events.forEach(({ event }) => {
            if (event.section === 'contracts') {
                // Log event
                console.log('Contract event:', {
                    section: event.section,
                    method: event.method,
                    data: event.data.toString()
                });
                
                // Alert if needed
                if (event.method === 'ContractExecution') {
                    alertDevOps('Contract execution completed');
                }
            }
        });
    });
}
```
