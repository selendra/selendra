# Deploying Smart Contracts with Foundry

## Setup

### Installation
```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
forge init selendra-foundry
cd selendra-foundry
```

### Configuration
```toml
# foundry.toml
[profile.default]
src = "src"
out = "out"
libs = ["lib"]
solc = "0.8.19"
optimizer = true
optimizer_runs = 200

[profile.selendra]
chainId = 1961
gas_price = 20000000000

[profile.selendra_testnet]
chainId = 1953

[rpc_endpoints]
selendra = "https://rpc.selendra.org"
selendra_testnet = "https://testnet-rpc.selendra.org"

[etherscan]
selendra = { key = "${SELENDRA_API_KEY}" }
selendra_testnet = { key = "${SELENDRA_API_KEY}" }
```

## Sample Contracts

### Liquidity Pool Contract
```solidity
// src/SelendraPool.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SelendraPool is ReentrancyGuard, Ownable {
    IERC20 public immutable token0;
    IERC20 public immutable token1;
    
    uint256 public reserve0;
    uint256 public reserve1;
    uint256 public constant MINIMUM_LIQUIDITY = 1000;
    
    mapping(address => uint256) public liquidity;
    uint256 public totalLiquidity;
    
    event LiquidityAdded(address indexed provider, uint256 amount0, uint256 amount1);
    event LiquidityRemoved(address indexed provider, uint256 amount0, uint256 amount1);
    event Swap(address indexed user, uint256 amountIn, uint256 amountOut, bool isToken0);
    
    constructor(address _token0, address _token1) {
        token0 = IERC20(_token0);
        token1 = IERC20(_token1);
    }
    
    function addLiquidity(uint256 amount0, uint256 amount1) external nonReentrant {
        require(amount0 > 0 && amount1 > 0, "Invalid amounts");
        
        token0.transferFrom(msg.sender, address(this), amount0);
        token1.transferFrom(msg.sender, address(this), amount1);
        
        uint256 liquidity_;
        if (totalLiquidity == 0) {
            liquidity_ = Math.sqrt(amount0 * amount1) - MINIMUM_LIQUIDITY;
        } else {
            liquidity_ = Math.min(
                (amount0 * totalLiquidity) / reserve0,
                (amount1 * totalLiquidity) / reserve1
            );
        }
        
        require(liquidity_ > 0, "Insufficient liquidity minted");
        
        liquidity[msg.sender] += liquidity_;
        totalLiquidity += liquidity_;
        reserve0 += amount0;
        reserve1 += amount1;
        
        emit LiquidityAdded(msg.sender, amount0, amount1);
    }
    
    function removeLiquidity(uint256 liquidityAmount) external nonReentrant {
        require(liquidityAmount > 0, "Invalid liquidity amount");
        require(liquidity[msg.sender] >= liquidityAmount, "Insufficient liquidity");
        
        uint256 amount0 = (liquidityAmount * reserve0) / totalLiquidity;
        uint256 amount1 = (liquidityAmount * reserve1) / totalLiquidity;
        
        require(amount0 > 0 && amount1 > 0, "Insufficient liquidity burned");
        
        liquidity[msg.sender] -= liquidityAmount;
        totalLiquidity -= liquidityAmount;
        reserve0 -= amount0;
        reserve1 -= amount1;
        
        token0.transfer(msg.sender, amount0);
        token1.transfer(msg.sender, amount1);
        
        emit LiquidityRemoved(msg.sender, amount0, amount1);
    }
    
    function swap(uint256 amountIn, bool isToken0) external nonReentrant {
        require(amountIn > 0, "Invalid input amount");
        
        IERC20 tokenIn = isToken0 ? token0 : token1;
        IERC20 tokenOut = isToken0 ? token1 : token0;
        uint256 reserveIn = isToken0 ? reserve0 : reserve1;
        uint256 reserveOut = isToken0 ? reserve1 : reserve0;
        
        tokenIn.transferFrom(msg.sender, address(this), amountIn);
        
        uint256 amountOut = getAmountOut(amountIn, reserveIn, reserveOut);
        require(amountOut > 0, "Insufficient output amount");
        
        if (isToken0) {
            reserve0 += amountIn;
            reserve1 -= amountOut;
        } else {
            reserve1 += amountIn;
            reserve0 -= amountOut;
        }
        
        tokenOut.transfer(msg.sender, amountOut);
        
        emit Swap(msg.sender, amountIn, amountOut, isToken0);
    }
    
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) 
        public pure returns (uint256) 
    {
        require(amountIn > 0, "Invalid input amount");
        require(reserveIn > 0 && reserveOut > 0, "Invalid reserves");
        
        uint256 amountInWithFee = amountIn * 997; // 0.3% fee
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = (reserveIn * 1000) + amountInWithFee;
        
        return numerator / denominator;
    }
}
```

## Deployment Scripts

### Deploy Script
```solidity
// script/DeployPool.s.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/SelendraPool.sol";
import "../src/MockToken.sol"; // For testing

contract DeployPool is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy mock tokens for testing
        MockToken token0 = new MockToken("Token0", "TK0");
        MockToken token1 = new MockToken("Token1", "TK1");

        // Deploy pool
        SelendraPool pool = new SelendraPool(
            address(token0),
            address(token1)
        );

        vm.stopBroadcast();

        console.log("Token0 deployed to:", address(token0));
        console.log("Token1 deployed to:", address(token1));
        console.log("Pool deployed to:", address(pool));
    }
}
```

## Testing

### Pool Tests
```solidity
// test/SelendraPool.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/SelendraPool.sol";
import "../src/MockToken.sol";

contract SelendraPoolTest is Test {
    SelendraPool pool;
    MockToken token0;
    MockToken token1;
    address user = address(1);
    
    function setUp() public {
        token0 = new MockToken("Token0", "TK0");
        token1 = new MockToken("Token1", "TK1");
        pool = new SelendraPool(address(token0), address(token1));
        
        token0.mint(user, 1000e18);
        token1.mint(user, 1000e18);
        
        vm.startPrank(user);
        token0.approve(address(pool), type(uint256).max);
        token1.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }
    
    function testAddLiquidity() public {
        vm.startPrank(user);
        pool.addLiquidity(100e18, 100e18);
        vm.stopPrank();
        
        assertEq(pool.reserve0(), 100e18);
        assertEq(pool.reserve1(), 100e18);
    }
    
    function testSwap() public {
        vm.startPrank(user);
        pool.addLiquidity(100e18, 100e18);
        
        uint256 amountIn = 10e18;
        uint256 expectedOut = pool.getAmountOut(amountIn, 100e18, 100e18);
        pool.swap(amountIn, true);
        
        assertEq(pool.reserve0(), 110e18);
        assertEq(pool.reserve1(), 100e18 - expectedOut);
        vm.stopPrank();
    }
}
```

## Deployment Commands
```bash
# Deploy to testnet
forge script script/DeployPool.s.sol:DeployPool --rpc-url selendra_testnet --broadcast

# Deploy to mainnet
forge script script/DeployPool.s.sol:DeployPool --rpc-url selendra --broadcast

# Run tests
forge test -vv

# Generate gas report
forge test --gas-report
```
