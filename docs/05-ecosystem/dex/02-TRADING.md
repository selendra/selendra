# Trading Guide

## Trading Interface

### 1. Web Interface
```typescript
interface TradingInterface {
    // Token Selection
    getAvailablePairs(): Promise<TradingPair[]>;
    getTokenInfo(address: string): Promise<TokenInfo>;
    
    // Price Information
    getPrice(
        tokenIn: string,
        tokenOut: string,
        amount: string
    ): Promise<PriceInfo>;
    
    // Trade Execution
    executeTrade(trade: Trade): Promise<string>;
    getTradeStatus(txHash: string): Promise<TradeStatus>;
}
```

### 2. Mobile Interface
```typescript
interface MobileTrading {
    // Market Data
    getMarketOverview(): Promise<MarketOverview>;
    getTokenPrices(): Promise<TokenPrice[]>;
    
    // Trading
    createOrder(order: Order): Promise<string>;
    cancelOrder(orderId: string): Promise<void>;
}
```

## Trading Functions

### 1. Token Swaps
```solidity
interface ISelendraDEXRouter {
    function swapExactTokensForTokens(
        uint amountIn,
        uint amountOutMin,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);
    
    function swapTokensForExactTokens(
        uint amountOut,
        uint amountInMax,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);
}
```

### 2. Price Calculations
```solidity
contract PriceCalculator {
    function getAmountOut(
        uint amountIn,
        uint reserveIn,
        uint reserveOut
    ) public pure returns (uint amountOut) {
        require(amountIn > 0, 'INSUFFICIENT_INPUT_AMOUNT');
        require(reserveIn > 0 && reserveOut > 0, 'INSUFFICIENT_LIQUIDITY');
        
        uint amountInWithFee = amountIn * 997;
        uint numerator = amountInWithFee * reserveOut;
        uint denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }
    
    function getAmountIn(
        uint amountOut,
        uint reserveIn,
        uint reserveOut
    ) public pure returns (uint amountIn) {
        require(amountOut > 0, 'INSUFFICIENT_OUTPUT_AMOUNT');
        require(reserveIn > 0 && reserveOut > 0, 'INSUFFICIENT_LIQUIDITY');
        
        uint numerator = reserveIn * amountOut * 1000;
        uint denominator = (reserveOut - amountOut) * 997;
        amountIn = (numerator / denominator) + 1;
    }
}
```

## Order Types

### 1. Market Orders
```typescript
interface MarketOrder {
    tokenIn: string;
    tokenOut: string;
    amountIn: string;
    slippageTolerance: number;
    deadline: number;
}
```

### 2. Limit Orders
```solidity
contract LimitOrderManager {
    struct LimitOrder {
        address maker;
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 amountOut;
        uint256 deadline;
        bool executed;
    }
    
    mapping(bytes32 => LimitOrder) public orders;
    
    function createLimitOrder(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 deadline
    ) external returns (bytes32) {
        // Order creation logic
    }
    
    function executeOrder(bytes32 orderId) external {
        // Order execution logic
    }
}
```

## Trading Strategies

### 1. DCA (Dollar Cost Averaging)
```solidity
contract DCAStrategy {
    struct DCAOrder {
        address tokenIn;
        address tokenOut;
        uint256 amountPerTrade;
        uint256 interval;
        uint256 nextExecution;
        bool active;
    }
    
    mapping(address => DCAOrder[]) public userOrders;
    
    function createDCAOrder(
        address tokenIn,
        address tokenOut,
        uint256 amountPerTrade,
        uint256 interval
    ) external {
        // DCA order creation
    }
    
    function executeDCAOrders() external {
        // Execute due DCA orders
    }
}
```

### 2. Grid Trading
```solidity
contract GridStrategy {
    struct GridOrder {
        uint256 upperPrice;
        uint256 lowerPrice;
        uint256 gridSize;
        uint256[] gridLevels;
        bool active;
    }
    
    mapping(address => GridOrder) public userGrids;
    
    function createGridStrategy(
        uint256 upperPrice,
        uint256 lowerPrice,
        uint256 gridSize
    ) external {
        // Grid strategy creation
    }
    
    function executeGridTrades() external {
        // Execute grid trades
    }
}
```

## Risk Management

### 1. Slippage Protection
```solidity
contract SlippageController {
    function calculateSlippage(
        uint256 expectedPrice,
        uint256 executionPrice
    ) public pure returns (uint256) {
        return (expectedPrice - executionPrice) * 10000 / expectedPrice;
    }
    
    function validateSlippage(
        uint256 slippage,
        uint256 maxSlippage
    ) public pure returns (bool) {
        return slippage <= maxSlippage;
    }
}
```

### 2. Price Impact
```solidity
contract PriceImpactCalculator {
    function calculatePriceImpact(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) public pure returns (uint256) {
        // Price impact calculation
    }
    
    function validatePriceImpact(
        uint256 priceImpact,
        uint256 maxImpact
    ) public pure returns (bool) {
        return priceImpact <= maxImpact;
    }
}
```

## Analytics

### 1. Trading Volume
```typescript
interface VolumeAnalytics {
    // Volume Tracking
    trackTrade(trade: Trade): void;
    getDailyVolume(token: string): Promise<Volume>;
    getVolumeHistory(
        token: string,
        days: number
    ): Promise<VolumeHistory>;
}
```

### 2. Price Charts
```typescript
interface PriceAnalytics {
    // Price Data
    getPriceHistory(
        token: string,
        timeframe: Timeframe
    ): Promise<PricePoint[]>;
    
    // Technical Indicators
    calculateMA(
        prices: PricePoint[],
        period: number
    ): Promise<number[]>;
    
    calculateRSI(
        prices: PricePoint[],
        period: number
    ): Promise<number[]>;
}
```
