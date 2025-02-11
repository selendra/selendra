# Selendra Architecture

## Overview

Selendra's architecture consists of:
- Core Runtime
- Network Layer
- Consensus Layer
- Storage Layer
- API Layer
- Smart Contract Layer

## Core Runtime

### 1. Runtime Architecture
```rust
pub struct Runtime {
    /// Runtime version
    version: RuntimeVersion,
    /// Runtime modules
    modules: Vec<Module>,
    /// Runtime storage
    storage: Storage,
    /// Runtime executor
    executor: Executor,
}

impl Runtime {
    /// Execute runtime call
    pub fn execute_call(
        &mut self,
        call: RuntimeCall,
    ) -> Result<RuntimeOutput, Error> {
        // Validate call
        self.validate_call(&call)?;
        
        // Execute call
        let output = self.executor.execute(call)?;
        
        // Update state
        self.update_state(output.state_changes)?;
        
        Ok(output)
    }
}
```

### 2. Module System
```rust
pub struct Module {
    /// Module name
    name: String,
    /// Module storage
    storage: ModuleStorage,
    /// Module calls
    calls: Vec<ModuleCall>,
    /// Module events
    events: Vec<ModuleEvent>,
}

impl Module {
    /// Execute module call
    pub fn execute_call(
        &mut self,
        call: ModuleCall,
    ) -> Result<ModuleOutput, Error> {
        // Check call permissions
        self.check_permissions(&call)?;
        
        // Execute call logic
        let output = match call {
            ModuleCall::Transfer(params) => self.transfer(params),
            ModuleCall::Stake(params) => self.stake(params),
            // ... other calls
        }?;
        
        // Emit events
        self.emit_events(output.events);
        
        Ok(output)
    }
}
```

## Network Layer

### 1. P2P Network
```rust
pub struct P2PNetwork {
    /// Network config
    config: NetworkConfig,
    /// Peer connections
    peers: PeerSet,
    /// Message handler
    handler: MessageHandler,
}

impl P2PNetwork {
    /// Handle incoming message
    pub fn handle_message(
        &mut self,
        peer_id: PeerId,
        message: NetworkMessage,
    ) -> Result<(), Error> {
        // Validate message
        self.validate_message(&message)?;
        
        // Process message
        match message {
            NetworkMessage::Block(block) => {
                self.handler.handle_block(block)
            }
            NetworkMessage::Transaction(tx) => {
                self.handler.handle_transaction(tx)
            }
            // ... other messages
        }
    }
}
```

### 2. Message Propagation
```rust
pub struct MessagePropagation {
    /// Message queue
    queue: MessageQueue,
    /// Peer manager
    peer_manager: PeerManager,
    /// Network metrics
    metrics: NetworkMetrics,
}

impl MessagePropagation {
    /// Propagate message to peers
    pub fn propagate_message(
        &mut self,
        message: NetworkMessage,
    ) -> Result<(), Error> {
        // Get target peers
        let targets = self.peer_manager.get_target_peers(&message);
        
        // Send message
        for peer in targets {
            self.send_to_peer(peer, message.clone())?;
        }
        
        // Update metrics
        self.metrics.record_propagation(&message);
        
        Ok(())
    }
}
```

## Consensus Layer

### 1. Block Production
```rust
pub struct BlockProduction {
    /// Block builder
    builder: BlockBuilder,
    /// Transaction pool
    tx_pool: TransactionPool,
    /// State manager
    state: StateManager,
}

impl BlockProduction {
    /// Produce new block
    pub fn produce_block(
        &mut self,
        parent: BlockHash,
    ) -> Result<Block, Error> {
        // Create block
        let mut block = self.builder.create_block(parent)?;
        
        // Add transactions
        let transactions = self.tx_pool.get_ready_transactions();
        for tx in transactions {
            if block.can_add_transaction(&tx) {
                block.add_transaction(tx);
            }
        }
        
        // Finalize block
        self.builder.finalize_block(&mut block)?;
        
        Ok(block)
    }
}
```

### 2. Block Validation
```rust
pub struct BlockValidation {
    /// Validation rules
    rules: ValidationRules,
    /// State validator
    state_validator: StateValidator,
    /// Transaction validator
    tx_validator: TransactionValidator,
}

impl BlockValidation {
    /// Validate block
    pub fn validate_block(
        &self,
        block: &Block,
    ) -> Result<(), Error> {
        // Check basic rules
        self.rules.check_block(block)?;
        
        // Validate state transitions
        self.state_validator.validate_state_transitions(block)?;
        
        // Validate transactions
        for tx in &block.transactions {
            self.tx_validator.validate_transaction(tx)?;
        }
        
        Ok(())
    }
}
```

## Storage Layer

### 1. State Storage
```rust
pub struct StateStorage {
    /// Storage backend
    backend: StorageBackend,
    /// State cache
    cache: StateCache,
    /// Pruning config
    pruning: PruningConfig,
}

impl StateStorage {
    /// Get storage value
    pub fn get_storage(
        &self,
        key: &StorageKey,
    ) -> Result<Option<StorageValue>, Error> {
        // Check cache
        if let Some(value) = self.cache.get(key) {
            return Ok(Some(value));
        }
        
        // Get from backend
        let value = self.backend.get(key)?;
        
        // Update cache
        if let Some(value) = value.clone() {
            self.cache.insert(key.clone(), value);
        }
        
        Ok(value)
    }
}
```

### 2. Database Management
```rust
pub struct DatabaseManager {
    /// Database instance
    db: Database,
    /// Column families
    columns: ColumnFamilies,
    /// Compaction config
    compaction: CompactionConfig,
}

impl DatabaseManager {
    /// Write batch to database
    pub fn write_batch(
        &mut self,
        batch: WriteBatch,
    ) -> Result<(), Error> {
        // Prepare batch
        let mut db_batch = self.db.batch();
        
        // Add operations
        for op in batch.operations {
            match op {
                Operation::Insert { key, value } => {
                    db_batch.put(key, value)?;
                }
                Operation::Delete { key } => {
                    db_batch.delete(key)?;
                }
            }
        }
        
        // Write batch
        self.db.write(db_batch)?;
        
        Ok(())
    }
}
```

## API Layer

### 1. RPC Interface
```rust
pub struct RPCServer {
    /// API handlers
    handlers: RPCHandlers,
    /// Request middleware
    middleware: RPCMiddleware,
    /// Response formatter
    formatter: ResponseFormatter,
}

impl RPCServer {
    /// Handle RPC request
    pub async fn handle_request(
        &self,
        request: RPCRequest,
    ) -> Result<RPCResponse, Error> {
        // Apply middleware
        let request = self.middleware.process_request(request)?;
        
        // Get handler
        let handler = self.handlers.get_handler(&request.method)?;
        
        // Execute handler
        let result = handler.handle(request.params).await?;
        
        // Format response
        let response = self.formatter.format_response(result)?;
        
        Ok(response)
    }
}
```

### 2. WebSocket Server
```rust
pub struct WebSocketServer {
    /// Connection manager
    connections: ConnectionManager,
    /// Subscription manager
    subscriptions: SubscriptionManager,
    /// Message handler
    handler: WSMessageHandler,
}

impl WebSocketServer {
    /// Handle WebSocket message
    pub async fn handle_message(
        &mut self,
        connection_id: ConnectionId,
        message: WSMessage,
    ) -> Result<(), Error> {
        match message {
            WSMessage::Subscribe(params) => {
                self.handle_subscribe(connection_id, params).await
            }
            WSMessage::Unsubscribe(params) => {
                self.handle_unsubscribe(connection_id, params).await
            }
            WSMessage::Request(request) => {
                self.handler.handle_request(request).await
            }
        }
    }
}
```

## Smart Contract Layer

### 1. Contract Execution
```rust
pub struct ContractExecutor {
    /// VM instance
    vm: VirtualMachine,
    /// Gas meter
    gas_meter: GasMeter,
    /// Storage access
    storage: ContractStorage,
}

impl ContractExecutor {
    /// Execute contract
    pub fn execute_contract(
        &mut self,
        contract: Contract,
        input: Vec<u8>,
    ) -> Result<ContractResult, Error> {
        // Prepare execution
        let context = self.prepare_context(&contract)?;
        
        // Execute code
        let result = self.vm.execute(
            contract.code.clone(),
            input,
            context,
            self.gas_meter,
        )?;
        
        // Process result
        self.process_result(result)
    }
}
```

### 2. Contract Storage
```rust
pub struct ContractStorage {
    /// Storage backend
    backend: StorageBackend,
    /// Cache layer
    cache: StorageCache,
    /// Access control
    access_control: AccessControl,
}

impl ContractStorage {
    /// Get contract storage
    pub fn get_storage(
        &self,
        contract_id: ContractId,
        key: StorageKey,
    ) -> Result<Option<StorageValue>, Error> {
        // Check access
        self.access_control.check_access(contract_id, &key)?;
        
        // Get from cache
        if let Some(value) = self.cache.get(&key) {
            return Ok(Some(value));
        }
        
        // Get from backend
        let value = self.backend.get(&key)?;
        
        // Update cache
        if let Some(value) = value.clone() {
            self.cache.insert(key, value);
        }
        
        Ok(value)
    }
}
```
