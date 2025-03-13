# Working with Selendra's RPC API

This guide introduces Selendra's RPC API, explaining how to interact with the blockchain using remote procedure calls. You'll learn how to query the network state, submit transactions, and build applications that interface directly with Selendra nodes.

## Introduction to Selendra's RPC API

Remote Procedure Call (RPC) APIs allow external applications to interact with Selendra nodes by sending requests to exposed endpoints. Selendra provides a comprehensive set of RPC methods to:

- Query blockchain state
- Read account information
- Submit transactions
- Retrieve block and transaction data
- Subscribe to network events
- Interact with runtime modules

Selendra's RPC system is accessible through multiple protocols:
- HTTP for basic requests
- WebSocket for persistent connections and subscriptions
- JSONRPC for standardized formatting

## Getting Started with RPC Endpoints

### Available Endpoints

Selendra provides several network endpoints:

| Network | HTTP Endpoint | WebSocket Endpoint |
|---------|--------------|-------------------|
| Mainnet | https://mainnet.selendra.org | wss://mainnet.selendra.org |
| Testnet | https://testnet.selendra.org | wss://testnet.selendra.org |
| Local Node | http://127.0.0.1:9933 | ws://127.0.0.1:9944 |

### Setting Up a Local Node (Optional)

For development, you can run a local Selendra node:

```bash
# Using Docker
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external --rpc-external

# Or if you've built from source
./target/release/selendra --dev --ws-external --rpc-external
```

## Basic RPC Requests

### Structure of RPC Requests

RPC requests follow the JSON-RPC 2.0 specification:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "method_name",
  "params": ["param1", "param2"]
}
```

### Making Simple HTTP Requests

Using `curl` for HTTP:

```bash
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"chain_getBlockHash","params":[0]}' http://127.0.0.1:9933
```

Example response:

```json
{
  "jsonrpc": "2.0",
  "result": "0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
  "id": 1
}
```

### Using WebSockets for Subscriptions

WebSockets allow for real-time updates through subscriptions:

```javascript
// Using JavaScript and Node.js
const WebSocket = require('websocket').w3cwebsocket;
const client = new WebSocket('ws://127.0.0.1:9944');

client.onopen = () => {
  // Subscribe to new heads (blocks)
  client.send(JSON.stringify({
    jsonrpc: '2.0',
    id: 1,
    method: 'chain_subscribeNewHeads',
    params: []
  }));
};

client.onmessage = (message) => {
  const response = JSON.parse(message.data);
  console.log('New block:', response);
};
```

## Essential RPC Methods

### Chain State Queries

Query the current state of the chain:

```bash
# Get the latest block hash
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"chain_getBlockHash","params":[]}' http://127.0.0.1:9933

# Get finalized head
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"chain_getFinalizedHead","params":[]}' http://127.0.0.1:9933

# Get block details
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"chain_getBlock","params":["0x...blockhash..."]}' http://127.0.0.1:9933
```

### Account Information

Retrieve account details:

```bash
# Get account balance
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"state_getStorage","params":["0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9...(Blake2 hash of System.Account key + account ID)"]}' http://127.0.0.1:9933

# More user-friendly using the state_call method
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"state_call","params":["AccountNonceApi_account_nonce", "0x...accountID..."]}' http://127.0.0.1:9933
```

### Transaction Submission

Submit transactions to the network:

```bash
# Submit a signed transaction (extrinsic)
curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"author_submitExtrinsic","params":["0x...signedTransaction..."]}' http://127.0.0.1:9933
```

## Using Libraries for API Access

Various libraries simplify interacting with Selendra's RPC API:

### JavaScript/TypeScript with Polkadot.js

```javascript
// Install required packages
// npm install @polkadot/api

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function connectToSelendra() {
  // Connect to a Selendra node
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Get chain information
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  
  console.log(`Connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
  
  // Get the latest block header
  const lastHeader = await api.rpc.chain.getHeader();
  console.log(`Last block: #${lastHeader.number}`);
  
  // Subscribe to new blocks
  await api.rpc.chain.subscribeNewHeads((header) => {
    console.log(`Chain is at block: #${header.number}`);
  });
  
  return api;
}

connectToSelendra().catch(console.error);
```

### Python with Substrate Interface

```python
# Install required package
# pip install substrate-interface

from substrateinterface import SubstrateInterface

# Connect to Selendra node
substrate = SubstrateInterface(
    url="ws://127.0.0.1:9944",
    ss58_format=42,  # Selendra's SS58 format
    type_registry_preset='substrate-node-template'
)

# Query chain information
chain_info = substrate.rpc_request(
    method="system_chain",
    params=[]
)
print(f"Connected to: {chain_info['result']}")

# Get the latest block number
block_hash = substrate.get_chain_head()
block = substrate.get_block(block_hash)
print(f"Latest block: #{block['header']['number']}")

# Get account balance
account_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"  # Example address
account_info = substrate.query('System', 'Account', [account_address])
balance = account_info.value['data']['free']
print(f"Account balance: {balance}")
```

### Rust with Substrate API Client

```rust
// Add to Cargo.toml
// [dependencies]
// substrate-api-client = "0.4.0"

use substrate_api_client::{Api, ApiClientError, UncheckedExtrinsicV4, XtStatus};
use sp_core::{sr25519, Pair};

fn main() -> Result<(), ApiClientError> {
    // Connect to node
    let url = "ws://127.0.0.1:9944";
    let api = Api::<sr25519::Pair>::new(url)?;
    
    // Get chain information
    let chain = api.get_chain()?;
    println!("Connected to: {}", chain);
    
    // Get latest block number
    let block_number = api.get_block_number(None)?;
    println!("Latest block: #{}", block_number);
    
    // Create keypair for transaction signing
    let seed = "//Alice";  // Dev account
    let pair = sr25519::Pair::from_string(seed, None).unwrap();
    
    // Create and send a transaction (balance transfer)
    let dest = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";  // Example address
    let amount = 1000000000;  // 1 SEL = 10^12 planck
    
    let xt: UncheckedExtrinsicV4<_> = api.balance_transfer(dest, amount, &pair)?;
    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized)?;
    
    println!("Transaction hash: {:?}", tx_hash);
    
    Ok(())
}
```

## Common API Tasks

### Query Account Balance

JavaScript example:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function getAccountBalance(address) {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Get account info
  const { data: { free, reserved, miscFrozen, feeFrozen } } = await api.query.system.account(address);
  
  console.log(`Account: ${address}`);
  console.log(`Free balance: ${free.toHuman()}`);
  console.log(`Reserved balance: ${reserved.toHuman()}`);
  console.log(`Misc Frozen: ${miscFrozen.toHuman()}`);
  console.log(`Fee Frozen: ${feeFrozen.toHuman()}`);
  
  await api.disconnect();
}

getAccountBalance('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY').catch(console.error);
```

### Transfer Tokens

JavaScript example:

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function transferTokens(fromSeed, toAddress, amount) {
  await cryptoWaitReady();
  
  // Connect to the node
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Create a keyring and add an account
  const keyring = new Keyring({ type: 'sr25519' });
  const fromAccount = keyring.addFromUri(fromSeed);
  
  console.log(`Sending ${amount} from ${fromAccount.address} to ${toAddress}`);
  
  // Create and sign transaction
  const transfer = api.tx.balances.transfer(toAddress, amount);
  
  // Sign and send the transaction
  const hash = await transfer.signAndSend(fromAccount);
  
  console.log(`Transaction hash: ${hash.toHex()}`);
  
  await api.disconnect();
}

// Example: Transfer 1 SEL
transferTokens('//Alice', '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000000000).catch(console.error);
```

### Query Blockchain Information

JavaScript example:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function getChainInfo() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Get chain information
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  
  console.log(`Chain: ${chain}`);
  console.log(`Node: ${nodeName} v${nodeVersion}`);
  
  // Get latest block
  const lastHeader = await api.rpc.chain.getHeader();
  console.log(`Latest block: #${lastHeader.number}`);
  
  // Get validation data
  const [validators, session] = await Promise.all([
    api.query.session.validators(),
    api.query.session.currentIndex()
  ]);
  
  console.log(`Current session: ${session}`);
  console.log(`Current validators: ${validators.length}`);
  
  await api.disconnect();
}

getChainInfo().catch(console.error);
```

### Listen to Chain Events

JavaScript example:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function subscribeToEvents() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  console.log('Subscribing to events...');
  
  // Subscribe to system events
  api.query.system.events((events) => {
    events.forEach((record) => {
      const { event, phase } = record;
      const types = event.typeDef;
      
      // Extract the data
      const eventData = event.data.map((data, index) => {
        return `${types[index].type}: ${data.toString()}`;
      });
      
      console.log(`Block: ${phase.toString()}`);
      console.log(`Event: ${event.section}.${event.method}:: ${event.meta.documentation.toString()}`);
      console.log(`Data: ${eventData.join(', ')}`);
    });
  });
  
  // Keep the script running
  return new Promise(() => {});
}

subscribeToEvents().catch(console.error);
```

## Working with Smart Contracts

### Query Contract State

JavaScript example for EVM contracts:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ethers } = require('ethers');

async function queryEVMContract() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Contract details
  const contractAddress = '0x123...'; // EVM contract address
  const contractABI = [ /* Contract ABI */ ];
  
  // Create ethers provider using Selendra's EVM RPC
  const evmProvider = new ethers.providers.JsonRpcProvider('http://127.0.0.1:9933/evm');
  const contract = new ethers.Contract(contractAddress, contractABI, evmProvider);
  
  // Call a read method
  const result = await contract.balanceOf('0x456...');
  console.log(`Balance: ${result.toString()}`);
  
  await api.disconnect();
}

queryEVMContract().catch(console.error);
```

### Call WASM Contract

JavaScript example for ink! contracts:

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');

async function callWasmContract() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Contract details
  const contractAddress = '5G...'; // Wasm contract address
  const contractABI = { /* Contract ABI */ };
  
  // Create contract instance
  const contract = new ContractPromise(api, contractABI, contractAddress);
  
  // Read contract state (query)
  const { result, output } = await contract.query.balanceOf(
    '', // Empty string as we're not sending value
    { gasLimit: -1 }, // Max gas
    '5F...' // Account to check balance of
  );
  
  if (result.isOk) {
    console.log(`Balance: ${output.toHuman()}`);
  }
  
  // Write to contract (transaction)
  const keyring = new Keyring({ type: 'sr25519' });
  const sender = keyring.addFromUri('//Alice');
  
  await contract.tx.transfer(
    { gasLimit: 3000n * 1000000n },
    '5F...', // Recipient
    1000 // Amount
  ).signAndSend(sender, (result) => {
    if (result.status.isInBlock) {
      console.log(`Transaction included in block: ${result.status.asInBlock}`);
    }
  });
  
  // Keep the script alive for transaction to complete
  await new Promise(resolve => setTimeout(resolve, 10000));
  
  await api.disconnect();
}

callWasmContract().catch(console.error);
```

## Advanced Topics

### Custom RPC Methods

Some Selendra pallets expose additional custom RPC methods:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function callCustomRPC() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  try {
    // Example of custom RPC call to a hypothetical pallet
    const result = await api.rpc.custom.methodName(param1, param2);
    console.log('Result:', result.toHuman());
  } catch (error) {
    console.error('Error calling custom RPC:', error);
  }
  
  await api.disconnect();
}

callCustomRPC().catch(console.error);
```

### Batching Transactions

Combining multiple operations into a single transaction:

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function batchTransactions() {
  await cryptoWaitReady();
  
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  const keyring = new Keyring({ type: 'sr25519' });
  const sender = keyring.addFromUri('//Alice');
  
  // Create multiple transfers
  const transfers = [
    api.tx.balances.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000000000),
    api.tx.balances.transfer('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY', 2000000000000)
  ];
  
  // Batch them into a single transaction
  const batch = api.tx.utility.batch(transfers);
  
  // Sign and send the batch
  const hash = await batch.signAndSend(sender);
  console.log(`Batch transaction submitted with hash: ${hash.toHex()}`);
  
  await new Promise(resolve => setTimeout(resolve, 10000));
  await api.disconnect();
}

batchTransactions().catch(console.error);
```

### Working with Metadata

Understanding chain metadata:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function inspectMetadata() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Get metadata
  const metadata = await api.rpc.state.getMetadata();
  
  // Extract module information
  const modules = metadata.asLatest.pallets;
  
  console.log(`Chain has ${modules.length} pallets`);
  
  // List all pallets and their calls
  modules.forEach((module) => {
    const name = module.name.toString();
    
    console.log(`\nPallet: ${name}`);
    
    if (module.calls.isSome) {
      const calls = module.calls.unwrap().calls;
      console.log('Available calls:');
      
      calls.forEach((call, index) => {
        const callName = call.name.toString();
        const docs = call.docs.map(d => d.toString()).join('\n');
        console.log(`  ${index}: ${callName} - ${docs}`);
      });
    } else {
      console.log('No calls available');
    }
  });
  
  await api.disconnect();
}

inspectMetadata().catch(console.error);
```

## Performance and Production Considerations

### Connection Management

Best practices for production environments:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

// Create a provider with custom settings
const provider = new WsProvider('wss://mainnet.selendra.org', {
  reconnect: true,
  reconnectDelay: 5000, // Wait 5s before reconnecting
  maxReconnects: 10     // Try up to 10 times
});

let api;

async function connectWithRetry() {
  try {
    api = await ApiPromise.create({ provider });
    
    // Handle disconnection
    api.on('disconnected', async () => {
      console.log('API disconnected. Attempting to reconnect...');
      await connectWithRetry();
    });
    
    console.log('API connected successfully');
    
    // Your application logic here
    
  } catch (error) {
    console.error('Failed to connect to API:', error);
    
    // Wait before retrying
    await new Promise(resolve => setTimeout(resolve, 10000));
    await connectWithRetry();
  }
}

// Handle graceful shutdown
process.on('SIGINT', async () => {
  if (api) {
    console.log('Disconnecting API...');
    await api.disconnect();
  }
  process.exit(0);
});

connectWithRetry().catch(console.error);
```

### Rate Limiting and Caching

For high-traffic applications:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const NodeCache = require('node-cache');

// Create a cache with 5-minute TTL
const cache = new NodeCache({ stdTTL: 300 });

async function getBalanceWithCache(address) {
  const cacheKey = `balance:${address}`;
  
  // Check if result is cached
  const cachedResult = cache.get(cacheKey);
  if (cachedResult) {
    console.log('Returning cached balance');
    return cachedResult;
  }
  
  // Not cached, query the chain
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  try {
    const { data: { free } } = await api.query.system.account(address);
    const balance = free.toString();
    
    // Cache the result
    cache.set(cacheKey, balance);
    
    return balance;
  } finally {
    await api.disconnect();
  }
}

// Implement request throttling
const MAX_REQUESTS_PER_MINUTE = 60;
let requestCount = 0;
let lastResetTime = Date.now();

function throttleRequests() {
  const now = Date.now();
  
  // Reset counter every minute
  if (now - lastResetTime > 60000) {
    requestCount = 0;
    lastResetTime = now;
  }
  
  if (requestCount >= MAX_REQUESTS_PER_MINUTE) {
    throw new Error('Rate limit exceeded. Try again later.');
  }
  
  requestCount++;
}

async function getRateLimitedBalance(address) {
  throttleRequests();
  return getBalanceWithCache(address);
}

// Example usage
getRateLimitedBalance('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY')
  .then(balance => console.log(`Balance: ${balance}`))
  .catch(console.error);
```

### Error Handling

Robust error handling for production applications:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function robustApiCall() {
  let api;
  
  try {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });
    
    // Try to get validators
    try {
      const validators = await api.query.session.validators();
      console.log(`Current validators: ${validators.length}`);
    } catch (queryError) {
      // Handle query-specific errors
      if (queryError.toString().includes('1010: Invalid transaction')) {
        console.error('Transaction is invalid:', queryError);
      } else if (queryError.toString().includes('1014: Priority is too low')) {
        console.error('Transaction priority too low:', queryError);
      } else {
        console.error('Query error:', queryError);
      }
    }
    
    // More operations...
    
  } catch (connectionError) {
    // Handle connection errors
    console.error('API connection error:', connectionError);
    
    // Attempt specific actions based on error
    if (connectionError.toString().includes('Connection refused')) {
      console.log('The node is not running or not accepting connections');
    } else if (connectionError.toString().includes('Connection timed out')) {
      console.log('The connection attempt timed out');
    }
  } finally {
    // Always clean up connection
    if (api) {
      await api.disconnect().catch(e => console.log('Disconnect error:', e));
    }
  }
}

robustApiCall().catch(e => console.error('Unexpected error:', e));
```

## Troubleshooting Common Issues

### Connection Problems

If you cannot connect to the node:

1. Verify the node is running: `curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"system_health","params":[]}' http://127.0.0.1:9933`
2. Check that RPC is enabled: Node should start with `--rpc-external` and/or `--ws-external`
3. Check network connectivity: `ping 127.0.0.1` or check firewall settings
4. Verify the correct port: Default HTTP is 9933, WS is 9944

### Transaction Failures

Common transaction error solutions:

1. **Insufficient balance**: Check account balance before sending
2. **Nonce too low/high**: Use `api.tx.<module>.<method>().signAndSend(account, { nonce: -1 })` to automatically use correct nonce
3. **Invalid format**: Ensure parameters match expected types
4. **Permissions**: Verify account has appropriate permissions for the call

Example nonce management:

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function sendWithProperNonce() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  const keyring = new Keyring({ type: 'sr25519' });
  const account = keyring.addFromUri('//Alice');
  
  // Get current nonce
  const nonce = await api.rpc.system.accountNextIndex(account.address);
  console.log(`Current nonce: ${nonce}`);
  
  // Use nonce explicitly
  const transfer = api.tx.balances.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000000000);
  
  try {
    const hash = await transfer.signAndSend(account, { nonce });
    console.log(`Transaction sent with hash: ${hash.toHex()}`);
  } catch (error) {
    console.error('Transaction error:', error);
  }
  
  await api.disconnect();
}

sendWithProperNonce().catch(console.error);
```

### Decoding Issues

If you encounter problems with data format:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToU8a, u8aToHex } = require('@polkadot/util');
const { decodeAddress, encodeAddress } = require('@polkadot/util-crypto');

async function handleDataFormats() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });
  
  // Convert between address formats
  try {
    const address = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    
    // Get public key from address
    const publicKey = decodeAddress(address);
    console.log(`Public key: ${u8aToHex(publicKey)}`);
    
    // Convert public key to different address format (e.g., Kusama)
    const kusamaAddress = encodeAddress(publicKey, 2);
    console.log(`Same account on Kusama: ${kusamaAddress}`);
  } catch (error) {
    console.error('Address conversion error:', error);
  }
  
  // Decode storage data
  try {
    // Query raw storage
    const storageKey = '0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9';
    const data = await api.rpc.state.getStorage(storageKey);
    
    if (data.isSome) {
      console.log('Raw data:', data.toString());
      
      // Manual decoding using API codec
      const decoded = api.createType('AccountInfo', data.unwrap());
      console.log('Decoded data:', decoded.toHuman());
    } else {
      console.log('No data found for this key');
    }
  } catch (error) {
    console.error('Storage decoding error:', error);
  }
  
  await api.disconnect();
}

handleDataFormats().catch(console.error);
```

## Security Best Practices

### Protecting Private Keys

Never hardcode private keys:

```javascript
// BAD - Don't do this
const privateKey = '0x1234...';

// GOOD - Use environment variables
const privateKey = process.env.PRIVATE_KEY;

// Even better - Use a secure vault or key management system
const { SecretManager } = require('@aws-sdk/client-secrets-manager');
const secretsManager = new SecretManager({ region: 'us-east-1' });

async function getPrivateKey() {
  const response = await secretsManager.getSecretValue({ SecretId: 'selendra-signer-key' });
  return JSON.parse(response.SecretString).privateKey;
}
```

### Validating User Input

Always validate user input before submitting to the chain:

```javascript
function validateAddress(address) {
  try {
    // Try to decode the address - will throw if invalid
    decodeAddress(address);
    return true;
  } catch (error) {
    console.error(`Invalid address: ${address}`);
    return false;
  }
}

function validateAmount(amount) {
  // Parse amount and check it's a positive number
  const parsedAmount = BigInt(amount);
  
  if (parsedAmount <= 0n) {
    console.error(`Amount must be positive: ${amount}`);
    return false;
  }
  
  // Check if amount is within reasonable limits
  const maxTransfer = 1000000000000000n; // Example limit
  
  if (parsedAmount > maxTransfer) {
    console.error(`Amount exceeds maximum transfer limit: ${amount}`);
    return false;
  }
  
  return true;
}

async function safeTransfer(fromAccount, toAddress, amount) {
  if (!validateAddress(toAddress) || !validateAmount(amount)) {
    throw new Error('Invalid transfer parameters');
  }
  
  // Proceed with transfer
  // ...
}
```

### Rate Limiting and DoS Protection

Protect your service from overload:

```javascript
const express = require('express');
const rateLimit = require('express-rate-limit');
const { ApiPromise, WsProvider } = require('@polkadot/api');

const app = express();

// Apply rate limiting middleware
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // limit each IP to 100 requests per windowMs
  message: 'Too many requests from this IP, please try again later'
});

// Apply to all API endpoints
app.use('/api/', limiter);

// Create a shared API instance
let api;
(async () => {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  api = await ApiPromise.create({ provider });
})().catch(console.error);

// Example API endpoint
app.get('/api/balance/:address', async (req, res) => {
  try {
    const { address } = req.params;
    
    // Validate address
    try {
      decodeAddress(address);
    } catch (error) {
      return res.status(400).json({ error: 'Invalid address format' });
    }
    
    // Get balance
    const { data: { free } } = await api.query.system.account(address);
    
    return res.json({ address, balance: free.toString() });
  } catch (error) {
    console.error('API error:', error);
    return res.status(500).json({ error: 'Internal server error' });
  }
});

app.listen(3000, () => {
  console.log('Server running on port 3000');
});

// Handle graceful shutdown
process.on('SIGINT', async () => {
  if (api) {
    await api.disconnect();
  }
  process.exit(0);
});
```

## Next Steps

After mastering the RPC API, consider exploring:

- [Smart Contract Development](./evm-contracts.md) with Solidity on Selendra
- [WebAssembly Smart Contracts](./wasm-contracts.md) using ink!
- [Building Your First dApp](./first-dapp.md) with Selendra
- [Testing Smart Contracts](./contract-testing.md) effectively

For additional assistance, join the Selendra developer community on:
- [Discord](https://discord.gg/selendra)
- [Telegram](https://t.me/selendrachain)
- [Forum](https://forum.selendra.org) 