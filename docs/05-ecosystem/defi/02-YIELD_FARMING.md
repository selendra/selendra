# Yield Farming

## Overview

Selendra's Yield Farming protocol enables:
- Liquidity mining
- Staking rewards
- Farming incentives
- Reward distribution
- Governance participation

## Core Components

### 1. Farming Pool
```solidity
contract FarmingPool {
    struct Pool {
        address stakingToken;
        address rewardToken;
        uint256 rewardRate;
        uint256 lastUpdateTime;
        uint256 rewardPerTokenStored;
        uint256 totalStaked;
    }
    
    mapping(uint256 => Pool) public pools;
    mapping(uint256 => mapping(address => uint256)) public userStakes;
    mapping(uint256 => mapping(address => uint256)) public rewards;
    
    function stake(
        uint256 poolId,
        uint256 amount
    ) external {
        updatePool(poolId);
        Pool storage pool = pools[poolId];
        
        userStakes[poolId][msg.sender] += amount;
        pool.totalStaked += amount;
        
        IERC20(pool.stakingToken).transferFrom(
            msg.sender,
            address(this),
            amount
        );
        
        emit Staked(msg.sender, poolId, amount);
    }
    
    function withdraw(
        uint256 poolId,
        uint256 amount
    ) external {
        updatePool(poolId);
        Pool storage pool = pools[poolId];
        
        userStakes[poolId][msg.sender] -= amount;
        pool.totalStaked -= amount;
        
        IERC20(pool.stakingToken).transfer(
            msg.sender,
            amount
        );
        
        emit Withdrawn(msg.sender, poolId, amount);
    }
}
```

### 2. Reward Distribution
```solidity
contract RewardDistributor {
    function updatePool(uint256 poolId) public {
        Pool storage pool = pools[poolId];
        if (block.timestamp <= pool.lastUpdateTime) return;
        
        if (pool.totalStaked == 0) {
            pool.lastUpdateTime = block.timestamp;
            return;
        }
        
        uint256 timeElapsed = block.timestamp - pool.lastUpdateTime;
        uint256 reward = timeElapsed * pool.rewardRate;
        pool.rewardPerTokenStored += 
            (reward * 1e18) / pool.totalStaked;
        pool.lastUpdateTime = block.timestamp;
    }
    
    function earned(
        uint256 poolId,
        address account
    ) public view returns (uint256) {
        Pool memory pool = pools[poolId];
        return (userStakes[poolId][account] * 
            (pool.rewardPerTokenStored - userRewardPerTokenPaid[poolId][account])) / 1e18
            + rewards[poolId][account];
    }
    
    function getReward(uint256 poolId) external {
        updatePool(poolId);
        Pool storage pool = pools[poolId];
        
        uint256 reward = earned(poolId, msg.sender);
        if (reward > 0) {
            rewards[poolId][msg.sender] = 0;
            IERC20(pool.rewardToken).transfer(msg.sender, reward);
            emit RewardPaid(msg.sender, poolId, reward);
        }
    }
}
```

### 3. Incentive Manager
```solidity
contract IncentiveManager {
    struct Incentive {
        address rewardToken;
        uint256 rewardAmount;
        uint256 startTime;
        uint256 endTime;
        bool active;
    }
    
    mapping(uint256 => Incentive) public poolIncentives;
    
    function addIncentive(
        uint256 poolId,
        address rewardToken,
        uint256 rewardAmount,
        uint256 duration
    ) external {
        require(
            IERC20(rewardToken).transferFrom(
                msg.sender,
                address(this),
                rewardAmount
            ),
            "Transfer failed"
        );
        
        poolIncentives[poolId] = Incentive({
            rewardToken: rewardToken,
            rewardAmount: rewardAmount,
            startTime: block.timestamp,
            endTime: block.timestamp + duration,
            active: true
        });
        
        emit IncentiveAdded(poolId, rewardToken, rewardAmount, duration);
    }
}
```

## Farming Strategies

### 1. Single Asset Staking
```solidity
contract SingleAssetStaking {
    struct StakingPool {
        address stakingToken;
        uint256 totalStaked;
        uint256 rewardRate;
        uint256 lastUpdateTime;
        uint256 rewardPerTokenStored;
    }
    
    mapping(uint256 => StakingPool) public stakingPools;
    
    function createStakingPool(
        address stakingToken,
        uint256 rewardRate
    ) external returns (uint256) {
        uint256 poolId = nextPoolId++;
        stakingPools[poolId] = StakingPool({
            stakingToken: stakingToken,
            totalStaked: 0,
            rewardRate: rewardRate,
            lastUpdateTime: block.timestamp,
            rewardPerTokenStored: 0
        });
        return poolId;
    }
}
```

### 2. LP Token Farming
```solidity
contract LPTokenFarming {
    struct LPPool {
        address lpToken;
        address token0;
        address token1;
        uint256 totalValueLocked;
        uint256 rewardMultiplier;
    }
    
    mapping(uint256 => LPPool) public lpPools;
    
    function addLPTokenFarm(
        address lpToken,
        address token0,
        address token1,
        uint256 rewardMultiplier
    ) external returns (uint256) {
        uint256 poolId = nextPoolId++;
        lpPools[poolId] = LPPool({
            lpToken: lpToken,
            token0: token0,
            token1: token1,
            totalValueLocked: 0,
            rewardMultiplier: rewardMultiplier
        });
        return poolId;
    }
}
```

## APY Calculation

### 1. Reward Calculator
```solidity
contract APYCalculator {
    function calculateAPY(
        uint256 rewardRate,
        uint256 totalStaked,
        uint256 rewardTokenPrice,
        uint256 stakingTokenPrice
    ) public pure returns (uint256) {
        // Daily reward in USD
        uint256 dailyReward = rewardRate * 86400 * rewardTokenPrice;
        
        // Total staked in USD
        uint256 totalStakedUSD = totalStaked * stakingTokenPrice;
        
        // APY = (Daily reward * 365 * 100) / Total staked
        return (dailyReward * 365 * 100) / totalStakedUSD;
    }
}
```

### 2. Price Feed
```solidity
contract PriceFeedAggregator {
    mapping(address => AggregatorV3Interface) public priceFeeds;
    
    function getLatestPrice(
        address token
    ) public view returns (uint256) {
        AggregatorV3Interface feed = priceFeeds[token];
        require(address(feed) != address(0), "No price feed");
        
        (
            uint80 roundId,
            int256 price,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        ) = feed.latestRoundData();
        
        require(price > 0, "Invalid price");
        return uint256(price);
    }
}
```

## Analytics

### 1. Pool Analytics
```typescript
interface PoolAnalytics {
    // TVL Tracking
    getTotalValueLocked(poolId: string): Promise<number>;
    getPoolAPY(poolId: string): Promise<number>;
    
    // User Analytics
    getUserStake(poolId: string, user: string): Promise<StakeInfo>;
    getUserRewards(poolId: string, user: string): Promise<RewardInfo>;
}
```

### 2. Performance Metrics
```typescript
interface PerformanceMetrics {
    // Historical Data
    getHistoricalAPY(
        poolId: string,
        timeframe: Timeframe
    ): Promise<APYHistory>;
    
    // Volume Analytics
    getStakingVolume(
        poolId: string,
        timeframe: Timeframe
    ): Promise<VolumeData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface FarmingSDK {
    // Pool Management
    getPoolInfo(poolId: string): Promise<PoolInfo>;
    getUserInfo(poolId: string, user: string): Promise<UserInfo>;
    
    // Staking Operations
    stake(poolId: string, amount: string): Promise<string>;
    withdraw(poolId: string, amount: string): Promise<string>;
    claim(poolId: string): Promise<string>;
    
    // Analytics
    getAPY(poolId: string): Promise<number>;
    getTVL(poolId: string): Promise<number>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onStake(callback: (event: StakeEvent) => void): void;
    onWithdraw(callback: (event: WithdrawEvent) => void): void;
    onReward(callback: (event: RewardEvent) => void): void;
    
    // Analytics
    getPoolStats(poolId: string): Promise<PoolStats>;
    getUserStats(user: string): Promise<UserStats>;
}
```
