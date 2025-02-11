# Selendra Network Integration Guide

## Network Information

### Chain Specifications
```rust
const SELENDRA_PROPERTIES = {
    ss58Format: 204,
    tokenDecimals: 18,
    tokenSymbol: 'SEL',
    chainId: 1961,    // Mainnet
    testnetChainId: 1953  // Testnet
};
```

### Network Endpoints
```javascript
const ENDPOINTS = {
    mainnet: {
        rpc: 'https://rpc.selendra.org',
        ws: 'wss://ws.selendra.org',
        explorer: 'https://explorer.selendra.org'
    },
    testnet: {
        rpc: 'https://testnet-rpc.selendra.org',
        ws: 'wss://testnet-ws.selendra.org',
        explorer: 'https://testnet.explorer.selendra.org'
    }
};
```

## Node Integration

### Running a Full Node
```bash
# Clone and build
git clone https://github.com/selendra/selendra
cd selendra
cargo build --release

# Run mainnet node
./target/release/selendra \
    --chain mainnet \
    --name "my-node" \
    --pruning archive \
    --rpc-cors all \
    --rpc-methods unsafe \
    --ws-external \
    --rpc-external

# Run testnet node
./target/release/selendra \
    --chain testnet \
    --name "my-testnet-node" \
    --pruning archive \
    --rpc-cors all
```

### Docker Deployment
```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /selendra
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder /selendra/target/release/selendra /usr/local/bin
EXPOSE 30333 9933 9944
ENTRYPOINT ["selendra"]
```

```yaml
# docker-compose.yml
version: '3'
services:
  selendra-node:
    build: .
    ports:
      - "30333:30333"
      - "9933:9933"
      - "9944:9944"
    volumes:
      - ./data:/data
    command: 
      - --chain=mainnet
      - --name=docker-node
      - --pruning=archive
      - --rpc-cors=all
```

## API Integration

### Polkadot.js Integration
```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function connectToSelendra() {
    const provider = new WsProvider('wss://ws.selendra.org');
    const api = await ApiPromise.create({ provider });
    
    // Subscribe to new blocks
    api.rpc.chain.subscribeNewHeads((header) => {
        console.log(`New block: ${header.number}`);
    });
    
    // Get chain state
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    
    console.log(`Connected to ${chain} using ${nodeName} v${nodeVersion}`);
    return api;
}
```

### Web3.js Integration
```javascript
const Web3 = require('web3');

async function connectWeb3() {
    const web3 = new Web3('https://rpc.selendra.org');
    
    // Check connection
    const networkId = await web3.eth.net.getId();
    console.log(`Connected to network ID: ${networkId}`);
    
    // Get latest block
    const block = await web3.eth.getBlock('latest');
    console.log(`Latest block: ${block.number}`);
    
    return web3;
}
```

## Transaction Handling

### Native Transactions
```javascript
async function sendTransaction(api, from, to, amount) {
    // Create transaction
    const transfer = api.tx.balances.transfer(to, amount);
    
    // Sign and send
    const hash = await transfer.signAndSend(from);
    console.log(`Transaction hash: ${hash.toHex()}`);
    
    // Wait for confirmation
    return new Promise((resolve) => {
        api.rpc.chain.subscribeNewHeads((header) => {
            api.rpc.chain.getBlock(header.hash).then((block) => {
                const txHash = block.block.extrinsics
                    .find(ex => ex.hash.toHex() === hash.toHex());
                if (txHash) {
                    resolve(header.number.toNumber());
                }
            });
        });
    });
}
```

### EVM Transactions
```javascript
async function sendEVMTransaction(web3, from, to, amount) {
    const tx = {
        from,
        to,
        value: web3.utils.toWei(amount, 'ether'),
        gas: 21000,
        gasPrice: await web3.eth.getGasPrice()
    };
    
    const receipt = await web3.eth.sendTransaction(tx);
    return receipt;
}
```

## Smart Contract Integration

### Contract Deployment
```javascript
async function deployContract(web3, abi, bytecode, from) {
    const contract = new web3.eth.Contract(abi);
    const deploy = contract.deploy({
        data: bytecode,
        arguments: [] // Constructor arguments
    });
    
    const gas = await deploy.estimateGas();
    const instance = await deploy.send({
        from,
        gas: Math.floor(gas * 1.1) // Add 10% buffer
    });
    
    console.log(`Contract deployed at: ${instance.options.address}`);
    return instance;
}
```

### Contract Interaction
```javascript
async function interactWithContract(web3, contractAddress, abi) {
    const contract = new web3.eth.Contract(abi, contractAddress);
    
    // Read contract state
    const result = await contract.methods.someMethod().call();
    
    // Write to contract
    const tx = await contract.methods.someMethod(params)
        .send({ from: account, gas: 100000 });
    
    return tx;
}
```

## Event Monitoring

### Subscribe to Events
```javascript
async function monitorEvents(api) {
    // Subscribe to system events
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event, phase } = record;
            
            // Handle different event types
            if (api.events.balances.Transfer.is(event)) {
                const [from, to, amount] = event.data;
                console.log(`Transfer: ${from} -> ${to}: ${amount}`);
            }
        });
    });
}
```

### Monitor EVM Events
```javascript
async function monitorEVMEvents(web3, contract) {
    contract.events.allEvents({
        fromBlock: 'latest'
    }, (error, event) => {
        if (error) {
            console.error('Event error:', error);
            return;
        }
        console.log('Event:', event);
    });
}
```

## Error Handling

### Common Errors
```javascript
class SelendraError extends Error {
    constructor(message, code) {
        super(message);
        this.code = code;
    }
}

function handleError(error) {
    if (error.toString().includes('1010')) {
        throw new SelendraError('Invalid Transaction', 1010);
    }
    if (error.toString().includes('1011')) {
        throw new SelendraError('Insufficient Balance', 1011);
    }
    throw error;
}
```

## Best Practices

### Performance Optimization
```javascript
// Batch transactions
async function batchTransactions(api, txs) {
    const batch = api.tx.utility.batch(txs);
    return batch.signAndSend(account);
}

// Connection management
let api = null;
async function getAPI() {
    if (!api) {
        api = await connectToSelendra();
    }
    return api;
}
```

### Security Considerations
```javascript
// Secure key management
const KEY_STORE = {
    encrypt: (key) => {
        // Implement secure encryption
    },
    decrypt: (encryptedKey) => {
        // Implement secure decryption
    }
};

// Rate limiting
class RateLimit {
    constructor(maxRequests, timeWindow) {
        this.requests = [];
        this.maxRequests = maxRequests;
        this.timeWindow = timeWindow;
    }
    
    canMakeRequest() {
        const now = Date.now();
        this.requests = this.requests.filter(time => 
            now - time < this.timeWindow);
        if (this.requests.length < this.maxRequests) {
            this.requests.push(now);
            return true;
        }
        return false;
    }
}
```
