# Debugging Guide for Selendra Development

This comprehensive guide covers common issues developers may encounter when building on Selendra and provides troubleshooting strategies and solutions to help you overcome these challenges.

## Understanding Selendra Architecture for Debugging

To effectively debug applications on Selendra, it's important to understand the system's architecture:

1. **Runtime**: The blockchain's state transition logic
2. **Node**: The client software running the blockchain
3. **RPC Layer**: API endpoints for interacting with the chain
4. **Smart Contracts**: Code that runs on the blockchain (EVM and WASM)
5. **Frontend Applications**: User interfaces interacting with the chain

Issues can occur at any of these layers, so debugging requires a systematic approach.

## Essential Debugging Tools

### Network Tools

- **Polkadot.js Apps**: Web interface for chain interaction at [https://polkadot.js.org/apps/](https://polkadot.js.org/apps/)
- **Selendra Explorer**: Blockchain explorer for transaction and block information
- **RPC Endpoints**: Test endpoints for interacting with the chain

### Development Tools

- **Console Logging**: Essential for debugging JavaScript/TypeScript applications
- **Hardhat Network**: Local Ethereum-compatible development environment
- **ink! Debug**: Debugging capabilities for ink! contracts
- **Browser DevTools**: For debugging frontend applications
- **Substrate Debug Kit**: Tools for on-chain debugging

### Monitoring Tools

- **Prometheus & Grafana**: For monitoring node performance
- **Selendra Telemetry**: Network telemetry information

## Common Issues and Solutions

### Transaction Failures

#### Issue: Transaction Fails with "Transaction Error: 1010"

This error typically means your transaction has a fundamental issue.

**Solutions:**

1. **Check Parameters**:
   ```javascript
   // Ensure all parameters match expected types
   // Example: Ensure amounts are within range
   const MAX_SAFE_AMOUNT = 1n * 10n ** 18n * 10n ** 9n; // avoid integer overflow
   if (amount > MAX_SAFE_AMOUNT) {
     console.error("Amount too large, will cause overflow");
   }
   ```

2. **Verify Signature**:
   ```javascript
   // Ensure you're using the correct account
   const keyring = new Keyring({ type: 'sr25519' });
   const account = keyring.addFromMnemonic('your mnemonic here');
   console.log(`Using account: ${account.address}`);
   // Verify this matches your expected sending address
   ```

3. **Check Extrinsic Format**:
   ```javascript
   // Log the formatted transaction before sending
   const tx = api.tx.balances.transfer(destination, amount);
   console.log('TX data:', tx.toHex());
   ```

#### Issue: "Inability to pay some fees"

Your account does not have enough balance to pay for transaction fees.

**Solutions:**

1. **Check Balance**:
   ```javascript
   const { data: { free, reserved } } = await api.query.system.account(address);
   console.log(`Free balance: ${free.toString()}`);
   console.log(`Reserved balance: ${reserved.toString()}`);
   ```

2. **Estimate Fees Before Sending**:
   ```javascript
   // Create the transaction
   const tx = api.tx.balances.transfer(destination, amount);
   
   // Get fee information
   const info = await tx.paymentInfo(sender);
   console.log(`Estimated fees: ${info.partialFee.toHuman()}`);
   
   // Only send if account has sufficient balance
   const { data: { free } } = await api.query.system.account(sender.address);
   if (free.sub(amount).sub(info.partialFee).isNeg()) {
     console.error("Insufficient funds for transaction + fees");
     return;
   }
   ```

### Smart Contract Issues

#### Issue: EVM Contract Deployment Failures

**Solutions:**

1. **Check Gas Limit**:
   ```javascript
   // Increase gas limit for complex contracts
   const deploymentResult = await contractFactory.deploy({
     gasLimit: 5000000, // Increase from default
   });
   ```

2. **Verify Contract Size**:
   ```javascript
   // Check if contract bytecode is within size limits
   const bytecode = contractFactory.bytecode;
   console.log(`Contract size: ${bytecode.length / 2 - 1} bytes`);
   if (bytecode.length / 2 - 1 > 24576) {
     console.error("Contract exceeds size limit, optimize code");
   }
   ```

3. **Check Constructor Arguments**:
   ```javascript
   // Log constructor arguments to verify format
   console.log("Constructor args:", JSON.stringify(args, null, 2));
   ```

#### Issue: WASM (ink!) Contract Issues

**Solutions:**

1. **Validate Contract Code**:
   ```bash
   # Run the ink! linter
   cargo clippy --all-features
   
   # Check contract size
   cargo contract build --release
   ls -la target/ink/my_contract.wasm
   ```

2. **Debug with Tracing**:
   ```rust
   // Add debug statements in ink! contract
   #[ink(message)]
   pub fn my_function(&self) -> Result<()> {
       ink_env::debug_println!("Entering my_function");
       // function logic
       ink_env::debug_println!("Exiting my_function");
       Ok(())
   }
   ```

3. **Check Storage Layout**:
   ```rust
   // Ensure storage layout is compatible with previous versions
   #[ink(storage)]
   pub struct MyContract {
       // Fields should be added to the end to maintain compatibility
       value: bool,
       // New fields here
   }
   ```

### Connection and Network Issues

#### Issue: Cannot Connect to Selendra Node

**Solutions:**

1. **Check Node Status**:
   ```bash
   # Check if node is running
   curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"system_health","params":[]}' http://127.0.0.1:9933
   ```

2. **Verify WebSocket Connection**:
   ```javascript
   const provider = new WsProvider('ws://127.0.0.1:9944');
   provider.on('error', (error) => {
     console.error('WebSocket error:', error);
   });
   provider.on('connected', () => {
     console.log('WebSocket connected!');
   });
   ```

3. **Configure Timeout Settings**:
   ```javascript
   // Increase timeout for connection
   const provider = new WsProvider('ws://127.0.0.1:9944', 1000);
   // 1000ms connection timeout
   ```

#### Issue: API Connection Drops Frequently

**Solutions:**

1. **Implement Reconnection Logic**:
   ```javascript
   const provider = new WsProvider('ws://127.0.0.1:9944', {
     reconnect: true,
     reconnectDelay: 5000, // 5s between reconnect attempts
     maxReconnects: 5
   });
   
   const api = await ApiPromise.create({ provider });
   
   api.on('disconnected', () => {
     console.log('API has been disconnected');
   });
   
   api.on('connected', () => {
     console.log('API has been connected');
   });
   ```

2. **Handle Disconnect Events**:
   ```javascript
   api.on('error', (error) => {
     console.error('API error:', error);
     // Implement your recovery strategy
   });
   ```

### Frontend Integration Issues

#### Issue: MetaMask Not Connecting to Selendra EVM

**Solutions:**

1. **Configure Network Correctly**:
   ```javascript
   // Request network addition
   async function addSelendraToMetaMask() {
     try {
       await window.ethereum.request({
         method: 'wallet_addEthereumChain',
         params: [{
           chainId: '0x7CA', // 1994 in hex
           chainName: 'Selendra',
           nativeCurrency: {
             name: 'SEL',
             symbol: 'SEL',
             decimals: 18
           },
           rpcUrls: ['https://mainnet.selendra.org/evm'],
           blockExplorerUrls: ['https://explorer.selendra.org']
         }]
       });
       console.log('Selendra network added to MetaMask');
     } catch (error) {
       console.error('Failed to add network:', error);
     }
   }
   ```

2. **Verify Chain ID**:
   ```javascript
   // Check connected chain
   async function checkChainId() {
     try {
       const chainId = await window.ethereum.request({ method: 'eth_chainId' });
       console.log(`Connected to chain ID: ${parseInt(chainId, 16)}`);
       
       if (parseInt(chainId, 16) !== 1994) { // Selendra chain ID
         console.error('Please switch to Selendra network');
       }
     } catch (error) {
       console.error('Error checking chain:', error);
     }
   }
   ```

#### Issue: PolkadotJS Extension Not Detecting Selendra

**Solutions:**

1. **Set Network Properties**:
   ```javascript
   // Configure the network properties
   const properties = {
     ss58Format: 42,
     tokenDecimals: 18,
     tokenSymbol: 'SEL'
   };
   
   // Create the API with these properties
   const api = await ApiPromise.create({
     provider,
     properties
   });
   ```

2. **Check for Extension**:
   ```javascript
   async function checkExtension() {
     if (!window.injectedWeb3['polkadot-js']) {
       console.error('Polkadot{.js} extension not found!');
       return false;
     }
     
     // Enable the extension
     const extension = await window.injectedWeb3['polkadot-js'].enable('My dApp');
     console.log('Extension enabled, accounts:', extension.accounts.length);
     return true;
   }
   ```

### Common Performance Issues

#### Issue: Slow Query Performance

**Solutions:**

1. **Implement Caching**:
   ```javascript
   // Simple in-memory cache
   const cache = new Map();
   const CACHE_TTL = 30000; // 30 seconds
   
   async function getBalanceWithCache(address) {
     const cacheKey = `balance:${address}`;
     const now = Date.now();
     
     // Check cache
     if (cache.has(cacheKey)) {
       const { value, timestamp } = cache.get(cacheKey);
       if (now - timestamp < CACHE_TTL) {
         console.log('Cache hit');
         return value;
       }
     }
     
     // Get fresh value
     const { data: { free } } = await api.query.system.account(address);
     const balance = free.toString();
     
     // Update cache
     cache.set(cacheKey, { value: balance, timestamp: now });
     
     return balance;
   }
   ```

2. **Batch Queries**:
   ```javascript
   // Instead of querying multiple accounts separately
   async function getMultipleBalances(addresses) {
     // Use multi-query
     const balances = await api.query.system.account.multi(addresses);
     
     return addresses.map((address, index) => ({
       address,
       balance: balances[index].data.free.toString()
     }));
   }
   ```

#### Issue: High Resource Usage on Node

**Solutions:**

1. **Monitor Resource Usage**:
   ```bash
   # Check node process resource usage
   ps aux | grep selendra
   
   # Monitor in real-time
   htop -p $(pgrep selendra)
   ```

2. **Adjust Node Configuration**:
   ```bash
   # Run with constrained resources
   selendra --db-cache 512 --wasm-execution Compiled --pruning archive
   ```

## Debugging Specific Components

### Debugging RPC Calls

```javascript
// Enable debug mode for API
const api = await ApiPromise.create({
  provider,
  noInitWarn: true,
  throwOnConnect: true,
  throwOnUnknown: true
});

// Track pending requests
const pendingRequests = new Map();

// Add a timestamp when request is sent
const send = api.provider.send;
api.provider.send = function(method, params) {
  const requestId = `${method}:${Date.now()}`;
  pendingRequests.set(requestId, {
    method,
    params,
    timestamp: Date.now()
  });
  
  console.log(`[RPC REQUEST] ${method} with params:`, params);
  
  return send.call(this, method, params)
    .then(result => {
      const request = pendingRequests.get(requestId);
      const elapsed = Date.now() - request.timestamp;
      console.log(`[RPC RESPONSE] ${method} completed in ${elapsed}ms`);
      pendingRequests.delete(requestId);
      return result;
    })
    .catch(error => {
      console.error(`[RPC ERROR] ${method} failed:`, error);
      pendingRequests.delete(requestId);
      throw error;
    });
};

// Periodically check for stuck requests
setInterval(() => {
  const now = Date.now();
  pendingRequests.forEach((request, id) => {
    const elapsed = now - request.timestamp;
    if (elapsed > 10000) { // 10 seconds
      console.warn(`[RPC WARNING] Request ${request.method} has been pending for ${elapsed}ms`);
    }
  });
}, 5000);
```

### Debugging EVM Contracts

```javascript
// Using Hardhat's console.log for Solidity debugging
// In your Solidity contract:
import "hardhat/console.sol";

contract Debugging {
    function complexOperation(uint x, uint y) public {
        console.log("Input values: %s, %s", x, y);
        
        uint result = x * y;
        console.log("Calculated result: %s", result);
        
        // More operations...
    }
}

// In your JavaScript test/deployment:
const { expect } = require("chai");

describe("Debugging", function() {
  it("Should perform complex operation", async function() {
    // Deploy with console log enabled
    const Debugging = await ethers.getContractFactory("Debugging");
    const debug = await Debugging.deploy();
    
    // Call with traced execution
    const tx = await debug.complexOperation(5, 10);
    const receipt = await tx.wait();
    
    // Logs will be output to console
    console.log("Gas used:", receipt.gasUsed.toString());
  });
});
```

### Debugging WASM (ink!) Contracts

```rust
// In your ink! contract
#[ink::contract]
mod debugging {
    #[ink(storage)]
    pub struct Debugging {
        value: u32,
    }
    
    impl Debugging {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            ink_env::debug_println!("Constructor called with value: {}", init_value);
            Self { value: init_value }
        }
        
        #[ink(message)]
        pub fn increment(&mut self, by: u32) {
            ink_env::debug_println!("Current value: {}, incrementing by: {}", self.value, by);
            self.value += by;
            ink_env::debug_println!("New value: {}", self.value);
        }
        
        #[ink(message)]
        pub fn get(&self) -> u32 {
            ink_env::debug_println!("Getting value: {}", self.value);
            self.value
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[ink::test]
        fn test_increment() {
            // Create contract with initial value 42
            let mut contract = Debugging::new(42);
            assert_eq!(contract.get(), 42);
            
            // Increment by 5
            contract.increment(5);
            assert_eq!(contract.get(), 47);
        }
    }
}
```

### Debugging Substrate Interactions

```javascript
// Enable detailed logging for chain state
async function inspectStateWithLogs(api, modulePrefix) {
  // Get metadata for keys
  const metadata = await api.rpc.state.getMetadata();
  
  // Find module in metadata
  const modules = metadata.asLatest.pallets;
  const moduleIndex = modules.findIndex(m => m.name.toString() === modulePrefix);
  
  if (moduleIndex === -1) {
    console.error(`Module ${modulePrefix} not found in metadata`);
    return;
  }
  
  console.log(`Inspecting ${modulePrefix} storage...`);
  
  // Get all storage items
  const storageItems = modules[moduleIndex].storage.unwrap().items;
  
  for (const item of storageItems) {
    const itemName = item.name.toString();
    console.log(`\nStorage item: ${modulePrefix}.${itemName}`);
    console.log(`Description: ${item.docs.map(d => d.toString()).join(' ')}`);
    
    try {
      // Attempt to query this storage item
      const value = await api.query[modulePrefix][itemName]();
      console.log(`Value: ${JSON.stringify(value.toHuman(), null, 2)}`);
    } catch (error) {
      console.log(`Could not query ${itemName}: ${error.message}`);
    }
  }
}

// Usage:
// inspectStateWithLogs(api, 'balances');
```

## Debugging Tools and Techniques

### Advanced Console Debugging

```javascript
// Create a custom logger with timestamps and categories
function createLogger(category, verbose = false) {
  return {
    log: (...args) => console.log(`[${new Date().toISOString()}] [${category}] [INFO]`, ...args),
    warn: (...args) => console.log(`[${new Date().toISOString()}] [${category}] [WARN]`, ...args),
    error: (...args) => console.log(`[${new Date().toISOString()}] [${category}] [ERROR]`, ...args),
    debug: (...args) => {
      if (verbose) console.log(`[${new Date().toISOString()}] [${category}] [DEBUG]`, ...args)
    }
  };
}

// Usage
const txLogger = createLogger('TRANSACTION', true);
const rpcLogger = createLogger('RPC', false);

// Logging transactions
txLogger.log('Creating transaction');
txLogger.debug('Transaction details:', tx.toHex());

// Logging RPC calls
rpcLogger.log('Connecting to node');
rpcLogger.warn('Slow response time detected');
```

### Using Substrate Debug Flags

```bash
# Run a Selendra node with debugging flags
RUST_LOG=debug,runtime=trace ./target/release/selendra \
  --dev \
  --tmp \
  --rpc-methods Unsafe \
  --rpc-cors all
```

### Monitoring Transaction Lifecycle

```javascript
async function monitorTransactionLifecycle(api, txHash) {
  console.log(`Monitoring transaction: ${txHash}`);
  
  // Subscribe to transaction status
  const unsub = await api.rpc.author.submitAndWatchExtrinsic(txHash, (status) => {
    console.log(`Current transaction status: ${status.type}`);
    
    if (status.isInBlock) {
      console.log(`Transaction included in block: ${status.asInBlock}`);
    }
    
    if (status.isFinalized) {
      console.log(`Transaction finalized in block: ${status.asFinalized}`);
      unsub();
    }
    
    if (status.isDropped) {
      console.error('Transaction dropped from the network');
      unsub();
    }
    
    if (status.isInvalid) {
      console.error('Transaction is invalid');
      unsub();
    }
  });
}
```

## Troubleshooting Patterns and Best Practices

### Creating a Minimal Reproducible Example

1. **Isolate the Issue**:
   - Identify the specific component or function causing problems
   - Remove unrelated code and dependencies

2. **Step-by-Step Example**:
   ```bash
   # Create a minimal project
   mkdir selendra-issue
   cd selendra-issue
   
   # Initialize Node.js project
   npm init -y
   
   # Install minimal dependencies
   npm install @polkadot/api
   
   # Create a single file test
   touch reproduce.js
   
   # Add only the code necessary to reproduce the issue
   # Example for a connection issue:
   cat > reproduce.js << 'EOF'
   const { ApiPromise, WsProvider } = require('@polkadot/api');
   
   async function reproduceIssue() {
     const provider = new WsProvider('wss://mainnet.selendra.org');
     const api = await ApiPromise.create({ provider });
     
     try {
       // Only the problematic operation
       const result = await api.query.system.account('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
       console.log('Result:', result.toHuman());
     } catch (error) {
       console.error('Error:', error);
     }
     
     await api.disconnect();
   }
   
   reproduceIssue().catch(console.error);
   EOF
   
   # Run the test
   node reproduce.js
   ```

### Effective Error Handling Strategies

1. **Use Descriptive Error Messages**:
   ```javascript
   try {
     // Potentially failing operation
     const result = await api.query.module.function(param);
     return result;
   } catch (error) {
     // Add context to the error
     throw new Error(`Failed to query module.function: ${error.message}`);
   }
   ```

2. **Implement Error Recovery**:
   ```javascript
   async function queryWithRetry(fn, maxRetries = 3, delay = 1000) {
     let lastError;
     
     for (let attempt = 1; attempt <= maxRetries; attempt++) {
       try {
         return await fn();
       } catch (error) {
         console.warn(`Attempt ${attempt}/${maxRetries} failed: ${error.message}`);
         lastError = error;
         
         if (attempt < maxRetries) {
           console.log(`Retrying in ${delay}ms...`);
           await new Promise(resolve => setTimeout(resolve, delay));
           // Exponential backoff
           delay *= 2;
         }
       }
     }
     
     throw new Error(`Operation failed after ${maxRetries} attempts: ${lastError.message}`);
   }
   
   // Usage
   try {
     const balance = await queryWithRetry(() => api.query.system.account(address));
     console.log('Balance:', balance.data.free.toHuman());
   } catch (error) {
     console.error('Failed to get balance:', error);
   }
   ```

3. **Categorize Errors**:
   ```javascript
   function handleError(error) {
     // Connection errors
     if (error.message.includes('WebSocket')) {
       console.error('Network error, check your connection:', error.message);
       return { type: 'CONNECTION', message: 'Network connection issue' };
     }
     
     // API errors
     if (error.message.includes('1010:')) {
       console.error('Transaction error:', error.message);
       return { type: 'TRANSACTION', message: 'Invalid transaction parameters' };
     }
     
     // Contract errors
     if (error.message.includes('Contract call failed')) {
       console.error('Contract error:', error.message);
       return { type: 'CONTRACT', message: 'Smart contract execution failed' };
     }
     
     // Unknown errors
     console.error('Unknown error:', error);
     return { type: 'UNKNOWN', message: error.message };
   }
   ```

### Creating an Effective Bug Report

When reporting bugs to the Selendra team, include:

1. **Environment Information**:
   ```
   - Selendra version: (e.g., v3.0.0)
   - Node.js version: (e.g., v16.15.0)
   - Operating System: (e.g., Ubuntu 22.04)
   - Polkadot.js API version: (e.g., 8.5.1)
   ```

2. **Issue Description**:
   ```
   Brief description of the issue, what you expected to happen,
   and what actually happened.
   ```

3. **Steps to Reproduce**:
   ```
   1. Initialize API connection to endpoint X
   2. Call specific method Y with parameters Z
   3. Observe error with message "..."
   ```

4. **Code Sample**:
   ```javascript
   // Minimal code that reproduces the issue
   const { ApiPromise, WsProvider } = require('@polkadot/api');
   
   async function reproduceIssue() {
     // Problem reproduction code
   }
   ```

5. **Error Logs**:
   ```
   Full error message and stack trace
   ```

6. **Screenshots** (if applicable)

## Debugging in Production

### Implementing Fallback Mechanisms

```javascript
class ResilientApi {
  constructor(endpoints) {
    this.endpoints = endpoints;
    this.currentEndpointIndex = 0;
    this.api = null;
  }
  
  async connect() {
    let attempts = 0;
    const maxAttempts = this.endpoints.length * 2; // Try each endpoint at least twice
    
    while (attempts < maxAttempts && !this.api) {
      const endpoint = this.endpoints[this.currentEndpointIndex];
      console.log(`Attempting to connect to ${endpoint}`);
      
      try {
        const provider = new WsProvider(endpoint);
        this.api = await ApiPromise.create({ provider });
        console.log(`Connected to ${endpoint}`);
        
        // Handle disconnects
        this.api.on('disconnected', async () => {
          console.warn(`Disconnected from ${endpoint}`);
          this.api = null;
          
          // Try to reconnect, switching endpoint
          this.currentEndpointIndex = (this.currentEndpointIndex + 1) % this.endpoints.length;
          await this.connect();
        });
        
        return this.api;
      } catch (error) {
        console.error(`Failed to connect to ${endpoint}: ${error.message}`);
        this.currentEndpointIndex = (this.currentEndpointIndex + 1) % this.endpoints.length;
        attempts++;
      }
    }
    
    throw new Error('Failed to connect to any endpoint');
  }
  
  async ensureConnected() {
    if (!this.api) {
      return this.connect();
    }
    return this.api;
  }
  
  async query(module, method, ...args) {
    const api = await this.ensureConnected();
    try {
      return await api.query[module][method](...args);
    } catch (error) {
      console.error(`Query error (${module}.${method}): ${error.message}`);
      throw error;
    }
  }
  
  async tx(module, method, ...args) {
    const api = await this.ensureConnected();
    return api.tx[module][method](...args);
  }
}

// Usage
const resilientApi = new ResilientApi([
  'wss://mainnet.selendra.org',
  'wss://rpc.selendra.org',
  'wss://selendra-rpc.publicnode.com'
]);

async function getBalanceSafely(address) {
  try {
    const account = await resilientApi.query('system', 'account', address);
    return account.data.free.toString();
  } catch (error) {
    console.error('Failed to get balance:', error);
    return null;
  }
}
```

### Monitoring and Alerting

```javascript
// Simple health check endpoint for your application
const express = require('express');
const app = express();

// Track API health
let apiHealth = {
  lastSuccessfulQuery: null,
  failedQueries: 0,
  isHealthy: false
};

// Health check function
async function checkApiHealth(api) {
  try {
    // Attempt a simple query
    const header = await api.rpc.chain.getHeader();
    
    apiHealth.lastSuccessfulQuery = Date.now();
    apiHealth.isHealthy = true;
    apiHealth.failedQueries = 0;
    
    return true;
  } catch (error) {
    apiHealth.failedQueries++;
    
    if (apiHealth.failedQueries > 3) {
      apiHealth.isHealthy = false;
      
      // Send alert if not healthy
      sendAlertToTeam(`API health check failed ${apiHealth.failedQueries} times. Last error: ${error.message}`);
    }
    
    return false;
  }
}

// Example alert function
function sendAlertToTeam(message) {
  console.error(`ALERT: ${message}`);
  
  // In production, you might use:
  // - Email alerts
  // - SMS notifications
  // - Slack/Discord webhooks
  // - PagerDuty/Opsgenie integration
}

// Health check API
app.get('/health', (req, res) => {
  if (apiHealth.isHealthy) {
    return res.status(200).json({
      status: 'ok',
      lastSuccessfulQuery: apiHealth.lastSuccessfulQuery,
      message: 'API is healthy'
    });
  } else {
    return res.status(503).json({
      status: 'error',
      failedQueries: apiHealth.failedQueries,
      lastSuccessfulQuery: apiHealth.lastSuccessfulQuery,
      message: 'API is not responding correctly'
    });
  }
});

// Start server
app.listen(3000, () => {
  console.log('Health check server running on port 3000');
});

// Run periodic health checks
setInterval(() => {
  checkApiHealth(api).catch(console.error);
}, 60000); // Every minute
```

## Next Steps

After mastering debugging on Selendra, consider exploring:

- [EVM Contracts Guide](./evm-contracts.md): Build and deploy EVM contracts
- [WASM Contracts Guide](./wasm-contracts.md): Develop ink! smart contracts
- [Testing Guide](./contract-testing.md): Advanced testing techniques
- [RPC API Guide](./rpc-api-guide.md): Mastering the RPC interface

Join the Selendra developer community for support:
- [Discord](https://discord.gg/selendra)
- [Telegram](https://t.me/selendrachain)
- [Forum](https://forum.selendra.org)

Use the debugging techniques in this guide to solve common issues, but remember to ask for help from the community when needed. Debugging is often a collaborative process! 