# Derivatives Protocol

## Overview

Selendra's Derivatives Protocol enables:
- Perpetual futures
- Options trading
- Synthetic assets
- Cross-margin trading
- Risk management

## Core Components

### 1. Perpetual Protocol
```solidity
contract PerpetualProtocol {
    struct Position {
        address trader;
        int256 size;
        uint256 margin;
        uint256 entryPrice;
        uint256 liquidationPrice;
        bool isLong;
    }
    
    mapping(address => mapping(bytes32 => Position)) public positions;
    
    function openPosition(
        bytes32 marketId,
        bool isLong,
        uint256 size,
        uint256 margin
    ) external {
        require(margin >= minMargin, "Insufficient margin");
        
        // Calculate entry price
        uint256 entryPrice = getMarkPrice(marketId);
        
        // Calculate liquidation price
        uint256 liquidationPrice = calculateLiquidationPrice(
            isLong,
            entryPrice,
            margin,
            size
        );
        
        // Create position
        positions[msg.sender][marketId] = Position({
            trader: msg.sender,
            size: isLong ? int256(size) : -int256(size),
            margin: margin,
            entryPrice: entryPrice,
            liquidationPrice: liquidationPrice,
            isLong: isLong
        });
        
        emit PositionOpened(msg.sender, marketId, size, margin);
    }
    
    function closePosition(
        bytes32 marketId
    ) external {
        Position memory position = positions[msg.sender][marketId];
        require(position.size != 0, "No position");
        
        // Calculate PnL
        int256 pnl = calculatePnL(position);
        
        // Transfer funds
        if (pnl > 0) {
            token.transfer(msg.sender, uint256(pnl));
        }
        
        // Return margin
        token.transfer(msg.sender, position.margin);
        
        // Clear position
        delete positions[msg.sender][marketId];
        
        emit PositionClosed(msg.sender, marketId, pnl);
    }
}
```

### 2. Options Protocol
```solidity
contract OptionsProtocol {
    struct Option {
        address writer;
        address holder;
        uint256 strikePrice;
        uint256 premium;
        uint256 expiry;
        bool isCall;
        bool exercised;
    }
    
    mapping(uint256 => Option) public options;
    uint256 public optionCount;
    
    function writeOption(
        uint256 strikePrice,
        uint256 premium,
        uint256 duration,
        bool isCall
    ) external returns (uint256) {
        uint256 optionId = ++optionCount;
        
        // Create option
        options[optionId] = Option({
            writer: msg.sender,
            holder: address(0),
            strikePrice: strikePrice,
            premium: premium,
            expiry: block.timestamp + duration,
            isCall: isCall,
            exercised: false
        });
        
        // Lock collateral
        if (isCall) {
            token.transferFrom(msg.sender, address(this), 1 ether);
        } else {
            token.transferFrom(msg.sender, address(this), strikePrice);
        }
        
        emit OptionWritten(optionId, msg.sender, strikePrice);
        
        return optionId;
    }
    
    function buyOption(uint256 optionId) external {
        Option storage option = options[optionId];
        require(option.holder == address(0), "Already bought");
        require(block.timestamp < option.expiry, "Expired");
        
        // Pay premium
        token.transferFrom(msg.sender, option.writer, option.premium);
        
        // Assign holder
        option.holder = msg.sender;
        
        emit OptionBought(optionId, msg.sender);
    }
    
    function exerciseOption(uint256 optionId) external {
        Option storage option = options[optionId];
        require(msg.sender == option.holder, "Not holder");
        require(!option.exercised, "Already exercised");
        require(block.timestamp < option.expiry, "Expired");
        
        uint256 currentPrice = getPriceOracle().getPrice();
        
        if (option.isCall) {
            require(currentPrice > option.strikePrice, "Out of money");
            
            // Transfer underlying
            token.transfer(msg.sender, 1 ether);
            
            // Collect payment
            token.transferFrom(
                msg.sender,
                option.writer,
                option.strikePrice
            );
        } else {
            require(currentPrice < option.strikePrice, "Out of money");
            
            // Transfer strike price
            token.transfer(msg.sender, option.strikePrice);
            
            // Collect underlying
            token.transferFrom(msg.sender, option.writer, 1 ether);
        }
        
        option.exercised = true;
        emit OptionExercised(optionId);
    }
}
```

### 3. Synthetic Assets
```solidity
contract SyntheticProtocol {
    struct Synthetic {
        bytes32 assetId;
        uint256 collateralRatio;
        uint256 mintingFee;
        uint256 burningFee;
        bool paused;
    }
    
    mapping(bytes32 => Synthetic) public synthetics;
    mapping(address => mapping(bytes32 => uint256)) public positions;
    
    function createSynthetic(
        bytes32 assetId,
        uint256 collateralRatio,
        uint256 mintingFee,
        uint256 burningFee
    ) external onlyGovernance {
        synthetics[assetId] = Synthetic({
            assetId: assetId,
            collateralRatio: collateralRatio,
            mintingFee: mintingFee,
            burningFee: burningFee,
            paused: false
        });
        
        emit SyntheticCreated(assetId);
    }
    
    function mintSynthetic(
        bytes32 assetId,
        uint256 amount
    ) external {
        Synthetic memory synthetic = synthetics[assetId];
        require(!synthetic.paused, "Paused");
        
        // Calculate collateral
        uint256 price = getPriceOracle().getPrice(assetId);
        uint256 collateral = (amount * price * synthetic.collateralRatio) / 1e18;
        
        // Calculate fee
        uint256 fee = (amount * synthetic.mintingFee) / 1e18;
        
        // Lock collateral
        token.transferFrom(msg.sender, address(this), collateral + fee);
        
        // Mint synthetic
        positions[msg.sender][assetId] += amount;
        
        emit SyntheticMinted(msg.sender, assetId, amount);
    }
    
    function burnSynthetic(
        bytes32 assetId,
        uint256 amount
    ) external {
        require(
            positions[msg.sender][assetId] >= amount,
            "Insufficient balance"
        );
        
        Synthetic memory synthetic = synthetics[assetId];
        
        // Calculate collateral
        uint256 price = getPriceOracle().getPrice(assetId);
        uint256 collateral = (amount * price * synthetic.collateralRatio) / 1e18;
        
        // Calculate fee
        uint256 fee = (amount * synthetic.burningFee) / 1e18;
        
        // Burn synthetic
        positions[msg.sender][assetId] -= amount;
        
        // Return collateral
        token.transfer(msg.sender, collateral - fee);
        
        emit SyntheticBurned(msg.sender, assetId, amount);
    }
}
```

## Risk Management

### 1. Liquidation Engine
```solidity
contract LiquidationEngine {
    struct LiquidationConfig {
        uint256 threshold;
        uint256 bonus;
        uint256 penalty;
    }
    
    mapping(bytes32 => LiquidationConfig) public configs;
    
    function checkLiquidation(
        address trader,
        bytes32 marketId
    ) external returns (bool) {
        Position memory position = perpProtocol.getPosition(
            trader,
            marketId
        );
        
        if (position.size == 0) return false;
        
        uint256 currentPrice = getPriceOracle().getPrice(marketId);
        
        if (position.isLong) {
            return currentPrice <= position.liquidationPrice;
        } else {
            return currentPrice >= position.liquidationPrice;
        }
    }
    
    function liquidate(
        address trader,
        bytes32 marketId
    ) external {
        require(
            checkLiquidation(trader, marketId),
            "Not liquidatable"
        );
        
        Position memory position = perpProtocol.getPosition(
            trader,
            marketId
        );
        
        // Calculate distribution
        uint256 bonus = (position.margin * configs[marketId].bonus) / 1e18;
        uint256 penalty = (position.margin * configs[marketId].penalty) / 1e18;
        uint256 insurance = position.margin - bonus - penalty;
        
        // Distribute funds
        token.transfer(msg.sender, bonus);  // Liquidator
        token.transfer(address(insuranceFund), insurance);
        
        // Clear position
        perpProtocol.closePosition(trader, marketId);
        
        emit PositionLiquidated(trader, marketId, msg.sender);
    }
}
```

### 2. Insurance Fund
```solidity
contract InsuranceFund {
    struct Fund {
        uint256 balance;
        uint256 targetSize;
        uint256 withdrawalDelay;
    }
    
    Fund public fund;
    mapping(address => uint256) public withdrawalRequests;
    
    function deposit() external payable {
        fund.balance += msg.value;
        emit Deposited(msg.sender, msg.value);
    }
    
    function requestWithdrawal(uint256 amount) external {
        require(
            amount <= getWithdrawableAmount(),
            "Exceeds withdrawable"
        );
        
        withdrawalRequests[msg.sender] = block.timestamp + fund.withdrawalDelay;
        emit WithdrawalRequested(msg.sender, amount);
    }
    
    function withdraw() external {
        uint256 requestTime = withdrawalRequests[msg.sender];
        require(requestTime > 0, "No request");
        require(
            block.timestamp >= requestTime,
            "Too early"
        );
        
        uint256 amount = getWithdrawableAmount();
        require(amount > 0, "Nothing to withdraw");
        
        delete withdrawalRequests[msg.sender];
        fund.balance -= amount;
        
        payable(msg.sender).transfer(amount);
        emit Withdrawn(msg.sender, amount);
    }
}
```

## Analytics

### 1. Trading Analytics
```typescript
interface TradingAnalytics {
    // Position Analytics
    getOpenPositions(): Promise<PositionData>;
    getTradeHistory(): Promise<TradeHistory>;
    
    // Risk Analytics
    getLiquidationRisk(): Promise<RiskMetrics>;
    getMarginUtilization(): Promise<MarginData>;
    
    // Market Analytics
    getVolumeData(): Promise<VolumeData>;
    getOpenInterest(): Promise<OpenInterest>;
}
```

### 2. Performance Analytics
```typescript
interface PerformanceAnalytics {
    // PnL Analytics
    getPnLMetrics(): Promise<PnLMetrics>;
    getTraderPerformance(): Promise<Performance>;
    
    // Risk Metrics
    getVolatilityMetrics(): Promise<Volatility>;
    getLiquidationStats(): Promise<LiquidationStats>;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface DerivativesSDK {
    // Perpetual Operations
    openPosition(params: PositionParams): Promise<string>;
    closePosition(marketId: string): Promise<string>;
    
    // Options Operations
    writeOption(params: OptionParams): Promise<string>;
    exerciseOption(optionId: string): Promise<string>;
    
    // Synthetic Operations
    mintSynthetic(params: MintParams): Promise<string>;
    burnSynthetic(params: BurnParams): Promise<string>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onPositionUpdate(callback: PositionCallback): Subscription;
    onOptionEvent(callback: OptionCallback): Subscription;
    onLiquidation(callback: LiquidationCallback): Subscription;
    
    // Analytics
    getEventStats(): Promise<EventStats>;
    getMarketStats(): Promise<MarketStats>;
}
```
