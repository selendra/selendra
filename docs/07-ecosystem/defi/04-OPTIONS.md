# Options Protocol

## Overview

Selendra's Options Protocol enables:
- European/American options
- Covered calls/puts
- Options strategies
- Automated market making
- Greeks calculation

## Core Components

### 1. Options Factory
```solidity
contract OptionsFactory {
    struct OptionParameters {
        address underlying;
        uint256 strikePrice;
        uint256 expiry;
        bool isCall;
        OptionStyle style;
    }
    
    mapping(bytes32 => address) public options;
    
    function createOption(
        OptionParameters memory params
    ) external returns (address) {
        // Generate option ID
        bytes32 optionId = keccak256(abi.encode(params));
        require(options[optionId] == address(0), "Option exists");
        
        // Deploy option contract
        address option = address(new Option(params));
        options[optionId] = option;
        
        emit OptionCreated(optionId, option, params);
        
        return option;
    }
    
    function getOption(
        OptionParameters memory params
    ) external view returns (address) {
        bytes32 optionId = keccak256(abi.encode(params));
        return options[optionId];
    }
}
```

### 2. Option Contract
```solidity
contract Option {
    struct OptionData {
        address underlying;
        uint256 strikePrice;
        uint256 expiry;
        bool isCall;
        OptionStyle style;
        uint256 totalSupply;
        mapping(address => uint256) balances;
    }
    
    OptionData public optionData;
    
    function mint(uint256 amount) external {
        require(block.timestamp < optionData.expiry, "Expired");
        
        // Transfer collateral
        if (optionData.isCall) {
            IERC20(optionData.underlying).transferFrom(
                msg.sender,
                address(this),
                amount
            );
        } else {
            IERC20(optionData.underlying).transferFrom(
                msg.sender,
                address(this),
                amount * optionData.strikePrice / 1e18
            );
        }
        
        // Mint options
        optionData.balances[msg.sender] += amount;
        optionData.totalSupply += amount;
        
        emit OptionMinted(msg.sender, amount);
    }
    
    function exercise(uint256 amount) external {
        require(block.timestamp < optionData.expiry, "Expired");
        require(
            optionData.style == OptionStyle.American ||
            block.timestamp >= optionData.expiry - 1 days,
            "Not exercisable"
        );
        require(
            optionData.balances[msg.sender] >= amount,
            "Insufficient balance"
        );
        
        uint256 currentPrice = getPrice();
        
        if (optionData.isCall) {
            require(currentPrice > optionData.strikePrice, "Out of money");
            
            // Transfer strike price
            IERC20(optionData.underlying).transferFrom(
                msg.sender,
                address(this),
                amount * optionData.strikePrice / 1e18
            );
            
            // Transfer underlying
            IERC20(optionData.underlying).transfer(
                msg.sender,
                amount
            );
        } else {
            require(currentPrice < optionData.strikePrice, "Out of money");
            
            // Transfer underlying
            IERC20(optionData.underlying).transferFrom(
                msg.sender,
                address(this),
                amount
            );
            
            // Transfer strike price
            IERC20(optionData.underlying).transfer(
                msg.sender,
                amount * optionData.strikePrice / 1e18
            );
        }
        
        // Burn options
        optionData.balances[msg.sender] -= amount;
        optionData.totalSupply -= amount;
        
        emit OptionExercised(msg.sender, amount);
    }
}
```

### 3. Options AMM
```solidity
contract OptionsAMM {
    struct Pool {
        uint256 baseReserve;
        uint256 quoteReserve;
        uint256 totalSupply;
        mapping(address => uint256) balances;
    }
    
    mapping(address => Pool) public pools;
    
    function createPool(
        address option,
        uint256 baseAmount,
        uint256 quoteAmount
    ) external returns (uint256) {
        Pool storage pool = pools[option];
        require(pool.totalSupply == 0, "Pool exists");
        
        // Transfer tokens
        IERC20(option).transferFrom(msg.sender, address(this), baseAmount);
        IERC20(quoteToken).transferFrom(msg.sender, address(this), quoteAmount);
        
        // Initialize pool
        uint256 liquidity = Math.sqrt(baseAmount * quoteAmount);
        pool.baseReserve = baseAmount;
        pool.quoteReserve = quoteAmount;
        pool.totalSupply = liquidity;
        pool.balances[msg.sender] = liquidity;
        
        emit PoolCreated(option, baseAmount, quoteAmount);
        
        return liquidity;
    }
    
    function swap(
        address option,
        bool isBuy,
        uint256 amount
    ) external returns (uint256) {
        Pool storage pool = pools[option];
        require(pool.totalSupply > 0, "Pool not exists");
        
        // Calculate amounts
        uint256 inputAmount;
        uint256 outputAmount;
        
        if (isBuy) {
            inputAmount = amount;
            outputAmount = calculateSwapOutput(
                inputAmount,
                pool.quoteReserve,
                pool.baseReserve
            );
            
            // Transfer tokens
            IERC20(quoteToken).transferFrom(
                msg.sender,
                address(this),
                inputAmount
            );
            IERC20(option).transfer(msg.sender, outputAmount);
        } else {
            inputAmount = amount;
            outputAmount = calculateSwapOutput(
                inputAmount,
                pool.baseReserve,
                pool.quoteReserve
            );
            
            // Transfer tokens
            IERC20(option).transferFrom(
                msg.sender,
                address(this),
                inputAmount
            );
            IERC20(quoteToken).transfer(msg.sender, outputAmount);
        }
        
        // Update reserves
        if (isBuy) {
            pool.quoteReserve += inputAmount;
            pool.baseReserve -= outputAmount;
        } else {
            pool.baseReserve += inputAmount;
            pool.quoteReserve -= outputAmount;
        }
        
        emit Swap(option, msg.sender, isBuy, inputAmount, outputAmount);
        
        return outputAmount;
    }
}
```

## Option Strategies

### 1. Strategy Builder
```solidity
contract StrategyBuilder {
    struct Strategy {
        string name;
        OptionParameters[] legs;
        int256[] ratios;
        uint256 collateral;
    }
    
    mapping(bytes32 => Strategy) public strategies;
    
    function createStrategy(
        string memory name,
        OptionParameters[] memory legs,
        int256[] memory ratios
    ) external returns (bytes32) {
        require(legs.length == ratios.length, "Length mismatch");
        
        // Generate strategy ID
        bytes32 strategyId = keccak256(abi.encode(name, legs, ratios));
        
        // Create strategy
        strategies[strategyId] = Strategy({
            name: name,
            legs: legs,
            ratios: ratios,
            collateral: calculateCollateral(legs, ratios)
        });
        
        emit StrategyCreated(strategyId, name);
        
        return strategyId;
    }
    
    function executeStrategy(
        bytes32 strategyId,
        uint256 size
    ) external {
        Strategy memory strategy = strategies[strategyId];
        
        // Execute each leg
        for (uint256 i = 0; i < strategy.legs.length; i++) {
            if (strategy.ratios[i] > 0) {
                // Buy option
                optionsAMM.swap(
                    optionsFactory.getOption(strategy.legs[i]),
                    true,
                    uint256(strategy.ratios[i]) * size
                );
            } else {
                // Sell option
                optionsAMM.swap(
                    optionsFactory.getOption(strategy.legs[i]),
                    false,
                    uint256(-strategy.ratios[i]) * size
                );
            }
        }
        
        emit StrategyExecuted(strategyId, msg.sender, size);
    }
}
```

### 2. Greeks Calculator
```solidity
contract GreeksCalculator {
    using Math for uint256;
    
    struct Greeks {
        int256 delta;
        int256 gamma;
        int256 vega;
        int256 theta;
        int256 rho;
    }
    
    function calculateGreeks(
        address option
    ) external view returns (Greeks memory) {
        OptionData memory data = Option(option).optionData();
        
        uint256 S = getPrice();  // Current price
        uint256 K = data.strikePrice;  // Strike price
        uint256 T = data.expiry - block.timestamp;  // Time to expiry
        uint256 r = getInterestRate();  // Risk-free rate
        uint256 v = getVolatility();  // Implied volatility
        
        return Greeks({
            delta: calculateDelta(S, K, T, r, v, data.isCall),
            gamma: calculateGamma(S, K, T, r, v),
            vega: calculateVega(S, K, T, r, v),
            theta: calculateTheta(S, K, T, r, v, data.isCall),
            rho: calculateRho(S, K, T, r, v, data.isCall)
        });
    }
    
    function calculateImpliedVolatility(
        address option,
        uint256 price
    ) external view returns (uint256) {
        OptionData memory data = Option(option).optionData();
        
        // Newton-Raphson method
        uint256 v = 50e16;  // Initial guess: 50%
        for (uint256 i = 0; i < 100; i++) {
            uint256 diff = calculatePrice(data, v) - price;
            if (diff < 1e15) break;  // Precision: 0.1%
            
            uint256 vega = calculateVega(
                getPrice(),
                data.strikePrice,
                data.expiry - block.timestamp,
                getInterestRate(),
                v
            );
            
            v = v - diff / vega;
        }
        
        return v;
    }
}
```

## Risk Management

### 1. Position Manager
```solidity
contract PositionManager {
    struct Position {
        address[] options;
        int256[] amounts;
        uint256 collateral;
        uint256 margin;
    }
    
    mapping(address => Position[]) public positions;
    
    function openPosition(
        address[] memory options,
        int256[] memory amounts,
        uint256 collateral
    ) external {
        require(options.length == amounts.length, "Length mismatch");
        
        // Calculate required margin
        uint256 margin = calculateRequiredMargin(options, amounts);
        require(collateral >= margin, "Insufficient collateral");
        
        // Transfer collateral
        IERC20(collateralToken).transferFrom(
            msg.sender,
            address(this),
            collateral
        );
        
        // Create position
        positions[msg.sender].push(Position({
            options: options,
            amounts: amounts,
            collateral: collateral,
            margin: margin
        }));
        
        emit PositionOpened(msg.sender, positions[msg.sender].length - 1);
    }
    
    function closePosition(uint256 positionId) external {
        Position storage position = positions[msg.sender][positionId];
        
        // Calculate PnL
        int256 pnl = calculatePnL(position);
        
        // Return collateral
        if (pnl > 0) {
            IERC20(collateralToken).transfer(
                msg.sender,
                position.collateral + uint256(pnl)
            );
        } else {
            IERC20(collateralToken).transfer(
                msg.sender,
                position.collateral - uint256(-pnl)
            );
        }
        
        // Clear position
        delete positions[msg.sender][positionId];
        
        emit PositionClosed(msg.sender, positionId, pnl);
    }
}
```

### 2. Volatility Oracle
```solidity
contract VolatilityOracle {
    struct VolatilityData {
        uint256 timestamp;
        uint256 price;
        uint256 realized;
        uint256 implied;
    }
    
    mapping(address => VolatilityData[]) public volatilityHistory;
    
    function updateVolatility(address asset) external {
        uint256 price = getPrice(asset);
        uint256 realized = calculateRealizedVolatility(asset);
        uint256 implied = calculateImpliedVolatility(asset);
        
        volatilityHistory[asset].push(VolatilityData({
            timestamp: block.timestamp,
            price: price,
            realized: realized,
            implied: implied
        }));
        
        emit VolatilityUpdated(asset, realized, implied);
    }
    
    function getVolatility(
        address asset
    ) external view returns (uint256, uint256) {
        VolatilityData[] storage history = volatilityHistory[asset];
        require(history.length > 0, "No data");
        
        return (
            history[history.length - 1].realized,
            history[history.length - 1].implied
        );
    }
}
```

## Analytics

### 1. Options Analytics
```typescript
interface OptionsAnalytics {
    // Market Analytics
    getOpenInterest(): Promise<OpenInterest>;
    getVolatilitySurface(): Promise<VolatilitySurface>;
    
    // Greeks Analytics
    getPositionGreeks(): Promise<GreeksData>;
    getMarketGreeks(): Promise<MarketGreeks>;
    
    // Risk Analytics
    getExposureMetrics(): Promise<ExposureData>;
    getMarginUtilization(): Promise<MarginData>;
}
```

### 2. Strategy Analytics
```typescript
interface StrategyAnalytics {
    // Performance Analytics
    getStrategyPnL(): Promise<PnLData>;
    getStrategyMetrics(): Promise<StrategyMetrics>;
    
    // Risk Metrics
    getRiskExposure(): Promise<RiskData>;
    getStressTest(): Promise<StressTest>;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface OptionsSDK {
    // Option Operations
    createOption(params: OptionParams): Promise<string>;
    mintOption(optionId: string, amount: string): Promise<string>;
    exerciseOption(optionId: string, amount: string): Promise<string>;
    
    // Strategy Operations
    createStrategy(params: StrategyParams): Promise<string>;
    executeStrategy(strategyId: string, size: string): Promise<string>;
    
    // Analytics
    getOptionMetrics(optionId: string): Promise<OptionMetrics>;
    getStrategyMetrics(strategyId: string): Promise<StrategyMetrics>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onOptionCreated(callback: OptionCallback): Subscription;
    onStrategyExecuted(callback: StrategyCallback): Subscription;
    onPositionUpdate(callback: PositionCallback): Subscription;
    
    // Analytics
    getEventStats(): Promise<EventStats>;
    getMarketStats(): Promise<MarketStats>;
}
```
