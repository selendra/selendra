# Bitriel Wallet Documentation

## Overview

Bitriel is the official wallet for the Selendra ecosystem, providing secure asset management, DApp integration, and cross-chain capabilities.

## Features

### 1. Multi-Chain Support
- Selendra Network (Native)
- Ethereum
- BSC
- Polygon
- Other EVM-compatible chains

### 2. Asset Management
```typescript
// Asset Management Interface
interface AssetManager {
    getBalance(address: string, token: string): Promise<string>;
    sendTransaction(params: {
        to: string;
        amount: string;
        token: string;
        network: string;
    }): Promise<TransactionResult>;
    importToken(params: {
        address: string;
        symbol: string;
        decimals: number;
        network: string;
    }): Promise<void>;
}

// Implementation Example
class BitrielAssetManager implements AssetManager {
    async getBalance(address: string, token: string): Promise<string> {
        if (token === 'SEL') {
            return this.getNativeBalance(address);
        }
        return this.getTokenBalance(address, token);
    }

    async sendTransaction(params: TransactionParams): Promise<TransactionResult> {
        const { to, amount, token, network } = params;
        // Transaction logic
    }
}
```

### 3. DApp Browser Integration
```typescript
// DApp Browser Interface
interface DAppBrowser {
    connect(url: string): Promise<void>;
    signMessage(message: string): Promise<string>;
    signTransaction(tx: Transaction): Promise<string>;
    disconnect(): Promise<void>;
}

// Web3 Provider Implementation
class BitrielProvider {
    async request(params: JsonRpcRequest): Promise<any> {
        switch (params.method) {
            case 'eth_requestAccounts':
                return this.getAccounts();
            case 'eth_sendTransaction':
                return this.sendTransaction(params.params[0]);
            case 'eth_signMessage':
                return this.signMessage(params.params[0]);
            // Other RPC methods
        }
    }
}
```

### 4. Security Features

#### Secure Key Management
```typescript
interface KeyManager {
    generateWallet(): Promise<WalletInfo>;
    importWallet(privateKey: string): Promise<WalletInfo>;
    exportWallet(password: string): Promise<string>;
    signTransaction(tx: Transaction, password: string): Promise<string>;
}

class BitrielKeyManager implements KeyManager {
    private async encryptPrivateKey(
        privateKey: string,
        password: string
    ): Promise<string> {
        // Encryption implementation
    }

    private async decryptPrivateKey(
        encryptedKey: string,
        password: string
    ): Promise<string> {
        // Decryption implementation
    }
}
```

#### Biometric Authentication
```typescript
interface BiometricAuth {
    isBiometricAvailable(): Promise<boolean>;
    enableBiometric(password: string): Promise<void>;
    authenticateWithBiometric(): Promise<boolean>;
}

class BitrielBiometricAuth implements BiometricAuth {
    async enableBiometric(password: string): Promise<void> {
        const isAvailable = await this.isBiometricAvailable();
        if (!isAvailable) {
            throw new Error('Biometric authentication not available');
        }
        // Enable biometric auth
    }
}
```

### 5. NFT Support
```typescript
interface NFTManager {
    getNFTs(address: string): Promise<NFT[]>;
    transferNFT(params: {
        contractAddress: string;
        tokenId: string;
        to: string;
    }): Promise<TransactionResult>;
    viewNFTMetadata(
        contractAddress: string,
        tokenId: string
    ): Promise<NFTMetadata>;
}

class BitrielNFTManager implements NFTManager {
    async getNFTs(address: string): Promise<NFT[]> {
        // Fetch NFTs from multiple chains
        const selNFTs = await this.getSelNFTs(address);
        const evmNFTs = await this.getEVMNFTs(address);
        return [...selNFTs, ...evmNFTs];
    }
}
```

### 6. Cross-Chain Bridge Integration
```typescript
interface BridgeManager {
    getBridgeableAssets(): Promise<Asset[]>;
    estimateBridgeFee(params: {
        sourceChain: string;
        targetChain: string;
        asset: string;
        amount: string;
    }): Promise<string>;
    bridgeAsset(params: {
        sourceChain: string;
        targetChain: string;
        asset: string;
        amount: string;
        recipient: string;
    }): Promise<TransactionResult>;
}

class BitrielBridgeManager implements BridgeManager {
    async bridgeAsset(params: BridgeParams): Promise<TransactionResult> {
        // Verify chains
        // Check allowance
        // Execute bridge transaction
        // Monitor status
    }
}
```

### 7. DeFi Integration
```typescript
interface DeFiManager {
    getProtocols(): Promise<Protocol[]>;
    getYieldOpportunities(): Promise<YieldOpp[]>;
    stake(params: StakeParams): Promise<TransactionResult>;
    unstake(params: UnstakeParams): Promise<TransactionResult>;
    claimRewards(protocol: string): Promise<TransactionResult>;
}

class BitrielDeFiManager implements DeFiManager {
    async getYieldOpportunities(): Promise<YieldOpp[]> {
        // Fetch opportunities from SelendraDEX
        // Fetch opportunities from lending protocols
        // Calculate APYs
    }
}
```

## Mobile App Architecture

### 1. Core Components
```typescript
interface BitrielApp {
    wallet: BitrielWallet;
    security: BitrielSecurity;
    network: BitrielNetwork;
    storage: BitrielStorage;
}

class BitrielWallet {
    assets: AssetManager;
    nfts: NFTManager;
    bridge: BridgeManager;
    defi: DeFiManager;
}
```

### 2. State Management
```typescript
interface WalletState {
    accounts: Account[];
    selectedAccount: string;
    networks: Network[];
    selectedNetwork: string;
    tokens: Token[];
    transactions: Transaction[];
}

class BitrielStore {
    private state: WalletState;
    
    async updateBalance(): Promise<void> {
        // Update balances
        this.notifyListeners();
    }
}
```

### 3. Network Management
```typescript
interface NetworkManager {
    addNetwork(network: Network): Promise<void>;
    switchNetwork(networkId: string): Promise<void>;
    getGasPrice(networkId: string): Promise<string>;
}

class BitrielNetworkManager implements NetworkManager {
    async switchNetwork(networkId: string): Promise<void> {
        // Validate network
        // Update provider
        // Emit network change event
    }
}
```

## Integration Guide

### 1. DApp Integration
```javascript
// Connect to Bitriel Wallet
const bitriel = window.bitriel;

// Request account access
const accounts = await bitriel.request({
    method: 'eth_requestAccounts'
});

// Send transaction
const txHash = await bitriel.request({
    method: 'eth_sendTransaction',
    params: [{
        from: accounts[0],
        to: '0x...',
        value: '0x...',
        gas: '0x...',
        gasPrice: '0x...'
    }]
});
```

### 2. API Integration
```typescript
// Bitriel SDK
import { BitrielSDK } from '@bitriel/sdk';

const sdk = new BitrielSDK({
    environment: 'mainnet',
    apiKey: 'your-api-key'
});

// Get wallet balance
const balance = await sdk.wallet.getBalance(address);

// Send transaction
const tx = await sdk.wallet.sendTransaction({
    to: recipient,
    amount: '1.0',
    token: 'SEL'
});
```

## Security Considerations

### 1. Key Storage
- Encrypted storage using industry-standard algorithms
- Biometric authentication
- Secure enclave usage where available

### 2. Transaction Signing
- Offline signing support
- Hardware wallet integration
- Multi-signature support

### 3. Network Security
- SSL pinning
- Node redundancy
- Automatic network switching on failure

## Development Roadmap

### Q2 2025
- Enhanced DeFi integration
- Cross-chain bridge improvements
- Performance optimizations

### Q3 2025
- Hardware wallet support
- Advanced security features
- Multi-signature wallet support

### Q4 2025
- Social recovery
- Layer 2 integration
- Enhanced DApp browser

## API Reference

### Wallet Methods
```typescript
interface BitrielWalletAPI {
    // Account Management
    createAccount(): Promise<string>;
    importAccount(privateKey: string): Promise<string>;
    getAccounts(): Promise<string[]>;
    
    // Transactions
    sendTransaction(tx: Transaction): Promise<string>;
    signMessage(message: string): Promise<string>;
    signTypedData(data: TypedData): Promise<string>;
    
    // Token Management
    addToken(token: Token): Promise<void>;
    getTokens(): Promise<Token[]>;
    
    // Network Management
    addNetwork(network: Network): Promise<void>;
    switchNetwork(networkId: string): Promise<void>;
}
```
