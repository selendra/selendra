# KHRt Technical Specification

## Important Notice

This technical paper describes the proposed implementation of the KHRt stablecoin system. All specifications are subject to change based on regulatory requirements, security considerations, and technological advancements.

## System Architecture

### 1. Token Implementation

```solidity
contract KHRt is ERC20, Pausable, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    
    constructor() ERC20("Khmer Riel Token", "KHRt") {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }
    
    function mint(address to, uint256 amount) external {
        require(hasRole(MINTER_ROLE, msg.sender), "Must have minter role");
        _mint(to, amount);
    }
    
    function burn(address from, uint256 amount) external {
        require(hasRole(BURNER_ROLE, msg.sender), "Must have burner role");
        _burn(from, amount);
    }
    
    function pause() external {
        require(hasRole(PAUSER_ROLE, msg.sender), "Must have pauser role");
        _pause();
    }
    
    function unpause() external {
        require(hasRole(PAUSER_ROLE, msg.sender), "Must have pauser role");
        _unpause();
    }
}
```

### 2. Reserve Management System

```solidity
contract KHRtReserve {
    struct ReserveAsset {
        string assetType;    // "cash", "bond", "stablecoin"
        address asset;       // Asset contract address if applicable
        uint256 amount;      // Amount in smallest unit
        uint256 value;      // Value in KHR
    }
    
    struct ReserveRatio {
        uint256 cashRatio;      // Target: 40%
        uint256 bondRatio;      // Target: 40%
        uint256 stablecoinRatio;// Target: 20%
    }
    
    mapping(bytes32 => ReserveAsset) public reserves;
    ReserveRatio public targetRatio;
    
    event ReserveUpdated(string assetType, uint256 amount, uint256 value);
    event RatioUpdated(uint256 cashRatio, uint256 bondRatio, uint256 stablecoinRatio);
    
    function addReserve(
        string memory assetType,
        address asset,
        uint256 amount,
        uint256 value
    ) external onlyReserveManager {
        bytes32 key = keccak256(abi.encodePacked(assetType, asset));
        reserves[key] = ReserveAsset(assetType, asset, amount, value);
        emit ReserveUpdated(assetType, amount, value);
    }
    
    function updateTargetRatio(
        uint256 _cashRatio,
        uint256 _bondRatio,
        uint256 _stablecoinRatio
    ) external onlyAdmin {
        require(_cashRatio + _bondRatio + _stablecoinRatio == 100, "Invalid ratio");
        targetRatio = ReserveRatio(_cashRatio, _bondRatio, _stablecoinRatio);
        emit RatioUpdated(_cashRatio, _bondRatio, _stablecoinRatio);
    }
    
    function getReserveStatus() public view returns (
        uint256 totalValue,
        uint256 currentCashRatio,
        uint256 currentBondRatio,
        uint256 currentStablecoinRatio
    ) {
        // Calculate total value and ratios
        // Implementation details
    }
}
```

### 3. Liquidity Management

```solidity
contract KHRtLiquidity {
    struct LiquidityPool {
        uint256 cashReserve;
        uint256 stablecoinReserve;
        mapping(address => uint256) bankBalances;
        mapping(address => uint256) stablecoinBalances;
    }
    
    LiquidityPool public pool;
    
    function addCashLiquidity(address bank, uint256 amount) external onlyLiquidityProvider {
        pool.bankBalances[bank] += amount;
        pool.cashReserve += amount;
    }
    
    function addStablecoinLiquidity(
        address stablecoin,
        uint256 amount
    ) external onlyLiquidityProvider {
        pool.stablecoinBalances[stablecoin] += amount;
        pool.stablecoinReserve += amount;
    }
    
    function getOptimalLiquidity() public view returns (
        uint256 recommendedCash,
        uint256 recommendedStablecoin
    ) {
        // Calculate based on historical usage patterns
        // Implementation details
    }
}
```

## Reserve Mechanism

### 1. Asset Allocation

The reserve system is designed to maintain three types of assets:

1. **Cash Reserves (40%)**
   - Held in regulated Cambodian banks
   - Minimum 20% in immediate-access accounts
   - Maximum 20% in fixed-term deposits
   - Daily liquidity monitoring

2. **Government Bonds (40%)**
   - Cambodia government securities
   - Maximum maturity of 5 years
   - Laddered maturity structure
   - Regular yield optimization

3. **Stablecoin Reserves (20%)**
   - USDC: 15%
   - Other regulated stablecoins: 5%
   - Real-time liquidity monitoring
   - Automated rebalancing

### 2. Rebalancing Mechanism

```solidity
contract KHRtRebalancer {
    struct RebalanceThresholds {
        uint256 minCashRatio;      // 35%
        uint256 maxCashRatio;      // 45%
        uint256 minBondRatio;      // 35%
        uint256 maxBondRatio;      // 45%
        uint256 minStablecoinRatio;// 15%
        uint256 maxStablecoinRatio;// 25%
    }
    
    function checkRebalance() external returns (bool needed) {
        (
            uint256 totalValue,
            uint256 currentCashRatio,
            uint256 currentBondRatio,
            uint256 currentStablecoinRatio
        ) = reserve.getReserveStatus();
        
        if (isOutsideThresholds(
            currentCashRatio,
            currentBondRatio,
            currentStablecoinRatio
        )) {
            initiateRebalancing();
            return true;
        }
        return false;
    }
    
    function initiateRebalancing() internal {
        // Rebalancing logic
        // Implementation details
    }
}
```

### 3. Oracle System

```solidity
contract KHRtOracle {
    struct PriceData {
        uint256 price;
        uint256 timestamp;
        string source;
        uint256 confidence;
    }
    
    mapping(address => PriceData) public assetPrices;
    
    function updatePrice(
        address asset,
        uint256 price,
        string memory source,
        uint256 confidence
    ) external onlyOracle {
        require(confidence >= minimumConfidence, "Low confidence");
        assetPrices[asset] = PriceData(
            price,
            block.timestamp,
            source,
            confidence
        );
        emit PriceUpdated(asset, price, source, confidence);
    }
    
    function getPrice(address asset) external view returns (PriceData memory) {
        PriceData memory data = assetPrices[asset];
        require(
            block.timestamp - data.timestamp <= maxAge,
            "Price too old"
        );
        return data;
    }
}
```

## Risk Management

### 1. Transaction Monitoring

```solidity
contract KHRtMonitoring {
    struct TransactionLimit {
        uint256 daily;
        uint256 single;
        uint256 monthly;
    }
    
    mapping(address => TransactionLimit) public limits;
    mapping(address => uint256) public dailyVolume;
    mapping(address => uint256) public monthlyVolume;
    
    function checkLimit(
        address user,
        uint256 amount
    ) external view returns (bool) {
        TransactionLimit memory userLimits = limits[user];
        require(amount <= userLimits.single, "Exceeds single tx limit");
        require(
            dailyVolume[user] + amount <= userLimits.daily,
            "Exceeds daily limit"
        );
        require(
            monthlyVolume[user] + amount <= userLimits.monthly,
            "Exceeds monthly limit"
        );
        return true;
    }
}
```

### 2. Emergency Procedures

```solidity
contract KHRtEmergency {
    enum EmergencyState { Normal, Caution, Restricted, Halted }
    
    EmergencyState public currentState;
    
    function declareEmergency(
        EmergencyState state
    ) external onlyEmergencyCommittee {
        currentState = state;
        if (state == EmergencyState.Halted) {
            token.pause();
        }
        emit EmergencyDeclared(state);
    }
    
    function executeEmergencyPlan(
        bytes calldata plan
    ) external onlyEmergencyCommittee {
        require(currentState != EmergencyState.Normal, "No emergency");
        // Execute emergency procedures
        // Implementation details
    }
}
```

## Integration Interfaces

### 1. Payment Gateway API

```typescript
interface KHRtPaymentGateway {
    // Payment Processing
    function processPayment(PaymentRequest): Promise<PaymentResult>;
    function refundPayment(RefundRequest): Promise<RefundResult>;
    
    // Subscription Management
    function createSubscription(SubscriptionRequest): Promise<SubscriptionResult>;
    function cancelSubscription(string subscriptionId): Promise<boolean>;
    
    // Reporting
    function getTransactionHistory(HistoryRequest): Promise<Transaction[]>;
    function generateReport(ReportRequest): Promise<Report>;
}
```

### 2. Banking Integration API

```typescript
interface KHRtBankingAPI {
    // Account Operations
    function deposit(DepositRequest): Promise<DepositResult>;
    function withdraw(WithdrawRequest): Promise<WithdrawResult>;
    
    // Account Management
    function linkBank(LinkRequest): Promise<LinkResult>;
    function unlinkBank(string bankId): Promise<boolean>;
    
    // Balance Checking
    function checkBalance(string accountId): Promise<Balance>;
    function getReserveStatus(): Promise<ReserveStatus>;
}
```

## Technical Roadmap

### Phase 1: Foundation (Q1-Q2 2025)
- Core smart contract development
- Basic reserve management
- Initial security implementation

### Phase 2: Enhancement (Q3-Q4 2025)
- Advanced monitoring systems
- Automated rebalancing
- Enhanced security features

### Phase 3: Scaling (2026+)
- Performance optimizations
- Advanced features
- Cross-chain capabilities

## Security Considerations

1. **Smart Contract Security**
   - Multiple audit requirements
   - Formal verification
   - Bug bounty program

2. **Operational Security**
   - Multi-signature requirements
   - Role-based access control
   - Regular security reviews

3. **Reserve Security**
   - Multi-layer approval process
   - Real-time monitoring
   - Automated alerts

## Technical Disclaimer

1. This technical specification is subject to change based on:
   - Regulatory requirements
   - Security considerations
   - Technical improvements
   - Market conditions

2. All code examples are illustrative and may not represent final implementation.

3. Security features and risk management systems will be thoroughly tested before deployment.
