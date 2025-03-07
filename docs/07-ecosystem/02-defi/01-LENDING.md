# Lending Protocol

## Overview

The Selendra Lending Protocol enables:
- Decentralized lending and borrowing
- Multiple asset support
- Dynamic interest rates
- Collateralization
- Liquidation mechanisms

## Core Components

### 1. Lending Pool
```solidity
contract LendingPool {
    struct Reserve {
        address asset;
        uint256 totalDeposits;
        uint256 totalBorrows;
        uint256 utilizationRate;
        uint256 borrowRate;
        uint256 supplyRate;
    }
    
    mapping(address => Reserve) public reserves;
    
    function deposit(
        address asset,
        uint256 amount,
        address onBehalfOf
    ) external {
        // Deposit logic
        Reserve storage reserve = reserves[asset];
        reserve.totalDeposits += amount;
        updateRates(asset);
        
        // Mint sTokens (supply tokens)
        sToken[asset].mint(onBehalfOf, amount);
    }
    
    function borrow(
        address asset,
        uint256 amount,
        uint256 interestRateMode,
        address onBehalfOf
    ) external {
        // Borrow logic
        validateCollateral(onBehalfOf);
        Reserve storage reserve = reserves[asset];
        reserve.totalBorrows += amount;
        updateRates(asset);
        
        // Mint debtTokens
        debtToken[asset].mint(onBehalfOf, amount);
    }
}
```

### 2. Interest Rate Model
```solidity
contract InterestRateModel {
    uint256 public constant OPTIMAL_UTILIZATION_RATE = 80 * 1e16; // 80%
    uint256 public constant EXCESS_UTILIZATION_RATE = 20 * 1e16; // 20%
    
    function calculateInterestRates(
        uint256 totalDeposits,
        uint256 totalBorrows
    ) public pure returns (uint256, uint256) {
        uint256 utilizationRate = totalBorrows * 1e18 / totalDeposits;
        
        uint256 borrowRate;
        if (utilizationRate <= OPTIMAL_UTILIZATION_RATE) {
            borrowRate = baseRate + 
                (utilizationRate * rateSlope1) / 1e18;
        } else {
            borrowRate = baseRate + 
                (OPTIMAL_UTILIZATION_RATE * rateSlope1) / 1e18 +
                ((utilizationRate - OPTIMAL_UTILIZATION_RATE) * rateSlope2) / 1e18;
        }
        
        uint256 supplyRate = borrowRate * utilizationRate / 1e18;
        return (borrowRate, supplyRate);
    }
}
```

### 3. Collateral Manager
```solidity
contract CollateralManager {
    struct CollateralConfig {
        uint256 ltv;               // Loan to Value ratio
        uint256 liquidationThreshold;
        uint256 liquidationBonus;
        bool isActive;
    }
    
    mapping(address => CollateralConfig) public collateralConfigs;
    
    function calculateHealthFactor(
        address user
    ) public view returns (uint256) {
        (uint256 totalCollateral, uint256 totalDebt) = 
            getUserAccountData(user);
            
        if (totalDebt == 0) return type(uint256).max;
        
        return (totalCollateral * HEALTH_FACTOR_DECIMALS) / totalDebt;
    }
    
    function validateCollateral(
        address user,
        address asset,
        uint256 amount
    ) public view returns (bool) {
        CollateralConfig memory config = collateralConfigs[asset];
        require(config.isActive, "Asset not accepted as collateral");
        
        uint256 newHealthFactor = calculateNewHealthFactor(
            user,
            asset,
            amount
        );
        
        return newHealthFactor >= MINIMUM_HEALTH_FACTOR;
    }
}
```

## Liquidation Mechanism

### 1. Liquidation Contract
```solidity
contract LiquidationManager {
    uint256 public constant LIQUIDATION_CLOSE_FACTOR = 50; // 50%
    
    function liquidate(
        address collateral,
        address debt,
        address user,
        uint256 debtToCover
    ) external {
        require(
            canBeLiquidated(user, collateral, debt),
            "Position cannot be liquidated"
        );
        
        uint256 maxDebtToCover = getMaxDebtToCover(
            user,
            debt
        );
        
        require(
            debtToCover <= maxDebtToCover,
            "Amount exceeds max debt to cover"
        );
        
        // Execute liquidation
        executeLiquidation(
            collateral,
            debt,
            user,
            debtToCover
        );
    }
    
    function canBeLiquidated(
        address user,
        address collateral,
        address debt
    ) public view returns (bool) {
        uint256 healthFactor = calculateHealthFactor(user);
        return healthFactor < MINIMUM_HEALTH_FACTOR;
    }
}
```

### 2. Price Oracle
```solidity
contract PriceOracle {
    mapping(address => uint256) private prices;
    mapping(address => uint256) private lastUpdateTimestamp;
    
    function getAssetPrice(
        address asset
    ) external view returns (uint256) {
        require(
            isValidPrice(asset),
            "Price is stale"
        );
        return prices[asset];
    }
    
    function updatePrice(
        address asset,
        uint256 price
    ) external onlyOracle {
        prices[asset] = price;
        lastUpdateTimestamp[asset] = block.timestamp;
        emit PriceUpdated(asset, price);
    }
    
    function isValidPrice(
        address asset
    ) public view returns (bool) {
        return block.timestamp - lastUpdateTimestamp[asset] <= PRICE_EXPIRY;
    }
}
```

## Risk Management

### 1. Risk Parameters
```solidity
contract RiskManager {
    struct RiskParams {
        uint256 maxUtilizationRate;
        uint256 reserveFactor;
        uint256 liquidationPenalty;
        uint256 borrowCap;
    }
    
    mapping(address => RiskParams) public riskParams;
    
    function setRiskParams(
        address asset,
        RiskParams memory params
    ) external onlyRiskAdmin {
        validateRiskParams(params);
        riskParams[asset] = params;
        emit RiskParamsUpdated(asset, params);
    }
    
    function validateRiskParams(
        RiskParams memory params
    ) internal pure {
        require(
            params.maxUtilizationRate <= MAX_UTILIZATION_RATE,
            "Invalid utilization rate"
        );
        require(
            params.reserveFactor <= MAX_RESERVE_FACTOR,
            "Invalid reserve factor"
        );
    }
}
```

### 2. Emergency Procedures
```solidity
contract EmergencyControl {
    bool public paused;
    
    function pauseProtocol() external onlyEmergencyAdmin {
        paused = true;
        emit ProtocolPaused();
    }
    
    function unpauseProtocol() external onlyEmergencyAdmin {
        paused = false;
        emit ProtocolUnpaused();
    }
    
    function executeEmergencyWithdrawal(
        address asset,
        address user
    ) external whenPaused onlyEmergencyAdmin {
        // Emergency withdrawal logic
    }
}
```

## Integration

### 1. SDK Integration
```typescript
interface LendingSDK {
    // Asset Management
    getReserveData(asset: string): Promise<ReserveData>;
    getUserAccountData(user: string): Promise<AccountData>;
    
    // Lending Operations
    deposit(params: DepositParams): Promise<string>;
    withdraw(params: WithdrawParams): Promise<string>;
    borrow(params: BorrowParams): Promise<string>;
    repay(params: RepayParams): Promise<string>;
    
    // Collateral Management
    setUserUseReserveAsCollateral(
        asset: string,
        useAsCollateral: boolean
    ): Promise<string>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onDeposit(callback: (event: DepositEvent) => void): void;
    onBorrow(callback: (event: BorrowEvent) => void): void;
    onLiquidation(callback: (event: LiquidationEvent) => void): void;
    
    // Analytics
    getReserveStats(asset: string): Promise<ReserveStats>;
    getUserStats(user: string): Promise<UserStats>;
}
```
