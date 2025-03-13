# Asset Bridge

## Overview

Selendra's Asset Bridge enables:
- Cross-chain token transfers
- Native token wrapping
- Token standards conversion
- Liquidity management
- Asset verification

## Core Components

### 1. Bridge Core
```solidity
contract AssetBridge {
    struct BridgeConfig {
        uint256 minAmount;
        uint256 maxAmount;
        uint256 dailyLimit;
        uint256 fee;
    }
    
    struct TokenConfig {
        address sourceToken;
        address targetToken;
        uint256 targetChain;
        bool paused;
    }
    
    mapping(bytes32 => TokenConfig) public tokenConfigs;
    mapping(uint256 => BridgeConfig) public bridgeConfigs;
    
    function bridgeToken(
        address token,
        uint256 targetChain,
        address recipient,
        uint256 amount
    ) external payable {
        // Get token config
        bytes32 configId = getTokenConfigId(token, targetChain);
        TokenConfig memory config = tokenConfigs[configId];
        require(!config.paused, "Bridge paused");
        
        // Validate amount
        BridgeConfig memory bridgeConfig = bridgeConfigs[targetChain];
        require(
            amount >= bridgeConfig.minAmount &&
            amount <= bridgeConfig.maxAmount,
            "Invalid amount"
        );
        
        // Check daily limit
        require(
            !exceedsDailyLimit(targetChain, amount),
            "Exceeds limit"
        );
        
        // Transfer tokens
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        
        // Create bridge message
        bytes memory message = encodeBridgeMessage(
            token,
            recipient,
            amount
        );
        
        // Send message
        bytes32 messageId = messageRouter.sendMessage(
            targetChain,
            config.targetToken,
            message
        );
        
        emit TokensBridged(messageId, token, amount);
    }
    
    function claimTokens(
        bytes32 messageId,
        bytes memory proof
    ) external {
        // Verify message
        Message memory message = messageRouter.verifyMessage(
            messageId,
            proof
        );
        
        // Decode bridge data
        (
            address token,
            address recipient,
            uint256 amount
        ) = decodeBridgeMessage(message.payload);
        
        // Transfer tokens
        IERC20(token).transfer(recipient, amount);
        
        emit TokensClaimed(messageId, token, amount);
    }
}
```

### 2. Token Wrapper
```solidity
contract TokenWrapper {
    struct WrappedToken {
        address original;
        uint256 sourceChain;
        string name;
        string symbol;
        uint8 decimals;
    }
    
    mapping(address => WrappedToken) public wrappedTokens;
    
    function createWrappedToken(
        address original,
        uint256 sourceChain,
        string memory name,
        string memory symbol,
        uint8 decimals
    ) external returns (address) {
        // Deploy wrapped token
        address wrapped = address(new WrappedToken(
            name,
            symbol,
            decimals
        ));
        
        // Store config
        wrappedTokens[wrapped] = WrappedToken({
            original: original,
            sourceChain: sourceChain,
            name: name,
            symbol: symbol,
            decimals: decimals
        });
        
        emit WrappedTokenCreated(wrapped, original);
        
        return wrapped;
    }
    
    function mint(
        address token,
        address recipient,
        uint256 amount
    ) external onlyBridge {
        require(
            wrappedTokens[token].original != address(0),
            "Not wrapped token"
        );
        
        IWrappedToken(token).mint(recipient, amount);
        
        emit TokensMinted(token, recipient, amount);
    }
    
    function burn(
        address token,
        address from,
        uint256 amount
    ) external onlyBridge {
        require(
            wrappedTokens[token].original != address(0),
            "Not wrapped token"
        );
        
        IWrappedToken(token).burn(from, amount);
        
        emit TokensBurned(token, from, amount);
    }
}
```

### 3. Liquidity Manager
```solidity
contract LiquidityManager {
    struct Pool {
        address token;
        uint256 balance;
        uint256 minLiquidity;
        uint256 maxLiquidity;
    }
    
    mapping(address => Pool) public pools;
    
    function addLiquidity(
        address token,
        uint256 amount
    ) external {
        Pool storage pool = pools[token];
        require(
            pool.balance + amount <= pool.maxLiquidity,
            "Exceeds max liquidity"
        );
        
        // Transfer tokens
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        pool.balance += amount;
        
        emit LiquidityAdded(token, amount);
    }
    
    function removeLiquidity(
        address token,
        uint256 amount
    ) external {
        Pool storage pool = pools[token];
        require(
            pool.balance - amount >= pool.minLiquidity,
            "Below min liquidity"
        );
        
        // Transfer tokens
        IERC20(token).transfer(msg.sender, amount);
        pool.balance -= amount;
        
        emit LiquidityRemoved(token, amount);
    }
    
    function rebalancePools(
        address[] memory tokens,
        uint256[] memory amounts
    ) external onlyAdmin {
        require(
            tokens.length == amounts.length,
            "Length mismatch"
        );
        
        for (uint256 i = 0; i < tokens.length; i++) {
            Pool storage pool = pools[tokens[i]];
            
            if (pool.balance < amounts[i]) {
                // Need more liquidity
                uint256 deficit = amounts[i] - pool.balance;
                requestLiquidity(tokens[i], deficit);
            } else if (pool.balance > amounts[i]) {
                // Excess liquidity
                uint256 excess = pool.balance - amounts[i];
                releaseLiquidity(tokens[i], excess);
            }
        }
        
        emit PoolsRebalanced();
    }
}
```

## Bridge Security

### 1. Security Manager
```solidity
contract SecurityManager {
    struct SecurityConfig {
        uint256 confirmations;
        uint256 challengePeriod;
        uint256 challengeAmount;
        uint256 slashAmount;
    }
    
    SecurityConfig public config;
    
    function challengeTransfer(
        bytes32 transferId,
        bytes memory evidence
    ) external {
        require(
            isWithinChallengePeriod(transferId),
            "Challenge period ended"
        );
        
        // Lock challenge amount
        token.transferFrom(msg.sender, address(this), config.challengeAmount);
        
        // Create challenge
        challenges[transferId] = Challenge({
            challenger: msg.sender,
            evidence: evidence,
            timestamp: block.timestamp,
            resolved: false
        });
        
        emit TransferChallenged(transferId, msg.sender);
    }
    
    function resolveChallenge(
        bytes32 transferId,
        bool valid
    ) external onlyValidator {
        Challenge storage challenge = challenges[transferId];
        require(!challenge.resolved, "Already resolved");
        
        if (valid) {
            // Slash relayer
            slashRelayer(transferId);
            
            // Reward challenger
            token.transfer(challenge.challenger, config.slashAmount);
        } else {
            // Slash challenger
            token.transfer(address(this), config.challengeAmount);
        }
        
        challenge.resolved = true;
        emit ChallengeResolved(transferId, valid);
    }
}
```

### 2. Fraud Proof
```solidity
contract FraudProof {
    struct Proof {
        bytes32 transferId;
        bytes evidence;
        uint256 timestamp;
        bool verified;
    }
    
    mapping(bytes32 => Proof) public proofs;
    
    function submitProof(
        bytes32 transferId,
        bytes memory evidence
    ) external {
        require(
            !proofs[transferId].verified,
            "Already verified"
        );
        
        // Verify evidence
        require(
            verifyEvidence(transferId, evidence),
            "Invalid evidence"
        );
        
        // Store proof
        proofs[transferId] = Proof({
            transferId: transferId,
            evidence: evidence,
            timestamp: block.timestamp,
            verified: true
        });
        
        // Halt transfer
        bridge.haltTransfer(transferId);
        
        emit ProofSubmitted(transferId);
    }
    
    function verifyEvidence(
        bytes32 transferId,
        bytes memory evidence
    ) internal view returns (bool) {
        // Implement evidence verification logic
        // This could include:
        // - Double spend detection
        // - Invalid signature verification
        // - Incorrect amount verification
        // - etc.
    }
}
```

## Bridge Analytics

### 1. Transfer Analytics
```typescript
interface TransferAnalytics {
    // Volume Analytics
    getTransferVolume(): Promise<VolumeData>;
    getTokenDistribution(): Promise<Distribution>;
    
    // Bridge Stats
    getBridgeUtilization(): Promise<UtilizationData>;
    getLiquidityMetrics(): Promise<LiquidityData>;
    
    // Security Stats
    getChallengeStats(): Promise<ChallengeStats>;
    getFraudStats(): Promise<FraudData>;
}
```

### 2. Performance Analytics
```typescript
interface PerformanceAnalytics {
    // Speed Analytics
    getTransferSpeed(): Promise<SpeedMetrics>;
    getConfirmationTimes(): Promise<ConfirmationData>;
    
    // Cost Analytics
    getFeeAnalytics(): Promise<FeeData>;
    getGasAnalytics(): Promise<GasData>;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface BridgeSDK {
    // Bridge Operations
    bridgeToken(params: BridgeParams): Promise<string>;
    claimToken(transferId: string, proof: string): Promise<string>;
    
    // Liquidity Operations
    addLiquidity(token: string, amount: string): Promise<string>;
    removeLiquidity(token: string, amount: string): Promise<string>;
    
    // Security Operations
    submitChallenge(transferId: string, evidence: string): Promise<string>;
    verifyTransfer(transferId: string): Promise<boolean>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onTransfer(callback: TransferCallback): Subscription;
    onLiquidity(callback: LiquidityCallback): Subscription;
    onChallenge(callback: ChallengeCallback): Subscription;
    
    // Analytics
    getEventStats(): Promise<EventStats>;
    getBridgeStats(): Promise<BridgeStats>;
}
```
