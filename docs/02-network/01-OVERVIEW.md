# Selendra Network Overview

## Network Configuration

### Chain Specifications
```rust
// Runtime configuration
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("selendra"),
    impl_name: create_runtime_str!("selendra-node"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

// Network identifiers
pub const SS58_PREFIX: u8 = 204;
pub const MILLISECS_PER_BLOCK: u64 = 1000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Chain IDs
pub const MAINNET_CHAIN_ID: u64 = 1961;
pub const TESTNET_CHAIN_ID: u64 = 1953;
```

## Network Interaction

### Using Polkadot.js API
```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function connectToSelendra() {
    // Connect to Selendra node
    const provider = new WsProvider('wss://rpc.selendra.org');
    const api = await ApiPromise.create({ provider });

    // Get chain information
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    console.log(`Connected to ${chain} using ${nodeName} v${nodeVersion}`);
    return api;
}

// Query account balance
async function getBalance(api, address) {
    const { data: balance } = await api.query.system.account(address);
    return balance.free.toHuman();
}

// Submit transaction
async function transfer(api, from, to, amount) {
    const transfer = api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(from);
    return hash.toHex();
}
```

### Using Web3.js
```javascript
const Web3 = require('web3');

// Connect to Selendra EVM
const web3 = new Web3('https://rpc.selendra.org');

// Check connection
async function checkConnection() {
    const networkId = await web3.eth.net.getId();
    if (networkId === 1961) {
        console.log('Connected to Selendra Mainnet');
    } else if (networkId === 1953) {
        console.log('Connected to Selendra Testnet');
    }
}

// Send EVM transaction
async function sendTransaction(from, to, value) {
    const tx = {
        from,
        to,
        value: web3.utils.toWei(value, 'ether'),
        gas: 21000,
        gasPrice: await web3.eth.getGasPrice()
    };
    return web3.eth.sendTransaction(tx);
}
```

## Network Features

### Cross-chain Bridge Integration
```solidity
// Bridge contract example
contract SelendraBridge {
    mapping(bytes32 => bool) public processedMessages;
    mapping(address => uint256) public balances;
    
    event MessageProcessed(bytes32 messageId);
    event TokensLocked(address user, uint256 amount);
    
    function lockTokens(uint256 amount, string memory destinationChain) 
        external 
    {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        
        bytes32 messageId = keccak256(
            abi.encodePacked(
                msg.sender,
                amount,
                destinationChain,
                block.timestamp
            )
        );
        
        emit TokensLocked(msg.sender, amount);
        // Bridge logic here
    }
    
    function processMessage(bytes32 messageId, address recipient, uint256 amount) 
        external 
    {
        require(!processedMessages[messageId], "Message already processed");
        processedMessages[messageId] = true;
        balances[recipient] += amount;
        emit MessageProcessed(messageId);
    }
}
```

### Smart Contract Interaction
```typescript
// Contract interaction example
async function interactWithContract() {
    const contract = new web3.eth.Contract(ABI, CONTRACT_ADDRESS);
    
    // Read contract state
    const balance = await contract.methods.balanceOf(address).call();
    
    // Write to contract
    const tx = await contract.methods.transfer(recipient, amount)
        .send({ from: sender, gas: 100000 });
        
    return tx.transactionHash;
}
```

## Network Monitoring

### Block Production Monitoring
```python
from substrateinterface import SubstrateInterface
import time

def monitor_blocks():
    substrate = SubstrateInterface(
        url="wss://rpc.selendra.org"
    )
    
    def callback(obj):
        block_number = obj['header']['number']
        block_hash = obj['header']['hash']
        parent_hash = obj['header']['parentHash']
        print(f"New block: {block_number}")
        print(f"Hash: {block_hash}")
        print(f"Parent: {parent_hash}")
    
    substrate.subscribe_block_headers(callback)

if __name__ == "__main__":
    monitor_blocks()
```

### Network Health Check
```python
import requests
import json

def check_network_health():
    endpoints = {
        'mainnet': 'https://rpc.selendra.org',
        'testnet': 'https://testnet-rpc.selendra.org'
    }
    
    for network, endpoint in endpoints.items():
        try:
            response = requests.post(
                endpoint,
                json={
                    "jsonrpc": "2.0",
                    "method": "system_health",
                    "params": [],
                    "id": 1
                }
            )
            
            health = response.json()['result']
            print(f"{network} Health:")
            print(f"Syncing: {health['isSyncing']}")
            print(f"Peers: {health['peers']}")
            print(f"Should have peers: {health['shouldHavePeers']}")
            
        except Exception as e:
            print(f"Error checking {network}: {str(e)}")

if __name__ == "__main__":
    check_network_health()
```

## Network Utilities

### Transaction Fee Estimation
```javascript
async function estimateTransactionFee(api, tx) {
    const info = await tx.paymentInfo(sender);
    return {
        partialFee: info.partialFee.toHuman(),
        weight: info.weight.toHuman()
    };
}

// Usage example
const tx = api.tx.balances.transfer(recipient, amount);
const fee = await estimateTransactionFee(api, tx);
console.log(`Estimated fee: ${fee.partialFee}`);
```

### Block Time Analysis
```python
def analyze_block_times():
    substrate = SubstrateInterface(
        url="wss://rpc.selendra.org"
    )
    
    block_times = []
    last_timestamp = None
    
    for block_number in range(100):
        block = substrate.get_block(block_number)
        timestamp = block['header']['timestamp']
        
        if last_timestamp:
            block_time = timestamp - last_timestamp
            block_times.append(block_time)
            
        last_timestamp = timestamp
    
    avg_block_time = sum(block_times) / len(block_times)
    print(f"Average block time: {avg_block_time}ms")
```
