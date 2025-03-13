# Selendra Smart Contract Examples

## Basic Token Contract

### ERC-20 Token Implementation
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SelendraToken is ERC20, Ownable {
    constructor(uint256 initialSupply) ERC20("Selendra", "SEL") {
        _mint(msg.sender, initialSupply);
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
    
    function burn(uint256 amount) public {
        _burn(msg.sender, amount);
    }
}
```

### Deployment Script
```javascript
const { ethers } = require("hardhat");

async function main() {
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with account:", deployer.address);
    
    const Token = await ethers.getContractFactory("SelendraToken");
    const initialSupply = ethers.utils.parseEther("1000000");
    const token = await Token.deploy(initialSupply);
    
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

## DeFi Contract Examples

### Staking Contract
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract SelendraStaking is ReentrancyGuard {
    IERC20 public stakingToken;
    mapping(address => uint256) public stakedBalance;
    mapping(address => uint256) public stakingTimestamp;
    
    event Staked(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);
    
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
    
    function withdraw() external nonReentrant {
        uint256 amount = stakedBalance[msg.sender];
        require(amount > 0, "No staked tokens");
        require(
            block.timestamp >= stakingTimestamp[msg.sender] + 7 days,
            "Staking period not completed"
        );
        
        stakedBalance[msg.sender] = 0;
        stakingToken.transfer(msg.sender, amount);
        emit Withdrawn(msg.sender, amount);
    }
    
    function getStakedAmount(address user) external view returns (uint256) {
        return stakedBalance[user];
    }
}
```

### Liquidity Pool
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract SelendraPool is ReentrancyGuard {
    IERC20 public token0;
    IERC20 public token1;
    
    uint256 public reserve0;
    uint256 public reserve1;
    
    mapping(address => uint256) public lpTokens;
    uint256 public totalLpTokens;
    
    event LiquidityAdded(
        address indexed provider,
        uint256 amount0,
        uint256 amount1,
        uint256 lpTokens
    );
    
    constructor(address _token0, address _token1) {
        token0 = IERC20(_token0);
        token1 = IERC20(_token1);
    }
    
    function addLiquidity(uint256 amount0, uint256 amount1) 
        external 
        nonReentrant 
        returns (uint256 lpAmount) 
    {
        token0.transferFrom(msg.sender, address(this), amount0);
        token1.transferFrom(msg.sender, address(this), amount1);
        
        if (totalLpTokens == 0) {
            lpAmount = sqrt(amount0 * amount1);
        } else {
            lpAmount = min(
                (amount0 * totalLpTokens) / reserve0,
                (amount1 * totalLpTokens) / reserve1
            );
        }
        
        require(lpAmount > 0, "Insufficient liquidity minted");
        
        reserve0 += amount0;
        reserve1 += amount1;
        totalLpTokens += lpAmount;
        lpTokens[msg.sender] += lpAmount;
        
        emit LiquidityAdded(msg.sender, amount0, amount1, lpAmount);
    }
    
    function sqrt(uint256 y) internal pure returns (uint256 z) {
        if (y > 3) {
            z = y;
            uint256 x = y / 2 + 1;
            while (x < z) {
                z = x;
                x = (y / x + x) / 2;
            }
        } else if (y != 0) {
            z = 1;
        }
    }
    
    function min(uint256 x, uint256 y) internal pure returns (uint256) {
        return x <= y ? x : y;
    }
}
```

## NFT Contract Examples

### Basic NFT Collection
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract SelendraNFT is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    
    mapping(uint256 => string) private _tokenURIs;
    uint256 public constant MINT_PRICE = 0.1 ether;
    uint256 public constant MAX_SUPPLY = 10000;
    
    constructor() ERC721("Selendra NFT", "SNFT") {}
    
    function mint(string memory tokenURI) public payable returns (uint256) {
        require(msg.value >= MINT_PRICE, "Insufficient payment");
        require(_tokenIds.current() < MAX_SUPPLY, "Max supply reached");
        
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();
        
        _mint(msg.sender, newTokenId);
        _setTokenURI(newTokenId, tokenURI);
        
        return newTokenId;
    }
    
    function _setTokenURI(uint256 tokenId, string memory _tokenURI) 
        internal 
        virtual 
    {
        require(
            _exists(tokenId),
            "ERC721Metadata: URI set of nonexistent token"
        );
        _tokenURIs[tokenId] = _tokenURI;
    }
    
    function tokenURI(uint256 tokenId) 
        public 
        view 
        virtual 
        override 
        returns (string memory) 
    {
        require(
            _exists(tokenId),
            "ERC721Metadata: URI query for nonexistent token"
        );
        return _tokenURIs[tokenId];
    }
    
    function withdraw() public onlyOwner {
        uint256 balance = address(this).balance;
        payable(owner()).transfer(balance);
    }
}
```

### NFT Marketplace
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract SelendraNFTMarket is ReentrancyGuard {
    struct Listing {
        address seller;
        uint256 price;
        bool active;
    }
    
    mapping(address => mapping(uint256 => Listing)) public listings;
    uint256 public marketplaceFee = 25; // 2.5%
    
    event NFTListed(
        address indexed nftContract,
        uint256 indexed tokenId,
        address seller,
        uint256 price
    );
    event NFTSold(
        address indexed nftContract,
        uint256 indexed tokenId,
        address seller,
        address buyer,
        uint256 price
    );
    
    function listNFT(
        address nftContract,
        uint256 tokenId,
        uint256 price
    ) external {
        IERC721 nft = IERC721(nftContract);
        require(
            nft.ownerOf(tokenId) == msg.sender,
            "Not the NFT owner"
        );
        require(
            nft.getApproved(tokenId) == address(this),
            "NFT not approved for marketplace"
        );
        
        listings[nftContract][tokenId] = Listing(msg.sender, price, true);
        emit NFTListed(nftContract, tokenId, msg.sender, price);
    }
    
    function buyNFT(address nftContract, uint256 tokenId) 
        external 
        payable 
        nonReentrant 
    {
        Listing memory listing = listings[nftContract][tokenId];
        require(listing.active, "NFT not listed");
        require(msg.value >= listing.price, "Insufficient payment");
        
        listings[nftContract][tokenId].active = false;
        
        uint256 fee = (msg.value * marketplaceFee) / 1000;
        uint256 sellerAmount = msg.value - fee;
        
        payable(listing.seller).transfer(sellerAmount);
        
        IERC721(nftContract).safeTransferFrom(
            listing.seller,
            msg.sender,
            tokenId
        );
        
        emit NFTSold(
            nftContract,
            tokenId,
            listing.seller,
            msg.sender,
            msg.value
        );
    }
}
```

## Testing Examples

### Token Contract Tests
```javascript
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SelendraToken", function () {
    let Token;
    let token;
    let owner;
    let addr1;
    let addr2;
    
    beforeEach(async function () {
        Token = await ethers.getContractFactory("SelendraToken");
        [owner, addr1, addr2] = await ethers.getSigners();
        token = await Token.deploy(ethers.utils.parseEther("1000000"));
        await token.deployed();
    });
    
    describe("Deployment", function () {
        it("Should assign total supply to owner", async function () {
            const ownerBalance = await token.balanceOf(owner.address);
            expect(await token.totalSupply()).to.equal(ownerBalance);
        });
    });
    
    describe("Transactions", function () {
        it("Should transfer tokens between accounts", async function () {
            await token.transfer(addr1.address, 50);
            const addr1Balance = await token.balanceOf(addr1.address);
            expect(addr1Balance).to.equal(50);
            
            await token.connect(addr1).transfer(addr2.address, 50);
            const addr2Balance = await token.balanceOf(addr2.address);
            expect(addr2Balance).to.equal(50);
        });
        
        it("Should fail if sender doesn't have enough tokens", async function () {
            const initialOwnerBalance = await token.balanceOf(owner.address);
            await expect(
                token.connect(addr1).transfer(owner.address, 1)
            ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
            expect(
                await token.balanceOf(owner.address)
            ).to.equal(initialOwnerBalance);
        });
    });
});
```

### Staking Contract Tests
```javascript
const { expect } = require("chai");
const { ethers } = require("hardhat");
const { time } = require("@nomicfoundation/hardhat-network-helpers");

describe("SelendraStaking", function () {
    let Token;
    let token;
    let Staking;
    let staking;
    let owner;
    let addr1;
    
    beforeEach(async function () {
        [owner, addr1] = await ethers.getSigners();
        
        Token = await ethers.getContractFactory("SelendraToken");
        token = await Token.deploy(ethers.utils.parseEther("1000000"));
        
        Staking = await ethers.getContractFactory("SelendraStaking");
        staking = await Staking.deploy(token.address);
        
        await token.approve(
            staking.address,
            ethers.utils.parseEther("1000000")
        );
    });
    
    describe("Staking", function () {
        it("Should stake tokens", async function () {
            const stakeAmount = ethers.utils.parseEther("100");
            await staking.stake(stakeAmount);
            
            expect(
                await staking.getStakedAmount(owner.address)
            ).to.equal(stakeAmount);
        });
        
        it("Should not withdraw before time", async function () {
            const stakeAmount = ethers.utils.parseEther("100");
            await staking.stake(stakeAmount);
            
            await expect(
                staking.withdraw()
            ).to.be.revertedWith("Staking period not completed");
        });
        
        it("Should withdraw after time", async function () {
            const stakeAmount = ethers.utils.parseEther("100");
            await staking.stake(stakeAmount);
            
            await time.increase(7 * 24 * 60 * 60); // 7 days
            
            await staking.withdraw();
            expect(
                await staking.getStakedAmount(owner.address)
            ).to.equal(0);
        });
    });
});
```
