# Cross-Chain Messaging

## Overview

Selendra's Cross-Chain Messaging enables:
- Message passing between chains
- Cross-chain contract calls
- State synchronization
- Event propagation
- Message verification

## Core Components

### 1. Message Router
```solidity
contract MessageRouter {
    struct Message {
        uint256 nonce;
        uint256 sourceChain;
        uint256 targetChain;
        address sender;
        address recipient;
        bytes payload;
        MessageStatus status;
    }
    
    mapping(bytes32 => Message) public messages;
    mapping(uint256 => uint256) public nonces;
    
    function sendMessage(
        uint256 targetChain,
        address recipient,
        bytes calldata payload
    ) external payable returns (bytes32) {
        // Generate message ID
        uint256 nonce = ++nonces[targetChain];
        bytes32 messageId = keccak256(
            abi.encode(
                nonce,
                block.chainid,
                targetChain,
                msg.sender,
                recipient,
                payload
            )
        );
        
        // Store message
        messages[messageId] = Message({
            nonce: nonce,
            sourceChain: block.chainid,
            targetChain: targetChain,
            sender: msg.sender,
            recipient: recipient,
            payload: payload,
            status: MessageStatus.Pending
        });
        
        emit MessageSent(messageId, targetChain, recipient);
        
        return messageId;
    }
    
    function executeMessage(
        bytes32 messageId,
        bytes memory proof
    ) external {
        Message storage message = messages[messageId];
        require(
            message.status == MessageStatus.Pending,
            "Invalid status"
        );
        
        // Verify proof
        require(
            verifyMessageProof(messageId, proof),
            "Invalid proof"
        );
        
        // Execute message
        (bool success, bytes memory result) = message.recipient.call(
            abi.encodePacked(message.payload, message.sender)
        );
        
        require(success, string(result));
        
        // Update status
        message.status = MessageStatus.Executed;
        
        emit MessageExecuted(messageId);
    }
}
```

### 2. Message Verifier
```solidity
contract MessageVerifier {
    struct Checkpoint {
        uint256 blockNumber;
        bytes32 blockHash;
        bytes32 stateRoot;
        bytes signature;
    }
    
    mapping(uint256 => mapping(uint256 => Checkpoint)) public checkpoints;
    
    function verifyMessageProof(
        bytes32 messageId,
        bytes memory proof
    ) public view returns (bool) {
        // Decode proof
        (
            uint256 blockNumber,
            bytes32 blockHash,
            bytes32 stateRoot,
            bytes memory signature,
            bytes memory messageProof
        ) = abi.decode(
            proof,
            (uint256, bytes32, bytes32, bytes, bytes)
        );
        
        // Verify checkpoint
        require(
            verifyCheckpoint(
                blockNumber,
                blockHash,
                stateRoot,
                signature
            ),
            "Invalid checkpoint"
        );
        
        // Verify message inclusion
        require(
            verifyMessageInclusion(
                messageId,
                stateRoot,
                messageProof
            ),
            "Invalid message proof"
        );
        
        return true;
    }
    
    function submitCheckpoint(
        uint256 chainId,
        uint256 blockNumber,
        bytes32 blockHash,
        bytes32 stateRoot,
        bytes memory signature
    ) external {
        require(
            isValidator(msg.sender),
            "Not validator"
        );
        
        // Verify signature
        require(
            verifySignature(
                chainId,
                blockNumber,
                blockHash,
                stateRoot,
                signature
            ),
            "Invalid signature"
        );
        
        // Store checkpoint
        checkpoints[chainId][blockNumber] = Checkpoint({
            blockNumber: blockNumber,
            blockHash: blockHash,
            stateRoot: stateRoot,
            signature: signature
        });
        
        emit CheckpointSubmitted(chainId, blockNumber);
    }
}
```

### 3. Message Executor
```solidity
contract MessageExecutor {
    struct Execution {
        bytes32 messageId;
        address executor;
        uint256 timestamp;
        bytes result;
        ExecutionStatus status;
    }
    
    mapping(bytes32 => Execution) public executions;
    
    function executeMessage(
        bytes32 messageId,
        bytes memory proof
    ) external {
        require(
            !isExecuted(messageId),
            "Already executed"
        );
        
        // Verify message
        Message memory message = messageRouter.verifyMessage(
            messageId,
            proof
        );
        
        // Execute message
        (bool success, bytes memory result) = message.recipient.call(
            abi.encodePacked(message.payload, message.sender)
        );
        
        // Store execution
        executions[messageId] = Execution({
            messageId: messageId,
            executor: msg.sender,
            timestamp: block.timestamp,
            result: result,
            status: success ? ExecutionStatus.Success : ExecutionStatus.Failed
        });
        
        emit MessageExecuted(messageId, success);
    }
    
    function retryExecution(bytes32 messageId) external {
        Execution storage execution = executions[messageId];
        require(
            execution.status == ExecutionStatus.Failed,
            "Cannot retry"
        );
        
        // Get message
        Message memory message = messageRouter.getMessage(messageId);
        
        // Retry execution
        (bool success, bytes memory result) = message.recipient.call(
            abi.encodePacked(message.payload, message.sender)
        );
        
        // Update execution
        execution.timestamp = block.timestamp;
        execution.result = result;
        execution.status = success ? ExecutionStatus.Success : ExecutionStatus.Failed;
        
        emit ExecutionRetried(messageId, success);
    }
}
```

## Message Types

### 1. Contract Call Message
```solidity
contract ContractCallMessage {
    struct CallData {
        address target;
        bytes data;
        uint256 value;
        uint256 gasLimit;
    }
    
    function encodeCallMessage(
        CallData memory call
    ) public pure returns (bytes memory) {
        return abi.encode(
            MessageType.ContractCall,
            call
        );
    }
    
    function executeCallMessage(
        bytes memory payload,
        address sender
    ) external returns (bool, bytes memory) {
        // Decode call data
        CallData memory call = abi.decode(payload, (CallData));
        
        // Execute call
        (bool success, bytes memory result) = call.target.call{
            value: call.value,
            gas: call.gasLimit
        }(abi.encodePacked(call.data, sender));
        
        return (success, result);
    }
}
```

### 2. State Sync Message
```solidity
contract StateSyncMessage {
    struct StateData {
        bytes32 stateRoot;
        bytes proof;
        bytes data;
    }
    
    function encodeStateMessage(
        StateData memory state
    ) public pure returns (bytes memory) {
        return abi.encode(
            MessageType.StateSync,
            state
        );
    }
    
    function executeStateMessage(
        bytes memory payload,
        address sender
    ) external returns (bool) {
        // Decode state data
        StateData memory state = abi.decode(payload, (StateData));
        
        // Verify state proof
        require(
            verifyStateProof(
                state.stateRoot,
                state.proof,
                state.data
            ),
            "Invalid state proof"
        );
        
        // Update state
        updateState(state.data);
        
        return true;
    }
}
```

## Message Relayer

### 1. Relayer Network
```solidity
contract RelayerNetwork {
    struct Relayer {
        address addr;
        uint256 stake;
        uint256 rewards;
        bool active;
    }
    
    mapping(address => Relayer) public relayers;
    
    function registerRelayer() external payable {
        require(msg.value >= minStake, "Insufficient stake");
        
        relayers[msg.sender] = Relayer({
            addr: msg.sender,
            stake: msg.value,
            rewards: 0,
            active: true
        });
        
        emit RelayerRegistered(msg.sender);
    }
    
    function submitMessage(
        bytes32 messageId,
        bytes memory proof
    ) external {
        require(
            relayers[msg.sender].active,
            "Not active relayer"
        );
        
        // Execute message
        messageExecutor.executeMessage(messageId, proof);
        
        // Reward relayer
        uint256 reward = calculateReward(messageId);
        relayers[msg.sender].rewards += reward;
        
        emit MessageRelayed(messageId, msg.sender, reward);
    }
}
```

### 2. Reward Distribution
```solidity
contract RewardDistributor {
    struct RewardConfig {
        uint256 baseReward;
        uint256 speedBonus;
        uint256 complexityMultiplier;
    }
    
    RewardConfig public config;
    
    function calculateReward(
        bytes32 messageId
    ) public view returns (uint256) {
        Message memory message = messageRouter.getMessage(messageId);
        
        // Calculate base reward
        uint256 reward = config.baseReward;
        
        // Add speed bonus
        uint256 delay = block.timestamp - message.timestamp;
        if (delay < speedBonusThreshold) {
            reward += config.speedBonus;
        }
        
        // Add complexity multiplier
        uint256 complexity = calculateComplexity(message);
        reward += complexity * config.complexityMultiplier;
        
        return reward;
    }
    
    function distributeRewards() external {
        for (uint256 i = 0; i < relayers.length; i++) {
            address relayer = relayers[i];
            uint256 reward = getRelayerReward(relayer);
            
            if (reward > 0) {
                token.transfer(relayer, reward);
                clearRelayerReward(relayer);
            }
        }
        
        emit RewardsDistributed();
    }
}
```

## Analytics

### 1. Message Analytics
```typescript
interface MessageAnalytics {
    // Message Stats
    getMessageCount(): Promise<MessageCount>;
    getMessageLatency(): Promise<LatencyStats>;
    
    // Relayer Stats
    getRelayerPerformance(): Promise<RelayerStats>;
    getRewardDistribution(): Promise<RewardStats>;
    
    // Network Stats
    getNetworkMetrics(): Promise<NetworkMetrics>;
    getChainStats(): Promise<ChainStats>;
}
```

### 2. Performance Analytics
```typescript
interface PerformanceAnalytics {
    // Latency Analytics
    getLatencyMetrics(): Promise<LatencyMetrics>;
    getBottlenecks(): Promise<BottleneckData>;
    
    // Reliability Metrics
    getSuccessRate(): Promise<SuccessRate>;
    getFailureAnalysis(): Promise<FailureData>;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface MessagingSDK {
    // Message Operations
    sendMessage(params: MessageParams): Promise<string>;
    executeMessage(messageId: string, proof: string): Promise<string>;
    
    // Relayer Operations
    registerRelayer(stake: string): Promise<string>;
    submitMessage(messageId: string, proof: string): Promise<string>;
    
    // Analytics
    getMessageStats(): Promise<MessageStats>;
    getRelayerStats(): Promise<RelayerStats>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onMessageSent(callback: MessageCallback): Subscription;
    onMessageExecuted(callback: ExecutionCallback): Subscription;
    onRelayerUpdate(callback: RelayerCallback): Subscription;
    
    // Analytics
    getEventStats(): Promise<EventStats>;
    getNetworkStats(): Promise<NetworkStats>;
}
```
