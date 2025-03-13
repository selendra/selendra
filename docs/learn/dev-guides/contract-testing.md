# Testing Smart Contracts on Selendra

Effective testing is crucial for smart contract development to ensure security, correctness, and reliability. This guide covers comprehensive testing strategies for both EVM and WebAssembly contracts on Selendra.

## Why Testing is Critical for Smart Contracts

Smart contracts have unique characteristics that make testing particularly important:

1. **Immutability**: Once deployed, contracts can't be easily modified
2. **Financial Impact**: Bugs can lead to significant financial losses
3. **Public Execution**: Vulnerabilities are exposed to the entire network
4. **Complex State**: Contract behavior depends on the blockchain state

## Testing Approaches for Smart Contracts

### 1. Unit Testing

Testing individual functions and components in isolation.

**Benefits**:
- Fast execution
- Easy to isolate issues
- High coverage

**When to use**:
- During development
- When implementing new features
- After refactoring

### 2. Integration Testing

Testing interactions between components, such as multiple contracts working together.

**Benefits**:
- Identifies interface issues
- Tests realistic usage patterns
- Verifies cross-contract calls

**When to use**:
- After unit tests pass
- When multiple contracts interact
- When integrating with existing contracts

### 3. Property-Based Testing

Generates random inputs to discover edge cases.

**Benefits**:
- Discovers unexpected issues
- Tests a wide range of inputs
- Finds edge cases

**When to use**:
- For complex mathematical functions
- For systems with many possible states
- When security is paramount

### 4. Fuzz Testing

Similar to property-based testing but specifically targeting security vulnerabilities.

**Benefits**:
- Finds security holes
- Tests extreme inputs
- Discovers denial-of-service vulnerabilities

**When to use**:
- For financial contracts
- Before audits
- When high security is required

## Testing EVM (Solidity) Contracts

### Setting Up the Test Environment

For Solidity contracts, we recommend using Hardhat for testing:

```bash
# Install Hardhat and dependencies
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox chai ethers
```

Configure Hardhat to connect to Selendra's networks:

```javascript
// hardhat.config.js
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.18",
  networks: {
    selendraLocal: {
      url: "http://127.0.0.1:9933",
      accounts: [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
      ],
      chainId: 1994
    },
    // Add testnet and mainnet as needed
  }
};
```

### Writing Unit Tests

Create test files in the `test` directory:

```javascript
// test/Token.test.js
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Token Contract", function () {
  let Token;
  let token;
  let owner;
  let addr1;
  let addr2;
  let addrs;

  beforeEach(async function () {
    // Get signers (accounts)
    [owner, addr1, addr2, ...addrs] = await ethers.getSigners();

    // Deploy the contract
    Token = await ethers.getContractFactory("Token");
    token = await Token.deploy("Test Token", "TEST", 1000000);
    await token.deployed();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await token.owner()).to.equal(owner.address);
    });

    it("Should assign the total supply to the owner", async function () {
      const ownerBalance = await token.balanceOf(owner.address);
      expect(await token.totalSupply()).to.equal(ownerBalance);
    });
  });

  describe("Transactions", function () {
    it("Should transfer tokens between accounts", async function () {
      // Transfer 50 tokens from owner to addr1
      await token.transfer(addr1.address, 50);
      expect(await token.balanceOf(addr1.address)).to.equal(50);

      // Transfer 50 tokens from addr1 to addr2
      await token.connect(addr1).transfer(addr2.address, 50);
      expect(await token.balanceOf(addr2.address)).to.equal(50);
    });

    it("Should fail if sender doesn't have enough tokens", async function () {
      const initialOwnerBalance = await token.balanceOf(owner.address);

      // Try to send more tokens than available
      await expect(
        token.connect(addr1).transfer(owner.address, 1)
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");

      // Owner balance shouldn't have changed
      expect(await token.balanceOf(owner.address)).to.equal(initialOwnerBalance);
    });
  });
});
```

### Running Tests

```bash
npx hardhat test
```

For more detailed output:

```bash
npx hardhat test --verbose
```

### Testing with Local Selendra Node

For more realistic testing against a local Selendra node:

```bash
# Start a local Selendra node
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external

# Run tests against the local node
npx hardhat test --network selendraLocal
```

### Advanced EVM Testing Techniques

#### 1. Time Manipulation

Testing time-dependent contracts:

```javascript
describe("Time-dependent features", function () {
  it("Should allow withdrawal after lock period", async function () {
    await token.lock(100, 86400); // Lock 100 tokens for 1 day

    // Fast-forward time by 1 day
    await network.provider.send("evm_increaseTime", [86401]);
    await network.provider.send("evm_mine");

    // Now should be able to withdraw
    await token.unlock();
    expect(await token.lockedAmount()).to.equal(0);
  });
});
```

#### 2. Snapshot and Revert

Testing multiple scenarios from the same state:

```javascript
describe("Multiple scenarios", function () {
  let snapshotId;

  beforeEach(async function () {
    snapshotId = await network.provider.send("evm_snapshot");
  });

  afterEach(async function () {
    await network.provider.send("evm_revert", [snapshotId]);
  });

  it("Scenario 1", async function () {
    // Test something
  });

  it("Scenario 2", async function () {
    // Test something else from the same initial state
  });
});
```

#### 3. Gas Usage Analysis

Monitoring gas usage:

```javascript
it("Should use reasonable gas", async function () {
  const tx = await token.transfer(addr1.address, 100);
  const receipt = await tx.wait();
  
  console.log("Gas used:", receipt.gasUsed.toNumber());
  expect(receipt.gasUsed.toNumber()).to.be.lessThan(100000);
});
```

#### 4. Event Testing

Verifying events are emitted correctly:

```javascript
it("Should emit Transfer event", async function () {
  await expect(token.transfer(addr1.address, 50))
    .to.emit(token, "Transfer")
    .withArgs(owner.address, addr1.address, 50);
});
```

## Testing WebAssembly (ink!) Contracts

### Setting Up the Test Environment

For ink! contracts, we'll use the ink! testing framework:

```bash
# Make sure you have Rust installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Set up a new ink! project
cargo install cargo-contract
cargo contract new my_contract
cd my_contract
```

### Writing Unit Tests for ink! Contracts

In your ink! contract file, add unit tests at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;

    #[ink::test]
    fn default_works() {
        let contract = MyContract::default();
        assert_eq!(contract.get(), 0);
    }

    #[ink::test]
    fn it_works() {
        let mut contract = MyContract::new(42);
        assert_eq!(contract.get(), 42);
        contract.inc(5);
        assert_eq!(contract.get(), 47);
        contract.dec(42);
        assert_eq!(contract.get(), 5);
    }
}
```

### Running ink! Tests

```bash
cargo test
```

For more detailed output:

```bash
cargo test -- --nocapture
```

### Integration Testing for ink!

For more complex testing scenarios with ink!, use the `ink_e2e` crate:

```rust
#[cfg(test)]
mod e2e_tests {
    use super::*;
    use ink_e2e::*;

    #[ink_e2e::test]
    async fn e2e_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Arrange
        let constructor = MyContractRef::new(42);
        let contract_account_id = client
            .instantiate("my_contract", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Act
        let get = build_message::<MyContractRef>(contract_account_id.clone())
            .call(|contract| contract.get());
        let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;

        // Assert
        assert_eq!(get_result.return_value(), 42);

        Ok(())
    }
}
```

## Property-Based Testing

Property-based testing generates numerous test cases to find edge cases.

### For Solidity Contracts

Using the `hardhat-chai-matchers` with property-based testing:

```javascript
// Install dependencies
npm install --save-dev ethereum-waffle @nomiclabs/hardhat-waffle ethereum-waffle hardhat-chai-matchers

// In your test file
const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");
const { arbitraryValue } = require("hardhat-chai-matchers/internal/arbitrary");

describe("Property-based testing", function () {
  async function deployFixture() {
    const Token = await ethers.getContractFactory("Token");
    const token = await Token.deploy("Test Token", "TEST", 1000000);
    const [owner, addr1] = await ethers.getSigners();
    return { token, owner, addr1 };
  }

  it("should handle any valid transfer amount", async function () {
    for (let i = 0; i < 100; i++) {
      const { token, owner, addr1 } = await loadFixture(deployFixture);
      const randomAmount = arbitraryValue(1, 1000);
      
      await token.transfer(addr1.address, randomAmount);
      expect(await token.balanceOf(addr1.address)).to.equal(randomAmount);
    }
  });
});
```

### For ink! Contracts

Using proptest with ink!:

```bash
# Add proptest dependency
cargo add proptest --dev
```

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;
    use proptest::prelude::*;

    #[ink::test]
    fn test_addition_properties() {
        proptest!(|(a in 0..1000u32, b in 0..1000u32)| {
            let mut contract = MyContract::new(a);
            contract.inc(b);
            assert_eq!(contract.get(), a + b);
        });
    }
}
```

## Security-Focused Testing

When testing contracts that handle valuable assets, consider specialized security tests.

### Re-Entrancy Testing

```javascript
it("Should be protected against re-entrancy", async function () {
  // Deploy the malicious contract
  const Attacker = await ethers.getContractFactory("ReEntrancyAttacker");
  const attacker = await Attacker.deploy(vault.address);
  
  // Fund the vault
  await token.transfer(vault.address, 1000);
  
  // Attempt the attack
  await expect(
    attacker.attack(100)
  ).to.be.reverted;
  
  // Verify funds are safe
  expect(await token.balanceOf(vault.address)).to.equal(1000);
});
```

### Overflow/Underflow Testing

```javascript
it("Should handle overflow correctly", async function () {
  // For Solidity 0.8.x, this should revert
  await expect(
    contract.add(ethers.constants.MaxUint256, 1)
  ).to.be.reverted;
  
  // For Solidity <0.8.x without SafeMath, this would overflow
  // Check if your contract handles it properly
});
```

### Access Control Testing

```javascript
it("Should enforce access control", async function () {
  // Non-owner tries to call restricted function
  await expect(
    contract.connect(addr1).restrictedFunction()
  ).to.be.revertedWith("Ownable: caller is not the owner");
});
```

## Testing on Selendra Testnet

For end-to-end validation before mainnet deployment:

```javascript
// Update hardhat.config.js with testnet details
module.exports = {
  networks: {
    selendraTest: {
      url: "https://testnet.selendra.org",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 1994
    }
  }
};

// Run specific tests on testnet
npx hardhat test test/e2e.test.js --network selendraTest
```

## Continuous Integration for Smart Contracts

Set up CI/CD pipelines using GitHub Actions or similar tools:

```yaml
# .github/workflows/test.yml
name: Smart Contract Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Use Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '16'
      - name: Install dependencies
        run: npm ci
      - name: Run tests
        run: npx hardhat test
      - name: Run coverage
        run: npx hardhat coverage
```

## Code Coverage

Measuring test coverage for smart contracts:

```bash
# For Solidity
npm install --save-dev solidity-coverage
npx hardhat coverage

# For ink!
cargo install cargo-tarpaulin
cargo tarpaulin
```

## Best Practices for Smart Contract Testing

1. **Test All Public Functions**: Every public function should have tests
2. **Simulate Real-World Scenarios**: Test realistic usage patterns
3. **Stress Test with Edge Cases**: Test boundary conditions and limits
4. **Test Failure Modes**: Verify that functions fail correctly when they should
5. **Test Access Control**: Verify that permissions are enforced properly
6. **Use Both Unit and Integration Tests**: Combine different test approaches
7. **Test Gas Usage**: Monitor the gas efficiency of your contract
8. **Maintain Test Independence**: Tests should not depend on each other
9. **Use Test Fixtures**: Reset state between tests for consistency
10. **Regularly Update Tests**: Keep tests up-to-date with contract changes

## Common Testing Pitfalls

1. **Insufficient Test Coverage**: Missing important edge cases
2. **Shallow Testing**: Only testing the happy path
3. **Unrealistic Test Data**: Using data that doesn't reflect real usage
4. **Ignoring Error Cases**: Not testing how failures are handled
5. **Brittle Tests**: Tests that break with minor changes
6. **Slow Tests**: Tests that take too long to run
7. **Flaky Tests**: Tests that sometimes pass and sometimes fail
8. **Testing Internal Logic**: Over-testing implementation details
9. **Overlooking Gas Constraints**: Not considering gas limits
10. **Manual Testing Only**: Relying on manual verification rather than automated tests

## Testing Tools for Selendra Contracts

### For EVM (Solidity) Contracts

- **Hardhat**: Development environment and testing framework
- **Waffle**: Library for writing and testing smart contracts
- **Ethers.js**: Library for interacting with Ethereum
- **Solidity-Coverage**: Code coverage for Solidity smart contracts
- **Slither**: Static analysis framework
- **Mythril**: Security analysis tool

### For WASM (ink!) Contracts

- **ink! Testing Framework**: Built-in testing capabilities
- **ink! E2E**: End-to-end testing for ink! contracts
- **cargo-contract**: CLI utility for managing ink! contracts
- **Cargo Tarpaulin**: Code coverage for Rust

## Next Steps

- Learn about [EVM smart contracts](./evm-contracts.md) in more detail
- Explore [WebAssembly contracts](./wasm-contracts.md) with ink!
- Get started [building your first dApp](./first-dapp.md) with tested contracts 