# Deploying Smart Contracts with Hardhat

## Setup

### Installation
```bash
mkdir selendra-hardhat
cd selendra-hardhat
npm init -y
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npx hardhat init
```

### Configuration
```javascript
// hardhat.config.js
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.19",
  networks: {
    selendra: {
      url: "https://rpc.selendra.org",
      chainId: 1961,
      accounts: [process.env.PRIVATE_KEY],
      gasPrice: 20000000000
    },
    selendraTestnet: {
      url: "https://testnet-rpc.selendra.org",
      chainId: 1953,
      accounts: [process.env.PRIVATE_KEY]
    }
  }
};
```

## Sample Contracts

### NFT Contract
```solidity
// contracts/SelendraNFT.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SelendraNFT is ERC721, Ownable {
    uint256 private _tokenIds;
    mapping(uint256 => string) private _tokenURIs;

    constructor() ERC721("SelendraNFT", "SNFT") {}

    function mint(address to, string memory tokenURI) public onlyOwner returns (uint256) {
        _tokenIds++;
        uint256 newTokenId = _tokenIds;
        _mint(to, newTokenId);
        _setTokenURI(newTokenId, tokenURI);
        return newTokenId;
    }

    function _setTokenURI(uint256 tokenId, string memory _tokenURI) internal {
        require(_exists(tokenId), "Token does not exist");
        _tokenURIs[tokenId] = _tokenURI;
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(_exists(tokenId), "Token does not exist");
        return _tokenURIs[tokenId];
    }
}
```

### DeFi Contract
```solidity
// contracts/SelendraStaking.sol
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract SelendraStaking is ReentrancyGuard {
    IERC20 public stakingToken;
    mapping(address => uint256) public stakedBalance;
    mapping(address => uint256) public stakingTimestamp;
    
    uint256 public constant REWARD_RATE = 100; // 1% per day
    uint256 public constant MINIMUM_STAKING_PERIOD = 1 days;
    
    event Staked(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount, uint256 reward);
    
    constructor(address _stakingToken) {
        stakingToken = IERC20(_stakingToken);
    }
    
    function stake(uint256 amount) external nonReentrant {
        require(amount > 0, "Cannot stake 0");
        stakingToken.transferFrom(msg.sender, address(this), amount);
        stakedBalance[msg.sender] += amount;
        stakingTimestamp[msg.sender] = block.timestamp;
        emit Staked(msg.sender, amount);
    }
    
    function calculateReward(address user) public view returns (uint256) {
        uint256 timeStaked = block.timestamp - stakingTimestamp[user];
        return (stakedBalance[user] * timeStaked * REWARD_RATE) / (100 * 1 days);
    }
    
    function withdraw() external nonReentrant {
        require(block.timestamp >= stakingTimestamp[msg.sender] + MINIMUM_STAKING_PERIOD, 
                "Minimum staking period not met");
        uint256 reward = calculateReward(msg.sender);
        uint256 amount = stakedBalance[msg.sender];
        require(amount > 0, "No staked amount");
        
        stakedBalance[msg.sender] = 0;
        stakingToken.transfer(msg.sender, amount + reward);
        emit Withdrawn(msg.sender, amount, reward);
    }
}
```

## Deployment Scripts

### Deploy NFT
```javascript
// scripts/deploy-nft.js
async function main() {
  const SelendraNFT = await ethers.getContractFactory("SelendraNFT");
  console.log("Deploying SelendraNFT...");
  const nft = await SelendraNFT.deploy();
  await nft.deployed();
  console.log("SelendraNFT deployed to:", nft.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### Deploy Staking
```javascript
// scripts/deploy-staking.js
async function main() {
  // Deploy mock token first for testing
  const MockToken = await ethers.getContractFactory("MockERC20");
  const mockToken = await MockToken.deploy("Mock Token", "MTK");
  await mockToken.deployed();
  console.log("Mock Token deployed to:", mockToken.address);

  const SelendraStaking = await ethers.getContractFactory("SelendraStaking");
  console.log("Deploying SelendraStaking...");
  const staking = await SelendraStaking.deploy(mockToken.address);
  await staking.deployed();
  console.log("SelendraStaking deployed to:", staking.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

## Testing

### NFT Tests
```javascript
// test/SelendraNFT.test.js
const { expect } = require("chai");

describe("SelendraNFT", function () {
  let nft;
  let owner;
  let addr1;

  beforeEach(async function () {
    [owner, addr1] = await ethers.getSigners();
    const SelendraNFT = await ethers.getContractFactory("SelendraNFT");
    nft = await SelendraNFT.deploy();
    await nft.deployed();
  });

  it("Should mint a new token", async function () {
    await nft.mint(addr1.address, "ipfs://test");
    expect(await nft.ownerOf(1)).to.equal(addr1.address);
  });
});
```

## Deployment Commands
```bash
# Deploy to testnet
npx hardhat run scripts/deploy-nft.js --network selendraTestnet

# Deploy to mainnet
npx hardhat run scripts/deploy-staking.js --network selendra

# Verify contract
npx hardhat verify --network selendra DEPLOYED_CONTRACT_ADDRESS
```
