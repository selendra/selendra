# API Reference

## Overview

Selendra provides multiple API interfaces:
- REST API for standard HTTP requests
- WebSocket API for real-time data
- RPC API for direct node interaction
- GraphQL API for flexible queries

## REST API

### Base URL
```
https://api.selendra.org/v1
```

### Authentication
```http
Authorization: Bearer <API_KEY>
```

### Rate Limits
- Free tier: 10 requests/second
- Pro tier: 100 requests/second
- Enterprise tier: Custom limits

### Endpoints

#### Account API

##### Get Account Information
```http
GET /accounts/{address}

Parameters:
- address: Account address (required)

Response:
{
    "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "balance": {
        "free": "1000000000000",
        "reserved": "0",
        "miscFrozen": "0",
        "feeFrozen": "0"
    },
    "nonce": 5,
    "timestamp": "2025-02-11T10:56:16.000Z"
}
```

##### Get Account Transactions
```http
GET /accounts/{address}/transactions

Parameters:
- address: Account address (required)
- page: Page number (optional, default: 1)
- limit: Items per page (optional, default: 10)
- from: Start timestamp (optional)
- to: End timestamp (optional)

Response:
{
    "items": [
        {
            "hash": "0x1234...",
            "from": "5GrwvaEF...",
            "to": "5FHneW46...",
            "amount": "1000000000",
            "fee": "10000",
            "block": 12345,
            "timestamp": "2025-02-11T10:56:16.000Z",
            "status": "success"
        }
    ],
    "total": 100,
    "page": 1,
    "limit": 10
}
```

#### Block API

##### Get Block Information
```http
GET /blocks/{blockId}

Parameters:
- blockId: Block number or hash (required)

Response:
{
    "number": 12345,
    "hash": "0x1234...",
    "parentHash": "0xabcd...",
    "stateRoot": "0xefgh...",
    "extrinsicsRoot": "0xijkl...",
    "timestamp": "2025-02-11T10:56:16.000Z",
    "author": "5GrwvaEF...",
    "transactions": [
        {
            "hash": "0x1234...",
            "from": "5GrwvaEF...",
            "to": "5FHneW46...",
            "amount": "1000000000"
        }
    ]
}
```

##### Get Block Transactions
```http
GET /blocks/{blockId}/transactions

Parameters:
- blockId: Block number or hash (required)
- page: Page number (optional, default: 1)
- limit: Items per page (optional, default: 10)

Response:
{
    "items": [
        {
            "hash": "0x1234...",
            "from": "5GrwvaEF...",
            "to": "5FHneW46...",
            "amount": "1000000000",
            "fee": "10000",
            "status": "success"
        }
    ],
    "total": 50,
    "page": 1,
    "limit": 10
}
```

#### Transaction API

##### Submit Transaction
```http
POST /transactions

Request:
{
    "from": "5GrwvaEF...",
    "to": "5FHneW46...",
    "amount": "1000000000",
    "signature": "0x1234..."
}

Response:
{
    "hash": "0x1234...",
    "status": "pending",
    "timestamp": "2025-02-11T10:56:16.000Z"
}
```

##### Get Transaction Status
```http
GET /transactions/{hash}

Parameters:
- hash: Transaction hash (required)

Response:
{
    "hash": "0x1234...",
    "from": "5GrwvaEF...",
    "to": "5FHneW46...",
    "amount": "1000000000",
    "fee": "10000",
    "block": 12345,
    "timestamp": "2025-02-11T10:56:16.000Z",
    "status": "success",
    "confirmations": 10
}
```

#### Contract API

##### Deploy Contract
```http
POST /contracts

Request:
{
    "bytecode": "0x1234...",
    "abi": [...],
    "constructorArgs": [...],
    "value": "0",
    "gasLimit": 1000000
}

Response:
{
    "address": "5GrwvaEF...",
    "transactionHash": "0x1234...",
    "timestamp": "2025-02-11T10:56:16.000Z"
}
```

##### Call Contract
```http
POST /contracts/{address}/call

Parameters:
- address: Contract address (required)

Request:
{
    "method": "transfer",
    "args": ["5FHneW46...", "1000000000"],
    "value": "0",
    "gasLimit": 1000000
}

Response:
{
    "transactionHash": "0x1234...",
    "result": "0x5678...",
    "timestamp": "2025-02-11T10:56:16.000Z"
}
```

## WebSocket API

### Connection
```javascript
const ws = new WebSocket('wss://ws.selendra.org/v1');

ws.onopen = () => {
    console.log('Connected to Selendra WebSocket');
};

ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    console.log('Received:', message);
};
```

### Subscriptions

#### Subscribe to New Blocks
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "subscribe_newHeads",
    "params": []
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x1234..." // Subscription ID
}

// Subscription Message
{
    "jsonrpc": "2.0",
    "method": "subscribe_newHeads",
    "params": {
        "subscription": "0x1234...",
        "result": {
            "number": 12345,
            "hash": "0x1234...",
            "parentHash": "0xabcd...",
            "timestamp": "2025-02-11T10:56:16.000Z"
        }
    }
}
```

#### Subscribe to Account Changes
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "subscribe_account",
    "params": ["5GrwvaEF..."]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x5678..." // Subscription ID
}

// Subscription Message
{
    "jsonrpc": "2.0",
    "method": "subscribe_account",
    "params": {
        "subscription": "0x5678...",
        "result": {
            "address": "5GrwvaEF...",
            "balance": "1000000000000",
            "nonce": 5
        }
    }
}
```

## RPC API

### JSON-RPC Endpoint
```
https://rpc.selendra.org
```

### Methods

#### Chain Methods

##### chain_getBlock
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "chain_getBlock",
    "params": ["0x1234..."]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "block": {
            "header": {
                "number": "0x1234",
                "parentHash": "0xabcd...",
                "stateRoot": "0xefgh..."
            },
            "extrinsics": [
                "0x1234..."
            ]
        }
    }
}
```

##### chain_getBlockHash
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "chain_getBlockHash",
    "params": [12345]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x1234..."
}
```

#### State Methods

##### state_getStorage
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "state_getStorage",
    "params": ["0x1234..."]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x5678..."
}
```

##### state_queryStorage
```javascript
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "state_queryStorage",
    "params": [
        ["0x1234..."],
        "0xabcd...",
        "0xefgh..."
    ]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": [
        {
            "block": "0x1234...",
            "changes": [
                ["0x1234...", "0x5678..."]
            ]
        }
    ]
}
```

## GraphQL API

### Endpoint
```
https://graphql.selendra.org/v1
```

### Queries

#### Get Account
```graphql
query GetAccount($address: String!) {
    account(address: $address) {
        address
        balance {
            free
            reserved
            miscFrozen
            feeFrozen
        }
        nonce
        transactions(limit: 10) {
            hash
            from
            to
            amount
            timestamp
            status
        }
    }
}
```

#### Get Block
```graphql
query GetBlock($number: Int!) {
    block(number: $number) {
        number
        hash
        parentHash
        stateRoot
        extrinsicsRoot
        timestamp
        author
        transactions {
            hash
            from
            to
            amount
            status
        }
    }
}
```

#### Get Transactions
```graphql
query GetTransactions($address: String!, $limit: Int!, $offset: Int!) {
    transactions(
        address: $address,
        limit: $limit,
        offset: $offset
    ) {
        items {
            hash
            from
            to
            amount
            fee
            block
            timestamp
            status
        }
        total
        offset
        limit
    }
}
```

### Mutations

#### Submit Transaction
```graphql
mutation SubmitTransaction($input: TransactionInput!) {
    submitTransaction(input: $input) {
        hash
        status
        timestamp
    }
}
```

### Subscriptions

#### Subscribe to Blocks
```graphql
subscription OnNewBlock {
    newBlock {
        number
        hash
        parentHash
        timestamp
        author
        transactions {
            hash
            from
            to
            amount
        }
    }
}
```

#### Subscribe to Account Changes
```graphql
subscription OnAccountChange($address: String!) {
    accountChange(address: $address) {
        address
        balance {
            free
            reserved
            miscFrozen
            feeFrozen
        }
        nonce
    }
}
```

## Error Codes

### HTTP Status Codes
- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 404: Not Found
- 429: Too Many Requests
- 500: Internal Server Error

### Error Response Format
```json
{
    "error": {
        "code": "ERROR_CODE",
        "message": "Error description",
        "details": {
            "field": "Additional error details"
        }
    }
}
```

### Common Error Codes
```javascript
const ERROR_CODES = {
    // Authentication Errors
    INVALID_API_KEY: "Invalid API key",
    EXPIRED_API_KEY: "API key has expired",
    
    // Rate Limit Errors
    RATE_LIMIT_EXCEEDED: "Rate limit exceeded",
    
    // Validation Errors
    INVALID_ADDRESS: "Invalid account address",
    INVALID_AMOUNT: "Invalid transaction amount",
    INVALID_SIGNATURE: "Invalid transaction signature",
    
    // Transaction Errors
    INSUFFICIENT_BALANCE: "Insufficient balance",
    NONCE_TOO_LOW: "Nonce too low",
    GAS_TOO_LOW: "Gas limit too low",
    
    // Contract Errors
    CONTRACT_NOT_FOUND: "Contract not found",
    CONTRACT_EXECUTION_FAILED: "Contract execution failed",
    
    // Node Errors
    NODE_OFFLINE: "Node is offline",
    SYNC_ERROR: "Node synchronization error"
}
```
