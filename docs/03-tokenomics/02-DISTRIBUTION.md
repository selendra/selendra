# Token Distribution

## Overview

The Selendra token (SEL) has a total supply of 1,000,000,000 SEL distributed across:
- Community Development (30%)
- Ecosystem Growth (25%)
- Team and Advisors (15%)
- Private Sale (10%)
- Public Sale (10%)
- Reserve Fund (10%)

## Distribution Schedule

### 1. Initial Distribution
```typescript
interface InitialDistribution {
    communityDevelopment: {
        amount: 300_000_000,
        vestingPeriod: '48 months',
        cliffPeriod: '12 months',
        vestingSchedule: 'Linear'
    },
    ecosystemGrowth: {
        amount: 250_000_000,
        vestingPeriod: '36 months',
        cliffPeriod: '6 months',
        vestingSchedule: 'Linear'
    },
    teamAndAdvisors: {
        amount: 150_000_000,
        vestingPeriod: '48 months',
        cliffPeriod: '12 months',
        vestingSchedule: 'Linear'
    },
    privateSale: {
        amount: 100_000_000,
        vestingPeriod: '24 months',
        cliffPeriod: '3 months',
        vestingSchedule: 'Linear'
    },
    publicSale: {
        amount: 100_000_000,
        vestingPeriod: '12 months',
        cliffPeriod: '0 months',
        vestingSchedule: 'Linear'
    },
    reserveFund: {
        amount: 100_000_000,
        vestingPeriod: '60 months',
        cliffPeriod: '24 months',
        vestingSchedule: 'Linear'
    }
}
```

### 2. Vesting Contract
```solidity
contract TokenVesting {
    struct VestingSchedule {
        uint256 totalAmount;
        uint256 startTime;
        uint256 cliffDuration;
        uint256 vestingDuration;
        uint256 releasedAmount;
        bool revocable;
        bool revoked;
    }
    
    mapping(address => VestingSchedule) public vestingSchedules;
    
    function createVestingSchedule(
        address beneficiary,
        uint256 startTime,
        uint256 cliffDuration,
        uint256 vestingDuration,
        uint256 totalAmount,
        bool revocable
    ) external {
        require(
            vestingSchedules[beneficiary].totalAmount == 0,
            "Schedule exists"
        );
        
        vestingSchedules[beneficiary] = VestingSchedule({
            totalAmount: totalAmount,
            startTime: startTime,
            cliffDuration: cliffDuration,
            vestingDuration: vestingDuration,
            releasedAmount: 0,
            revocable: revocable,
            revoked: false
        });
    }
    
    function release(address beneficiary) external {
        VestingSchedule storage schedule = vestingSchedules[beneficiary];
        require(!schedule.revoked, "Schedule revoked");
        
        uint256 releasable = getReleasableAmount(beneficiary);
        require(releasable > 0, "No tokens to release");
        
        schedule.releasedAmount += releasable;
        token.transfer(beneficiary, releasable);
    }
    
    function getReleasableAmount(
        address beneficiary
    ) public view returns (uint256) {
        VestingSchedule memory schedule = vestingSchedules[beneficiary];
        
        if (block.timestamp < schedule.startTime + schedule.cliffDuration) {
            return 0;
        }
        
        if (block.timestamp >= schedule.startTime + schedule.vestingDuration) {
            return schedule.totalAmount - schedule.releasedAmount;
        }
        
        uint256 timeFromStart = block.timestamp - schedule.startTime;
        uint256 vestedAmount = (schedule.totalAmount * timeFromStart) / 
            schedule.vestingDuration;
            
        return vestedAmount - schedule.releasedAmount;
    }
}
```

## Token Utility

### 1. Network Utility
```solidity
contract TokenUtility {
    struct StakingConfig {
        uint256 minimumStake;
        uint256 maximumStake;
        uint256 unbondingPeriod;
        uint256 rewardRate;
    }
    
    struct GovernanceConfig {
        uint256 proposalDeposit;
        uint256 votingPeriod;
        uint256 enactmentPeriod;
        uint256 minimumParticipation;
    }
    
    StakingConfig public stakingConfig;
    GovernanceConfig public governanceConfig;
    
    function updateStakingConfig(
        StakingConfig memory newConfig
    ) external onlyGovernance {
        stakingConfig = newConfig;
        emit StakingConfigUpdated(newConfig);
    }
    
    function updateGovernanceConfig(
        GovernanceConfig memory newConfig
    ) external onlyGovernance {
        governanceConfig = newConfig;
        emit GovernanceConfigUpdated(newConfig);
    }
}
```

### 2. DeFi Utility
```solidity
contract DeFiUtility {
    struct LiquidityConfig {
        uint256 minimumLiquidity;
        uint256 tradingFee;
        uint256 liquidityMiningRate;
    }
    
    struct LendingConfig {
        uint256 collateralRatio;
        uint256 liquidationThreshold;
        uint256 borrowInterestRate;
        uint256 supplyInterestRate;
    }
    
    mapping(address => LiquidityConfig) public liquidityConfigs;
    mapping(address => LendingConfig) public lendingConfigs;
    
    function provideLiquidity(
        address pool,
        uint256 amount
    ) external {
        LiquidityConfig memory config = liquidityConfigs[pool];
        require(amount >= config.minimumLiquidity, "Insufficient liquidity");
        
        // Transfer tokens
        token.transferFrom(msg.sender, address(this), amount);
        
        // Mint LP tokens
        lpToken.mint(msg.sender, amount);
        
        emit LiquidityProvided(msg.sender, pool, amount);
    }
    
    function borrow(
        address pool,
        uint256 amount
    ) external {
        LendingConfig memory config = lendingConfigs[pool];
        uint256 collateral = getCollateral(msg.sender);
        
        require(
            amount <= (collateral * config.collateralRatio) / 1e18,
            "Insufficient collateral"
        );
        
        // Transfer tokens
        token.transfer(msg.sender, amount);
        
        emit Borrowed(msg.sender, pool, amount);
    }
}
```

## Token Economics

### 1. Supply Management
```solidity
contract SupplyManagement {
    struct SupplyConfig {
        uint256 inflationRate;
        uint256 burnRate;
        uint256 maxSupply;
    }
    
    SupplyConfig public supplyConfig;
    uint256 public totalSupply;
    uint256 public circulatingSupply;
    
    function mint(uint256 amount) external onlyMinter {
        require(
            totalSupply + amount <= supplyConfig.maxSupply,
            "Exceeds max supply"
        );
        
        totalSupply += amount;
        circulatingSupply += amount;
        
        token.mint(msg.sender, amount);
        emit TokensMinted(msg.sender, amount);
    }
    
    function burn(uint256 amount) external {
        token.burnFrom(msg.sender, amount);
        
        totalSupply -= amount;
        circulatingSupply -= amount;
        
        emit TokensBurned(msg.sender, amount);
    }
}
```

### 2. Fee Management
```solidity
contract FeeManagement {
    struct FeeConfig {
        uint256 transactionFee;
        uint256 stakingFee;
        uint256 governanceFee;
        uint256 protocolFee;
    }
    
    FeeConfig public feeConfig;
    address public feeCollector;
    
    function collectFees(
        address user,
        uint256 amount,
        FeeType feeType
    ) external returns (uint256) {
        uint256 fee = calculateFee(amount, feeType);
        
        if (fee > 0) {
            token.transferFrom(user, feeCollector, fee);
            emit FeeCollected(user, fee, feeType);
        }
        
        return fee;
    }
    
    function calculateFee(
        uint256 amount,
        FeeType feeType
    ) public view returns (uint256) {
        uint256 feeRate = getFeeRate(feeType);
        return (amount * feeRate) / 1e18;
    }
}
```

## Token Analytics

### 1. Market Analytics
```typescript
interface MarketAnalytics {
    // Price Data
    getTokenPrice(): Promise<TokenPrice>;
    getPriceHistory(timeframe: Timeframe): Promise<PriceHistory>;
    
    // Volume Data
    getTradingVolume(timeframe: Timeframe): Promise<VolumeData>;
    getLiquidityData(): Promise<LiquidityData>;
    
    // Market Metrics
    getMarketCap(): Promise<MarketCap>;
    getTokenMetrics(): Promise<TokenMetrics>;
}
```

### 2. Network Analytics
```typescript
interface NetworkAnalytics {
    // Staking Data
    getStakingMetrics(): Promise<StakingMetrics>;
    getValidatorStats(): Promise<ValidatorStats>;
    
    // Governance Data
    getProposalStats(): Promise<ProposalStats>;
    getVotingHistory(): Promise<VotingHistory>;
    
    // Usage Metrics
    getTransactionStats(): Promise<TransactionStats>;
    getUserMetrics(): Promise<UserMetrics>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface TokenSDK {
    // Basic Operations
    getBalance(address: string): Promise<Balance>;
    transfer(to: string, amount: string): Promise<string>;
    
    // Staking Operations
    stake(amount: string): Promise<string>;
    unstake(amount: string): Promise<string>;
    
    // Governance Operations
    createProposal(params: ProposalParams): Promise<string>;
    vote(proposalId: string, vote: Vote): Promise<string>;
}
```

### 2. Analytics Integration
```typescript
interface AnalyticsSDK {
    // Market Data
    subscribeToPrice(callback: PriceCallback): Subscription;
    subscribeToVolume(callback: VolumeCallback): Subscription;
    
    // Network Data
    subscribeToStaking(callback: StakingCallback): Subscription;
    subscribeToGovernance(callback: GovernanceCallback): Subscription;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```
