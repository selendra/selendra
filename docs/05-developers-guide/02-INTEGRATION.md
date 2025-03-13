# Integration Guide

## Overview

This guide covers integrating Selendra into your application:
- API Integration
- SDK Usage
- Event Handling
- Error Management
- Performance Optimization

## API Integration

### 1. REST API
```typescript
interface SelendraAPI {
    // Configuration
    const config = {
        baseURL: 'https://api.selendra.org',
        timeout: 30000,
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': process.env.SELENDRA_API_KEY
        }
    };
    
    // HTTP Client
    const client = axios.create(config);
    
    // API Methods
    async function getAccountInfo(address: string) {
        const response = await client.get(`/accounts/${address}`);
        return response.data;
    }
    
    async function submitTransaction(tx: Transaction) {
        const response = await client.post('/transactions', tx);
        return response.data;
    }
    
    async function queryState(path: string) {
        const response = await client.get(`/state/${path}`);
        return response.data;
    }
}
```

### 2. WebSocket API
```typescript
interface SelendraWS {
    // WebSocket Connection
    const ws = new WebSocket('wss://ws.selendra.org');
    
    // Message Handling
    ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        handleMessage(message);
    };
    
    // Subscription
    function subscribe(topic: string) {
        ws.send(JSON.stringify({
            type: 'subscribe',
            topic
        }));
    }
    
    // Request-Response
    async function request(method: string, params: any[]) {
        const id = generateRequestId();
        
        ws.send(JSON.stringify({
            jsonrpc: '2.0',
            id,
            method,
            params
        }));
        
        return new Promise((resolve) => {
            responses[id] = resolve;
        });
    }
}
```

## SDK Usage

### 1. Account Management
```typescript
class AccountManager {
    private sdk: SelendraSDK;
    
    constructor(sdk: SelendraSDK) {
        this.sdk = sdk;
    }
    
    // Create Account
    async createAccount() {
        const mnemonic = await this.sdk.createMnemonic();
        const account = await this.sdk.accountFromMnemonic(mnemonic);
        
        return {
            address: account.address,
            mnemonic
        };
    }
    
    // Import Account
    async importAccount(mnemonic: string) {
        const account = await this.sdk.accountFromMnemonic(mnemonic);
        return account;
    }
    
    // Sign Message
    async signMessage(message: string, account: Account) {
        const signature = await account.sign(message);
        return signature;
    }
}
```

### 2. Transaction Management
```typescript
class TransactionManager {
    private sdk: SelendraSDK;
    
    constructor(sdk: SelendraSDK) {
        this.sdk = sdk;
    }
    
    // Send Transaction
    async sendTransaction(tx: Transaction) {
        // Prepare transaction
        const prepared = await this.sdk.tx.prepare(tx);
        
        // Sign transaction
        const signed = await this.sdk.tx.sign(prepared);
        
        // Submit transaction
        const result = await this.sdk.tx.submit(signed);
        
        // Wait for confirmation
        await this.sdk.tx.waitForConfirmation(result.hash);
        
        return result;
    }
    
    // Batch Transactions
    async sendBatch(txs: Transaction[]) {
        const batch = await this.sdk.tx.batch(txs);
        return this.sendTransaction(batch);
    }
}
```

### 3. Contract Integration
```typescript
class ContractManager {
    private sdk: SelendraSDK;
    
    constructor(sdk: SelendraSDK) {
        this.sdk = sdk;
    }
    
    // Deploy Contract
    async deployContract(abi: any, bytecode: string, args: any[]) {
        const contract = await this.sdk.contract.deploy({
            abi,
            bytecode,
            args
        });
        
        return contract;
    }
    
    // Load Contract
    async loadContract(address: string, abi: any) {
        const contract = await this.sdk.contract.load({
            address,
            abi
        });
        
        return contract;
    }
    
    // Call Contract
    async callContract(
        contract: Contract,
        method: string,
        args: any[]
    ) {
        const result = await contract.call(method, args);
        return result;
    }
}
```

## Event Handling

### 1. Event Subscription
```typescript
class EventManager {
    private sdk: SelendraSDK;
    private subscriptions: Map<string, Subscription>;
    
    constructor(sdk: SelendraSDK) {
        this.sdk = sdk;
        this.subscriptions = new Map();
    }
    
    // Subscribe to Events
    async subscribe(eventName: string, callback: Function) {
        const subscription = await this.sdk.events.subscribe(
            eventName,
            (event) => {
                try {
                    callback(event);
                } catch (error) {
                    console.error('Event handler error:', error);
                }
            }
        );
        
        this.subscriptions.set(eventName, subscription);
        return subscription;
    }
    
    // Unsubscribe from Events
    async unsubscribe(eventName: string) {
        const subscription = this.subscriptions.get(eventName);
        if (subscription) {
            await subscription.unsubscribe();
            this.subscriptions.delete(eventName);
        }
    }
}
```

### 2. Event Processing
```typescript
class EventProcessor {
    // Process Event
    async processEvent(event: Event) {
        switch (event.type) {
            case 'Transfer':
                await this.handleTransfer(event);
                break;
            case 'Contract':
                await this.handleContract(event);
                break;
            default:
                console.log('Unknown event:', event);
        }
    }
    
    // Handle Transfer Event
    private async handleTransfer(event: TransferEvent) {
        const {
            from,
            to,
            amount
        } = event.data;
        
        // Update balances
        await this.updateBalance(from, -amount);
        await this.updateBalance(to, amount);
        
        // Notify users
        await this.notifyUsers(from, to, amount);
    }
    
    // Handle Contract Event
    private async handleContract(event: ContractEvent) {
        const {
            contract,
            method,
            args
        } = event.data;
        
        // Update contract state
        await this.updateContractState(contract, method, args);
        
        // Log event
        await this.logContractEvent(event);
    }
}
```

## Error Management

### 1. Error Handling
```typescript
class ErrorHandler {
    // Handle API Errors
    handleApiError(error: any) {
        if (error.response) {
            // HTTP Error
            return this.handleHttpError(error.response);
        } else if (error.request) {
            // Network Error
            return this.handleNetworkError(error.request);
        } else {
            // Client Error
            return this.handleClientError(error);
        }
    }
    
    // Handle Transaction Errors
    handleTransactionError(error: any) {
        switch (error.code) {
            case 'INSUFFICIENT_FUNDS':
                return new Error('Insufficient funds');
            case 'NONCE_TOO_LOW':
                return new Error('Nonce too low');
            case 'GAS_TOO_LOW':
                return new Error('Gas too low');
            default:
                return error;
        }
    }
    
    // Handle Contract Errors
    handleContractError(error: any) {
        if (error.message.includes('revert')) {
            return new Error('Contract reverted');
        } else if (error.message.includes('gas')) {
            return new Error('Out of gas');
        } else {
            return error;
        }
    }
}
```

### 2. Error Recovery
```typescript
class ErrorRecovery {
    // Retry Operation
    async retry<T>(
        operation: () => Promise<T>,
        maxAttempts: number = 3,
        delay: number = 1000
    ): Promise<T> {
        let lastError: Error;
        
        for (let attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                return await operation();
            } catch (error) {
                lastError = error;
                
                if (attempt < maxAttempts) {
                    await this.delay(delay * attempt);
                }
            }
        }
        
        throw lastError;
    }
    
    // Handle Failed Transaction
    async recoverTransaction(tx: Transaction) {
        // Check transaction status
        const status = await this.checkTransactionStatus(tx.hash);
        
        if (status === 'failed') {
            // Prepare new transaction
            const newTx = await this.prepareRecoveryTransaction(tx);
            
            // Submit new transaction
            return this.submitTransaction(newTx);
        }
        
        return tx;
    }
}
```

## Performance Optimization

### 1. Caching
```typescript
class CacheManager {
    private cache: Map<string, CacheEntry>;
    
    constructor() {
        this.cache = new Map();
    }
    
    // Get Cached Data
    async get<T>(
        key: string,
        fetch: () => Promise<T>,
        ttl: number = 60000
    ): Promise<T> {
        const cached = this.cache.get(key);
        
        if (cached && !this.isExpired(cached)) {
            return cached.data as T;
        }
        
        const data = await fetch();
        
        this.cache.set(key, {
            data,
            timestamp: Date.now(),
            ttl
        });
        
        return data;
    }
    
    // Clear Cache
    clear(pattern?: string) {
        if (pattern) {
            const regex = new RegExp(pattern);
            for (const key of this.cache.keys()) {
                if (regex.test(key)) {
                    this.cache.delete(key);
                }
            }
        } else {
            this.cache.clear();
        }
    }
}
```

### 2. Batch Processing
```typescript
class BatchProcessor {
    private queue: any[];
    private batchSize: number;
    private timeout: number;
    private timer: NodeJS.Timeout | null;
    
    constructor(
        batchSize: number = 100,
        timeout: number = 1000
    ) {
        this.queue = [];
        this.batchSize = batchSize;
        this.timeout = timeout;
        this.timer = null;
    }
    
    // Add Item to Batch
    async add(item: any) {
        this.queue.push(item);
        
        if (this.queue.length >= this.batchSize) {
            await this.process();
        } else if (!this.timer) {
            this.timer = setTimeout(
                () => this.process(),
                this.timeout
            );
        }
    }
    
    // Process Batch
    private async process() {
        if (this.timer) {
            clearTimeout(this.timer);
            this.timer = null;
        }
        
        if (this.queue.length === 0) return;
        
        const batch = this.queue.splice(0, this.batchSize);
        await this.processBatch(batch);
    }
}
```

### 3. Connection Management
```typescript
class ConnectionManager {
    private connections: Map<string, Connection>;
    private maxConnections: number;
    
    constructor(maxConnections: number = 10) {
        this.connections = new Map();
        this.maxConnections = maxConnections;
    }
    
    // Get Connection
    async getConnection(endpoint: string): Promise<Connection> {
        let connection = this.connections.get(endpoint);
        
        if (!connection || !connection.isAlive()) {
            connection = await this.createConnection(endpoint);
            this.connections.set(endpoint, connection);
        }
        
        return connection;
    }
    
    // Monitor Connections
    async monitor() {
        setInterval(() => {
            for (const [endpoint, connection] of this.connections) {
                if (!connection.isAlive()) {
                    this.connections.delete(endpoint);
                }
            }
        }, 60000);
    }
}
```
