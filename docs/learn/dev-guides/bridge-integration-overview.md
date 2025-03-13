# Bridge Integration Overview

This guide provides an introduction to cross-chain bridges on Selendra and how developers can leverage them in their applications.

## What are Blockchain Bridges?

Blockchain bridges are protocols that enable the transfer of assets and data between different blockchain networks. They serve as connectors that allow users to:

- Transfer tokens between chains
- Use assets from one blockchain on another
- Interact with dApps across multiple blockchains

## Selendra Bridge Architecture

Selendra implements a robust bridge architecture that connects to multiple blockchain ecosystems:

```
                    ┌─────────────────┐
                    │                 │
                    │    Ethereum     │
                    │                 │
                    └────────┬────────┘
                             │
                             ▼
┌─────────────┐     ┌─────────────────┐     ┌─────────────────┐
│             │     │                 │     │                 │
│  Binance    │◄───►│    Selendra     │◄───►│    Polkadot     │
│ Smart Chain │     │     Bridge      │     │    Ecosystem    │
│             │     │     System      │     │                 │
└─────────────┘     └─────────────────┘     └─────────────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │                 │
                    │  Other Chains   │
                    │  (Future)       │
                    └─────────────────┘
```

## Bridge Types Supported

Selendra supports multiple bridge implementations:

### 1. Trusted Bridges (Federated)

- **Security Model**: Relies on a set of trusted validators
- **Speed**: Fast finality (minutes)
- **Use Cases**: Most token transfers and basic cross-chain interactions
- **Implementation**: Multi-signature federation securing bridge funds

### 2. Trustless Bridges (Light Clients)

- **Security Model**: Uses on-chain verification of the source chain
- **Speed**: Medium finality (depends on source chain)
- **Use Cases**: High-value transfers requiring maximum security
- **Implementation**: Substrate light clients for connected chains

### 3. Optimistic Bridges

- **Security Model**: Assumes transactions are valid but allows for fraud proofs
- **Speed**: Fast for users, with challenge period for security
- **Use Cases**: Balancing security and user experience
- **Implementation**: Challenge-response system with time delays

## Integrated Bridged Assets

The following assets are currently bridged to Selendra:

| Asset | Origin Chain | Bridge Type | Notes |
|-------|-------------|------------|-------|
| WETH | Ethereum | Trusted | Wrapped ETH on Selendra |
| USDT | Ethereum | Trusted | ERC-20 stablecoin |
| USDC | Ethereum | Trusted | ERC-20 stablecoin |
| BNB | BSC | Trusted | Binance Coin |
| DOT | Polkadot | Trustless | Native Polkadot token |

## Integrating Bridges in Your dApp

### Prerequisites

Before integrating bridges into your application:

1. Understand the security model of each bridge
2. Be familiar with Selendra's EVM and Substrate APIs
3. Know how to interact with standard token contracts

### Basic Integration Steps

#### 1. Bridge Detection

Detect if a user has access to bridge functionality:

```javascript
// Example using JavaScript
async function checkBridgeAvailability() {
  const api = await ApiPromise.create({ provider: new WsProvider('wss://mainnet.selendra.org') });
  
  // Check if bridge module exists and is active
  const bridgeModules = await api.query.system.account.keys();
  const hasBridgeModule = bridgeModules.some(key => key.toString().includes('bridge'));
  
  return hasBridgeModule;
}
```

#### 2. Asset Verification

Verify bridge-related assets on Selendra:

```javascript
// Check if a token is a bridged asset
async function isBridgedToken(tokenAddress) {
  const bridgedAssets = [
    '0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063', // (Example) Bridged USDT
    '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174', // (Example) Bridged USDC
    // Add other known bridged tokens
  ];
  
  return bridgedAssets.includes(tokenAddress);
}
```

#### 3. Initiating Bridge Transfers

Example of initiating a bridge transfer from your dApp:

```javascript
async function initiateEthereumBridgeTransfer(amount, recipient, token) {
  // Bridge contract address on Ethereum
  const bridgeContract = '0xBridgeContractAddress';
  
  // Create contract instance (using ethers.js)
  const provider = new ethers.providers.Web3Provider(window.ethereum);
  const signer = provider.getSigner();
  const contract = new ethers.Contract(bridgeContract, BridgeABI, signer);
  
  // Approve token transfer if needed
  const tokenContract = new ethers.Contract(token, ERC20ABI, signer);
  await tokenContract.approve(bridgeContract, amount);
  
  // Call bridge function
  const tx = await contract.bridgeAsset(token, amount, recipient);
  return tx.hash;
}
```

#### 4. Listening for Bridge Events

Monitor bridge events to notify users of transfer completion:

```javascript
function listenForBridgeCompletionEvents(txHash, callback) {
  const provider = new ethers.providers.Web3Provider(window.ethereum);
  const selendraProvider = new WsProvider('wss://mainnet.selendra.org');
  
  // Set up event listener on Selendra
  const unsubscribe = selendraProvider.subscribe(
    'BridgeCompleted',
    (event) => {
      if (event.originTxHash === txHash) {
        callback(event);
      }
    }
  );
  
  return unsubscribe;
}
```

## Security Considerations

When integrating with bridges, keep these security considerations in mind:

1. **Understand the Trust Model**: Different bridges have different security assumptions
2. **Validate Destination Addresses**: Always validate recipient addresses before initiating transfers
3. **Handle Delays Gracefully**: Communicate expected bridge finality times to users
4. **Verify Asset Contracts**: Ensure you're interacting with the correct bridged asset contracts
5. **Implement Proper Error Handling**: Bridge transfers can fail for various reasons

## Common Bridge Integration Patterns

### 1. Bridge UI Component

Create a reusable bridge UI component for your dApp:

```jsx
function BridgeWidget({ supportedTokens, onBridgeComplete }) {
  const [selectedToken, setSelectedToken] = useState(null);
  const [amount, setAmount] = useState('');
  const [destinationAddress, setDestinationAddress] = useState('');
  
  const handleBridge = async () => {
    // Bridge logic here
  };
  
  return (
    <div className="bridge-widget">
      <h3>Bridge Assets</h3>
      {/* UI for token selection, amount input, etc. */}
      <button onClick={handleBridge}>Initiate Bridge Transfer</button>
    </div>
  );
}
```

### 2. Bridge Transaction Status Tracker

Implement a system to track the status of bridge transactions:

```javascript
const bridgeStatuses = {
  INITIATED: 'Initiated on source chain',
  CONFIRMED: 'Confirmed on source chain',
  PROCESSING: 'Processing by bridge validators',
  COMPLETED: 'Completed on destination chain',
  FAILED: 'Failed'
};

class BridgeTransactionTracker {
  constructor(sourceChain, destChain, txHash) {
    this.sourceChain = sourceChain;
    this.destChain = destChain;
    this.txHash = txHash;
    this.status = bridgeStatuses.INITIATED;
    this.listeners = [];
  }
  
  updateStatus(newStatus) {
    this.status = newStatus;
    this.notifyListeners();
  }
  
  addStatusListener(callback) {
    this.listeners.push(callback);
  }
  
  notifyListeners() {
    this.listeners.forEach(listener => listener(this.status));
  }
  
  // Methods to check status on both chains
}
```

## Testing Bridge Integrations

Before deploying to production:

1. **Use Testnet Bridges**: All major bridges have testnet deployments
2. **Test Edge Cases**: Try different asset types, amounts, and error conditions
3. **Simulate Delays**: Ensure your UI handles bridge processing times correctly
4. **Validate Token Contracts**: Confirm bridged assets appear correctly

## Additional Resources

- [Selendra Bridge API Documentation](https://docs.selendra.org/bridge-api)
- [Ethereum Bridge Contract Reference](https://etherscan.io/address/0xbridge)
- [Bridge Security Audit Reports](https://docs.selendra.org/security)

## Next Steps

- Learn about [connecting to external chains](./connecting-external-chains.md) in detail
- Explore [transaction handling](./transaction-handling.md) for cross-chain operations 