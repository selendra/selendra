# Developing WebAssembly Smart Contracts with ink! on Selendra

This guide introduces WebAssembly (WASM) smart contract development using ink! on Selendra. You'll learn how to write, test, deploy, and interact with ink! contracts.

## Introduction to WebAssembly Smart Contracts

WebAssembly (WASM) is a binary instruction format designed as a portable compilation target for programming languages. On Selendra, WASM contracts offer several advantages:

- **Performance**: Near-native execution speed
- **Efficiency**: Lower resource consumption
- **Language Flexibility**: Multiple languages can compile to WASM
- **Security**: Memory-safe execution environment
- **Determinism**: Consistent execution across all nodes

ink! is a Rust-based eDSL (embedded Domain Specific Language) for writing smart contracts specifically designed for Substrate-based blockchains like Selendra.

## Why Use ink! on Selendra?

- **Rust Security**: Benefit from Rust's memory safety and ownership model
- **Performance**: Efficient execution with lower fees compared to EVM contracts
- **Native Integration**: Built specifically for Substrate-based chains
- **Modern Development**: Use Rust's robust tooling and ecosystem
- **Interoperability**: Interact with other Substrate pallets and runtime modules

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed (stable version 1.69+)
- Rust WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [cargo-contract](https://github.com/paritytech/cargo-contract) CLI tool
- Basic knowledge of Rust programming language
- Familiarity with blockchain concepts
- A Selendra wallet with test tokens (for deploying)

## Setting Up Your Development Environment

### Install Rust and WebAssembly Support

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Update Rust
rustup update
```

### Install cargo-contract CLI Tool

```bash
# Install dependencies
sudo apt install build-essential pkg-config libssl-dev

# Install cargo-contract
cargo install cargo-contract --force
```

### Set Up a Local Selendra Node (Optional)

For local development and testing:

```bash
# Pull and run the Selendra node in development mode
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external
```

## Creating Your First ink! Contract

### Initialize a New Contract Project

```bash
# Create a new contract project
cargo contract new flipper

# Navigate to project directory
cd flipper
```

This creates a basic project structure:

```
flipper/
├── .gitignore
├── Cargo.toml
├── lib.rs
└── .cargo/
    └── config.toml
```

### Understanding the Default Contract

The generated contract is a simple "Flipper" that stores a boolean value and allows toggling it:

```rust
#[ink::contract]
mod flipper {
    /// The storage of the flipper contract.
    #[ink(storage)]
    pub struct Flipper {
        /// The single value stored in the contract.
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Returns the current value.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Flips the current value.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// Testing in this way has the benefit that you can use the standard Rust test runner
    /// and its features such as parallelization, filtering and so on.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
```

### Build Your Contract

```bash
# Build your contract
cargo contract build
```

This produces several files in the `target/ink` directory:
- `flipper.contract`: The complete contract bundle for deployment
- `flipper.wasm`: The compiled WebAssembly binary
- `metadata.json`: Contract metadata (ABI, docs, etc.)

## Creating a Token Contract

Let's create a more useful contract - a simple fungible token:

```rust
#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// ERC-20 error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Insufficient balance for operation
        InsufficientBalance,
        /// Insufficient allowance for operation
        InsufficientAllowance,
    }

    /// Type alias for the contract's result type
    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply
        total_supply: Balance,
        /// Mapping from owner to balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping of allowances: (owner, spender) -> allowed
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
        /// Token decimals
        decimals: u8,
    }

    /// Event emitted when a token transfer occurs
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &initial_supply);
            
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            
            Self {
                total_supply: initial_supply,
                balances,
                allowances: Mapping::default(),
                name,
                symbol,
                decimals,
            }
        }

        /// Returns the token name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Returns the token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Returns the token decimals
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }

        /// Returns the total token supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Transfers token from the caller to the given recipient
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            
            Ok(())
        }

        /// Returns the amount which `spender` is allowed to withdraw from `owner`
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers tokens from one account to another using the allowance mechanism
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            
            self.transfer_from_to(from, to, value)?;
            
            let new_allowance = allowance - value;
            self.allowances.insert((from, caller), &new_allowance);
            
            Ok(())
        }

        /// Transfers token from the sender to the given recipient
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::*, DefaultEnvironment as Env};

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(
                1000,
                String::from("TestToken"),
                String::from("TT"),
                18,
            );
            
            assert_eq!(contract.total_supply(), 1000);
            assert_eq!(contract.name(), String::from("TestToken"));
            assert_eq!(contract.symbol(), String::from("TT"));
            assert_eq!(contract.decimals(), 18);
            
            let accounts = default_accounts::<Env>();
            assert_eq!(contract.balance_of(accounts.alice), 1000);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(
                1000,
                String::from("TestToken"),
                String::from("TT"),
                18,
            );
            
            let accounts = default_accounts::<Env>();
            
            assert_eq!(contract.balance_of(accounts.alice), 1000);
            assert_eq!(contract.balance_of(accounts.bob), 0);
            
            // Transfer from Alice to Bob
            assert_eq!(contract.transfer(accounts.bob, 100), Ok(()));
            
            // Check the updated balances
            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            // Create the contract with an initial supply of 100 tokens
            let mut contract = Erc20::new(
                100,
                String::from("TestToken"),
                String::from("TT"),
                18,
            );
            
            let accounts = default_accounts::<Env>();
            
            // Try to transfer more than Alice has
            assert_eq!(
                contract.transfer(accounts.bob, 200),
                Err(Error::InsufficientBalance)
            );
            
            // Balances should remain unchanged
            assert_eq!(contract.balance_of(accounts.alice), 100);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn approve_and_transfer_from_works() {
            // Create the contract with an initial supply of 1000 tokens
            let mut contract = Erc20::new(
                1000,
                String::from("TestToken"),
                String::from("TT"),
                18,
            );
            
            let accounts = default_accounts::<Env>();
            
            // Alice approves Bob to spend 100 tokens
            assert_eq!(contract.approve(accounts.bob, 100), Ok(()));
            
            // Check the allowance
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 100);
            
            // Set Bob as the caller
            set_caller::<Env>(accounts.bob);
            
            // Bob transfers 50 tokens from Alice to Charlie
            assert_eq!(
                contract.transfer_from(accounts.alice, accounts.charlie, 50),
                Ok(())
            );
            
            // Check the updated balances
            assert_eq!(contract.balance_of(accounts.alice), 950);
            assert_eq!(contract.balance_of(accounts.charlie), 50);
            
            // Check the updated allowance
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 50);
        }
    }
}
```

## Testing ink! Contracts

ink! contracts can be tested using Rust's native testing framework and the ink! testing utilities.

### Unit Testing

Unit tests are defined within your contract code using the `#[cfg(test)]` annotation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::{test::*, DefaultEnvironment as Env};

    #[ink::test]
    fn test_constructor() {
        // Test contract constructor
    }

    #[ink::test]
    fn test_transfer() {
        // Test transfer functionality
    }
}
```

Run your tests with:

```bash
cargo test
```

### End-to-End (E2E) Testing

For more complex scenarios, you can use `ink_e2e` for end-to-end tests:

1. Add the dependency to your `Cargo.toml`:

```toml
[dev-dependencies]
ink_e2e = "4.2.0"
```

2. Create E2E tests:

```rust
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::build_message;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn e2e_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Arrange
        let constructor = Erc20Ref::new(
            1000,
            "TestToken".to_string(),
            "TT".to_string(),
            18,
        );
        
        let contract_acc_id = client
            .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Act - transfer to Bob
        let transfer = build_message::<Erc20Ref>(contract_acc_id.clone())
            .call(|contract| contract.transfer(ink_e2e::bob(), 100));
            
        let transfer_result = client
            .call(&ink_e2e::alice(), transfer, 0, None)
            .await
            .expect("transfer failed");

        // Assert - check Bob's balance
        let balance_of = build_message::<Erc20Ref>(contract_acc_id.clone())
            .call(|contract| contract.balance_of(ink_e2e::bob()));
            
        let balance = client
            .call_dry_run(&ink_e2e::alice(), &balance_of, 0, None)
            .await
            .return_value();

        assert_eq!(balance, 100);

        Ok(())
    }
}
```

3. Run E2E tests:

```bash
cargo test --features e2e-tests
```

## Deploying ink! Contracts to Selendra

### Prepare Your Contract for Deployment

Build an optimized version of your contract:

```bash
cargo contract build --release
```

### Deploying Using Contracts UI

1. Start your local Selendra node or connect to a testnet
2. Visit [Contracts UI](https://contracts-ui.substrate.io/)
3. Connect to your local node or Selendra testnet
4. Click "Upload & Deploy Contract"
5. Upload the `.contract` file from your `target/ink` directory
6. Enter the constructor parameters
7. Submit the transaction and confirm deployment

### Deploying Using cargo-contract CLI

You can also deploy directly using the command line:

```bash
# Deploy to local node
cargo contract instantiate \
  --suri "//Alice" \
  --constructor new \
  --args 1000000 "MyToken" "MT" 18 \
  --url "ws://127.0.0.1:9944"
```

## Interacting with Deployed Contracts

### Using Contracts UI

1. Go to [Contracts UI](https://contracts-ui.substrate.io/)
2. Connect to your Selendra node
3. Find your contract in the "My Contracts" section
4. Use the UI to call contract functions and view results

### Using ink! Client Library

You can interact with your contracts programmatically using the ink! client library:

```rust
use ink_client::{Client, ContractAt, Environment};
use codec::Encode;

async fn interact_with_contract() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a node
    let client = Client::new("ws://127.0.0.1:9944").await?;
    
    // Get contract reference
    let contract = client.instantiated_contract(
        "0x123...".parse()?, // contract address
        include_bytes!("../metadata.json"), // contract metadata
    );
    
    // Call a message
    let result = contract
        .call("transfer")
        .args(("0x456...".parse()?, 100u128))
        .send_with_signer(&signer)
        .await?;
    
    println!("Transfer result: {:?}", result);
    
    Ok(())
}
```

## Advanced ink! Contract Development

### Contract Storage Patterns

ink! provides several storage primitives:

- `ink::storage::Value<T>` - for single values
- `ink::storage::Mapping<K, V>` - for key-value pairs
- `ink::storage::Vec<T>` - for dynamic arrays
- `ink::storage::LazyHashMap<K, V>` - for lazily loaded key-value pairs

Example of complex storage:

```rust
#[ink(storage)]
pub struct ComplexContract {
    // Single value
    admin: AccountId,
    
    // Mapping from account to balance
    balances: Mapping<AccountId, Balance>,
    
    // Nested mapping for allowances
    allowances: Mapping<(AccountId, AccountId), Balance>,
    
    // Dynamic array
    token_holders: Vec<AccountId>,
    
    // Lazy loading large data
    metadata: LazyHashMap<TokenId, TokenMetadata>,
}
```

### Contract Inheritance with Traits

Use Rust traits to implement contract inheritance or shared functionality:

```rust
pub trait Ownable {
    /// Returns the current owner
    fn owner(&self) -> AccountId;
    
    /// Transfers ownership to a new account
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()>;
    
    /// Checks if the caller is the owner
    fn ensure_owner(&self) -> Result<()>;
}

impl Ownable for MyContract {
    fn owner(&self) -> AccountId {
        self.owner
    }
    
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
        self.ensure_owner()?;
        self.owner = new_owner;
        Ok(())
    }
    
    fn ensure_owner(&self) -> Result<()> {
        if self.env().caller() != self.owner {
            return Err(Error::NotOwner);
        }
        Ok(())
    }
}
```

### Cross-Contract Calls

ink! supports calling other contracts from within your contract:

```rust
#[ink(message)]
pub fn call_other_contract(&mut self) -> Result<()> {
    // Get a reference to another contract
    let other_contract_address: AccountId = self.other_contract_address;
    
    let other_contract = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
        .call(other_contract_address)
        .gas_limit(0)
        .transferred_value(0)
        .exec_input(
            ink::env::call::ExecutionInput::new(ink::env::call::Selector::new([0x13, 0x37, 0x13, 0x37]))
            .push_arg(42u32)
        )
        .returns::<Result<()>>();
    
    other_contract
}
```

### Handling Contract Upgrades

Contract upgradeability can be implemented using proxy patterns:

```rust
#[ink(storage)]
pub struct Proxy {
    /// The implementation contract address
    implementation: AccountId,
    /// The contract owner
    owner: AccountId,
}

impl Proxy {
    #[ink(constructor)]
    pub fn new(implementation: AccountId) -> Self {
        Self {
            implementation,
            owner: Self::env().caller(),
        }
    }
    
    #[ink(message)]
    pub fn upgrade(&mut self, new_implementation: AccountId) -> Result<()> {
        if self.env().caller() != self.owner {
            return Err(Error::NotOwner);
        }
        self.implementation = new_implementation;
        Ok(())
    }
    
    #[ink(message, payable, selector = _)]
    pub fn forward(&self) -> Result<()> {
        // Forward call to the implementation contract
        ink::env::call::build_call::<ink::env::DefaultEnvironment>()
            .call(self.implementation)
            .transferred_value(self.env().transferred_value())
            .call_flags(ink::env::CallFlags::default().set_forward_input(true).set_tail_call(true))
            .invoke();
        
        // This code is never reached due to tail call optimization
        unreachable!()
    }
}
```

## Best Practices for ink! Smart Contracts

1. **Keep Contracts Simple**: Single responsibility principle
2. **Optimize for Gas**: Minimize storage operations
3. **Secure Contract Access**: Implement proper access control
4. **Handle Errors Gracefully**: Return meaningful errors
5. **Thoroughly Test Contracts**: Unit, integration, and property tests
6. **Document Your Code**: Use clear comments and documentation
7. **Follow Rust Best Practices**: Leverage type safety and ownership
8. **Minimize Attack Surface**: Avoid complex logic when possible
9. **Use Well-Tested Libraries**: Don't reinvent the wheel
10. **Consider Upgradeability**: Plan for future changes

## Common Issues and Troubleshooting

### Storage Errors

```
error[E0277]: the trait bound `MyStruct: scale::Encode` is not satisfied
```

Ensure all stored types implement the necessary traits:

```rust
#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MyStruct {
    field1: u32,
    field2: bool,
}
```

### Gas Estimation Failures

If you encounter gas estimation failures:

- Reduce contract complexity
- Break operations into smaller transactions
- Ensure state changes are minimal
- Check for infinite loops

### Contract Too Large

If your contract exceeds size limits:

- Remove unnecessary functions
- Optimize storage patterns
- Split into multiple contracts
- Reduce dependencies
- Use Rust compiler optimizations

## Selendra-Specific Considerations

- **Chain Extensions**: Use Selendra-specific extensions for additional functionality
- **Gas Limits**: Be aware of Selendra's gas limits
- **XCM Integration**: Consider cross-chain message passing in your design
- **Compatibility**: Test thoroughly on Selendra testnet before mainnet deployment

## Next Steps

- Explore the [ink! documentation](https://use.ink/)
- Check out [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts) for ink! contract standards
- Learn about [testing smart contracts](./contract-testing.md) in detail
- Try building [your first dApp](./first-dapp.md) with ink! contracts
- Dive into [Selendra's API reference](https://docs.selendra.org/api) for advanced integrations 