# EVM Smart Contracts on Selendra

This guide provides a deep dive into developing and deploying Ethereum Virtual Machine (EVM) smart contracts on Selendra. By the end, you'll understand how to write, test, deploy, and interact with Solidity contracts in the Selendra environment.

## Why Use EVM on Selendra?

Selendra's EVM compatibility layer offers several advantages:

- Full compatibility with Ethereum smart contracts
- Lower transaction fees than Ethereum mainnet
- Faster block times (1 second) and quicker finality (2-3 seconds)
- Seamless integration with existing Ethereum developer tools
- The security and scalability of Substrate framework

## Prerequisites

Before starting, make sure you have:
- Set up your [development environment](./dev-environment-setup.md)
- Basic knowledge of Solidity
- Familiarity with Hardhat or Truffle development frameworks

## Setting Up an EVM Project

### 1. Project Initialization

The easiest way to start a new EVM project on Selendra is using Hardhat:

```bash
# Create a new directory
mkdir selendra-evm-project
cd selendra-evm-project

# Initialize a new npm project
npm init -y

# Install Hardhat and dependencies
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox ethers
```

### 2. Initialize Hardhat

```bash
npx hardhat init
```

Choose "Create a JavaScript project" when prompted.

### 3. Configure Hardhat for Selendra

Edit the `hardhat.config.js` file:

```javascript
require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.18",
  networks: {
    selendraLocal: {
      url: "http://127.0.0.1:9933",
      accounts: [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" // Development account private key
      ],
      chainId: 1994
    },
    selendraTest: {
      url: "https://testnet.selendra.org",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 1994
    },
    selendraMain: {
      url: "https://mainnet.selendra.org",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 1994
    }
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  }
};
```

## Writing Solidity Smart Contracts

Selendra supports Solidity contracts up to the latest compiler version. Let's create a simple token contract as an example.

### Creating a Basic Token Contract

Create a new file `contracts/SelendraToken.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SelendraToken is ERC20, Ownable {
    constructor(uint256 initialSupply) ERC20("Selendra Test Token", "STT") {
        _mint(msg.sender, initialSupply * 10 ** decimals());
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

To use OpenZeppelin contracts:

```bash
npm install @openzeppelin/contracts
```

### Solidity Best Practices for Selendra

Follow these best practices when writing Solidity contracts for Selendra:

1. **Gas Optimization**: Selendra has a similar gas model to Ethereum, so optimize your contracts for gas efficiency:
   - Use appropriate data types (uint256 is often more efficient than smaller sizes)
   - Batch operations when possible
   - Minimize on-chain storage

2. **Security Considerations**:
   - Use the latest Solidity compiler (0.8.x) for built-in overflow protection
   - Follow checks-effects-interactions pattern to prevent reentrancy attacks
   - Use OpenZeppelin's audited contracts for standard functionality

3. **Selendra-Specific Considerations**:
   - Selendra's block time is faster (1 second), so time-based logic might need adjustment
   - Default gas values may need adjustment due to computational differences

## Compiling and Testing Smart Contracts

### Compiling Contracts

```bash
npx hardhat compile
```

This will compile your contracts and generate artifacts in the `artifacts` directory.

### Writing Tests

Create a test file `test/SelendraToken.js`:

```javascript
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SelendraToken", function () {
  let token;
  let owner;
  let addr1;
  let addr2;
  let addrs;

  beforeEach(async function () {
    [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
    
    const TokenFactory = await ethers.getContractFactory("SelendraToken");
    token = await TokenFactory.deploy(1000000); // 1 million tokens
    await token.deployed();
  });

  describe("Deployment", function () {
    it("Should assign the total supply to the owner", async function () {
      const ownerBalance = await token.balanceOf(owner.address);
      expect(await token.totalSupply()).to.equal(ownerBalance);
    });
    
    it("Should set the correct token name and symbol", async function () {
      expect(await token.name()).to.equal("Selendra Test Token");
      expect(await token.symbol()).to.equal("STT");
    });
  });

  describe("Transactions", function () {
    it("Should transfer tokens between accounts", async function () {
      // Transfer 50 tokens from owner to addr1
      await token.transfer(addr1.address, 50);
      const addr1Balance = await token.balanceOf(addr1.address);
      expect(addr1Balance).to.equal(50);
      
      // Transfer 50 tokens from addr1 to addr2
      await token.connect(addr1).transfer(addr2.address, 50);
      const addr2Balance = await token.balanceOf(addr2.address);
      expect(addr2Balance).to.equal(50);
    });
    
    it("Should fail if sender doesn't have enough tokens", async function () {
      const initialOwnerBalance = await token.balanceOf(owner.address);
      
      // Try to send 1 token from addr1 (0 tokens) to owner
      await expect(
        token.connect(addr1).transfer(owner.address, 1)
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
      
      // Owner balance shouldn't have changed
      expect(await token.balanceOf(owner.address)).to.equal(initialOwnerBalance);
    });
  });
  
  describe("Minting", function () {
    it("Should allow owner to mint new tokens", async function () {
      const initialSupply = await token.totalSupply();
      await token.mint(addr1.address, 1000);
      
      expect(await token.totalSupply()).to.equal(initialSupply.add(1000));
      expect(await token.balanceOf(addr1.address)).to.equal(1000);
    });
    
    it("Should not allow non-owners to mint tokens", async function () {
      await expect(
        token.connect(addr1).mint(addr1.address, 1000)
      ).to.be.revertedWith("Ownable: caller is not the owner");
    });
  });
});
```

### Running Tests

```bash
npx hardhat test
```

## Deploying to Selendra

### Creating Deployment Scripts

Create a file `scripts/deploy.js`:

```javascript
const hre = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  
  // Deploy the contract
  const TokenFactory = await hre.ethers.getContractFactory("SelendraToken");
  const token = await TokenFactory.deploy(1000000); // 1 million tokens
  
  await token.deployed();
  
  console.log("Token deployed to:", token.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### Deploying to Local Node

First, make sure your local Selendra node is running:

```bash
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external
```

Then deploy your contract:

```bash
npx hardhat run scripts/deploy.js --network selendraLocal
```

### Deploying to Testnet

For testnet deployment, you'll need to set your private key as an environment variable:

```bash
export PRIVATE_KEY=your_private_key_here
npx hardhat run scripts/deploy.js --network selendraTest
```

Note: Never expose your private key in your code or commit it to version control.

### Verifying Your Contract (Optional)

Selendra supports Ethereum-compatible block explorers that offer contract verification. The process varies depending on which explorer is used for the network, but typically follows a similar pattern to Etherscan verification.

## Interacting with Deployed Contracts

### Using Hardhat Console

You can interact with your deployed contracts using the Hardhat console:

```bash
npx hardhat console --network selendraLocal
```

```javascript
// Get the contract factory
const TokenFactory = await ethers.getContractFactory("SelendraToken");

// Connect to your deployed contract
const token = await TokenFactory.attach("YOUR_DEPLOYED_CONTRACT_ADDRESS");

// Check total supply
const totalSupply = await token.totalSupply();
console.log("Total supply:", ethers.utils.formatEther(totalSupply));

// Get balance of an address
const balance = await token.balanceOf("ADDRESS_TO_CHECK");
console.log("Balance:", ethers.utils.formatEther(balance));

// Transfer tokens
const tx = await token.transfer("RECIPIENT_ADDRESS", ethers.utils.parseEther("100"));
await tx.wait();
console.log("Transfer complete");
```

### Using Web3.js or Ethers.js in a Frontend

Here's how to interact with your contract from a frontend app using ethers.js:

```javascript
// Connect to Selendra
const provider = new ethers.providers.JsonRpcProvider("https://mainnet.selendra.org");

// Connect to an existing contract
const contractABI = [...]; // Your contract ABI
const contractAddress = "YOUR_CONTRACT_ADDRESS";
const tokenContract = new ethers.Contract(contractAddress, contractABI, provider);

// Read-only operations
const totalSupply = await tokenContract.totalSupply();
console.log("Total supply:", ethers.utils.formatEther(totalSupply));

// For write operations, you need a signer
const privateKey = "YOUR_PRIVATE_KEY";
const wallet = new ethers.Wallet(privateKey, provider);
const tokenWithSigner = tokenContract.connect(wallet);

// Send a transaction
const tx = await tokenWithSigner.transfer("RECIPIENT_ADDRESS", ethers.utils.parseEther("10"));
const receipt = await tx.wait();
console.log("Transaction confirmed:", receipt.transactionHash);
```

### Using MetaMask

Your dApp users can interact with your contract using MetaMask:

1. Add Selendra network to MetaMask:
   - Network Name: Selendra Mainnet
   - RPC URL: https://mainnet.selendra.org
   - Chain ID: 1994
   - Currency Symbol: SEL
   - Block Explorer: (Your block explorer URL)

2. Connect your dApp to MetaMask using the Ethereum provider:

```javascript
async function connectWallet() {
  // Request account access
  const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
  return accounts[0];
}

async function sendTransaction() {
  const provider = new ethers.providers.Web3Provider(window.ethereum);
  const signer = provider.getSigner();
  
  const contractABI = [...]; // Your contract ABI
  const contractAddress = "YOUR_CONTRACT_ADDRESS";
  const contract = new ethers.Contract(contractAddress, contractABI, signer);
  
  // Call contract methods
  const tx = await contract.transfer("RECIPIENT_ADDRESS", ethers.utils.parseEther("10"));
  await tx.wait();
}
```

## Advanced Topics

### Gas Optimization Techniques

1. **Storage Optimization**:
   - Pack multiple variables into a single storage slot (for variables less than 32 bytes)
   - Use `bytes32` instead of `string` when possible
   - Use mappings instead of arrays when you need key-based lookup

2. **Computation Optimization**:
   - Move complex calculations off-chain when possible
   - Cache results of repeated operations
   - Use bit manipulation for flags instead of multiple booleans

3. **Batch Processing**:
   - Process multiple items in a single transaction
   - Implement patterns that allow bulk operations

### Contract Upgradeability

For upgradeable contracts, use the OpenZeppelin Upgrades plugin:

```bash
npm install @openzeppelin/hardhat-upgrades @openzeppelin/contracts-upgradeable
```

Create upgradeable contracts:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract UpgradeableToken is Initializable, ERC20Upgradeable, OwnableUpgradeable {
    function initialize(uint256 initialSupply) public initializer {
        __ERC20_init("Selendra Upgradeable Token", "SUT");
        __Ownable_init();
        _mint(msg.sender, initialSupply * 10 ** decimals());
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

Deploy using the upgrades plugin:

```javascript
const { ethers, upgrades } = require("hardhat");

async function main() {
  const TokenFactory = await ethers.getContractFactory("UpgradeableToken");
  const token = await upgrades.deployProxy(TokenFactory, [1000000], { initializer: 'initialize' });
  
  await token.deployed();
  console.log("Upgradeable token deployed to:", token.address);
}

main();
```

### Cross-Contract Communication

Selendra's EVM allows contracts to interact with each other just like on Ethereum:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract TokenVault {
    IERC20 public token;
    
    constructor(address _tokenAddress) {
        token = IERC20(_tokenAddress);
    }
    
    function deposit(uint256 _amount) external {
        // Transfer tokens from user to this contract
        require(token.transferFrom(msg.sender, address(this), _amount), "Transfer failed");
    }
    
    function withdraw(uint256 _amount) external {
        // Transfer tokens from this contract to user
        require(token.transfer(msg.sender, _amount), "Transfer failed");
    }
}
```

## Troubleshooting Common Issues

### Gas Estimation Issues

If you encounter "Gas estimation failed" errors:
- Explicitly set a higher gas limit in your transaction
- Check for reverts in your contract logic
- Ensure your contract isn't running into infinite loops

```javascript
// Example of setting explicit gas limit
const tx = await contract.complexFunction({
  gasLimit: 1000000
});
```

### Transaction Failures

If transactions fail:
- Check your account balance (both for tokens and native SEL)
- Verify contract permissions and requirements
- Check for any custom errors or requires in your contract

### Connection Issues

If you can't connect to the Selendra network:
- Verify the RPC endpoint URL
- Ensure your node is fully synced
- Check network configuration in Hardhat config

## Best Practices

1. **Always Test Thoroughly**:
   - Test on local node first
   - Then test on testnet
   - Only then deploy to mainnet

2. **Security First**:
   - Get contracts audited for high-value applications
   - Use established patterns and libraries
   - Keep private keys secure

3. **Gas Efficiency**:
   - Optimize contracts for low gas usage
   - Consider batch operations for multiple transactions

4. **Code Quality**:
   - Document your code with NatSpec comments
   - Use consistent naming conventions
   - Structure code logically

## Next Steps

- Learn how to [test your smart contracts effectively](./contract-testing.md)
- Explore [building your first dApp](./first-dapp.md) with your new contract
- Understand [transaction handling](./transaction-handling.md) on Selendra 