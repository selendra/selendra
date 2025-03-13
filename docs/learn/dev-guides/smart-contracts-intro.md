# Introduction to Smart Contracts on Selendra

Selendra offers developers the flexibility to build smart contracts using both EVM (Ethereum Virtual Machine) and WebAssembly (WASM) environments. This guide provides an overview of smart contract development on Selendra and helps you choose the right approach for your project.

## What Are Smart Contracts?

Smart contracts are self-executing programs that run on a blockchain. They automatically enforce and execute the terms of an agreement when predefined conditions are met. Key characteristics include:

- **Immutability**: Once deployed, the code cannot be changed (unless specifically designed to be upgradeable)
- **Transparency**: The code is visible to all participants on the blockchain
- **Automation**: Execution happens automatically when conditions are met
- **Trustlessness**: No need for intermediaries to ensure execution

## Smart Contract Options on Selendra

Selendra supports two types of smart contract environments:

### 1. EVM Smart Contracts

The Ethereum Virtual Machine (EVM) compatibility layer allows you to deploy and run Solidity smart contracts just like on Ethereum.

**Advantages:**
- Direct compatibility with Ethereum tooling (Hardhat, Truffle, Remix)
- Ability to port existing Ethereum contracts with minimal changes
- Large ecosystem of libraries and development resources
- Familiar development experience for Ethereum developers

**Ideal for:**
- Projects migrating from Ethereum
- Developers already familiar with Solidity
- Applications needing Ethereum ecosystem compatibility

### 2. WebAssembly (WASM) Smart Contracts

Using ink!, a Rust-based smart contract language optimized for Substrate blockchains.

**Advantages:**
- Better performance in many cases
- Stronger type safety due to Rust's compiler
- Deep integration with Substrate's native features
- Growing ecosystem within the Polkadot/Kusama ecosystem

**Ideal for:**
- Projects starting fresh on Selendra
- Developers with Rust experience
- Applications requiring advanced Substrate-specific features

## Key Differences Between EVM and WASM

| Feature | EVM (Solidity) | WASM (ink!) |
|---------|---------------|-------------|
| Language | Solidity, Vyper | Rust |
| Tooling | Mature (Hardhat, Truffle, etc.) | Emerging |
| Gas Model | Ethereum-compatible | Weight-based |
| Performance | Standard | Higher efficiency |
| Ecosystem | Large, established | Growing |
| Learning Curve | Moderate | Steeper (requires Rust knowledge) |

## Getting Started with Smart Contracts

### For EVM Development:

1. Set up your environment with Node.js and Hardhat
2. Write Solidity contracts
3. Connect to Selendra's EVM endpoint
4. Deploy and interact using standard Ethereum tools

See our [EVM Smart Contracts Guide](./evm-contracts.md) for detailed instructions.

### For WASM Development:

1. Set up your environment with Rust and ink!
2. Write contracts using ink! macros
3. Compile to WebAssembly
4. Deploy using Polkadot.js or similar tools

See our [WASM Contracts Guide](./wasm-contracts.md) for detailed instructions.

## Smart Contract Best Practices

Regardless of which environment you choose, follow these best practices:

1. **Security First**: Always audit your code and consider potential vulnerabilities
2. **Gas Optimization**: Optimize your code to minimize transaction costs
3. **Testing**: Write comprehensive tests covering various scenarios
4. **Upgradability**: Consider upgrade patterns for long-lived contracts
5. **Documentation**: Document your code thoroughly for future developers

## Example: Simple Storage Contract

### EVM (Solidity) Version

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleStorage {
    uint256 private storedData;

    function set(uint256 x) public {
        storedData = x;
    }

    function get() public view returns (uint256) {
        return storedData;
    }
}
```

### WASM (ink!) Version

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod simple_storage {
    #[ink(storage)]
    pub struct SimpleStorage {
        stored_data: u32,
    }

    impl SimpleStorage {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { stored_data: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        #[ink(message)]
        pub fn set(&mut self, new_value: u32) {
            self.stored_data = new_value;
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.stored_data
        }
    }
}
```

## Next Steps

- Learn how to [develop and deploy EVM contracts](./evm-contracts.md)
- Explore [WebAssembly contracts with ink!](./wasm-contracts.md)
- Understand how to [test your smart contracts effectively](./contract-testing.md) 