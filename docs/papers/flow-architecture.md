# Selendra Network Technical Architecture

This diagram illustrates the current technical architecture of Selendra Network. For future enhancements and the development roadmap, please refer to the whitepaper.

graph TB
    subgraph "Selendra Network Architecture"
        
    User((User)) --> |Submits Transactions| Network
    DApp((DApp)) --> |Interacts with| Network
    
    subgraph "Network Layer"
        Network[Network Layer]
        Network --> |P2P Communication<br>30333-30343| P2P[Clique P2P Protocol]
        Network --> |Validator Consensus<br>30343-30353| VCons[Specialized Validator<br>Networking]
        Network --> |RPC Interface<br>9944-9954| RPC[RPC Services]
        Bootnode[Bootnode Discovery<br>via libp2p] --> Network
    end
    
    Network --> Consensus
    
    subgraph "Consensus Layer"
        Consensus[Consensus Layer]
        Consensus --> |Block Production<br>1-second slots| Aura[Aura]
        Consensus --> |Block Finality<br>2-3 seconds| AlephBFT[AlephBFT]
        Validator[Validator Nodes<br>50,000 SEL stake] --> Consensus
    end
    
    Consensus --> Execution
    
    subgraph "Execution Environment"
        Execution[Execution Environment]
        Execution --> |Smart Contracts| EVM[EVM Compatibility]
        Execution --> |Smart Contracts| WASM[WebAssembly Support]
        Execution --> |Core Functionality| Frame[Substrate FRAME Pallets]
        Frame --> |Custom| CustomPallets[Custom Pallets<br>- aleph<br>- committee-management<br>- elections<br>- custom-signatures<br>- dynamic-evm-base-fee]
    end
    
    Execution --> State
    
    subgraph "State Management"
        State[State Management]
        State --> |Storage Backend| RocksDB[RocksDB]
        State --> |Storage Backend| ParityDB[ParityDB]
        State --> FrontierDB[Frontier DB<br>EVM-compatible State]
        State --> |Minimum History| Blocks[901+ Recent Blocks]
    end
    
    subgraph "Runtime Framework"
        Runtime[Runtime Framework]
        Runtime --> |Upgradeable Without Hardforks| OnChainGov[On-Chain Governance]
        Runtime --> |Economic Model| TokenEcon[21M SEL Annual Inflation]
        Runtime --> |Current Fee Model| CurrentFees[Current Fee Distribution<br>80% Burn<br>20% Validators]
        Runtime --> |Planned Fee Model| PlannedFees[Planned Fee Distribution<br>50% Burn<br>30% Validators<br>20% Treasury]
    end
    
    State -.-> Runtime
    Runtime -.-> Execution
    
    subgraph "Performance Metrics"
        Perf[Current Performance]
        Perf --> |Simple Transfers| TPS1[2,000-2,500 TPS]
        Perf --> |Basic Contracts| TPS2[1,000-1,300 TPS]
        Perf --> |Complex Contracts| TPS3[400-600 TPS]
        Perf --> |Target| Future[Path to 10,000+ TPS]
    end
    
    end
    
    classDef network fill:#d2f5ff,stroke:#0077b6,stroke-width:2px;
    classDef consensus fill:#ffe8d6,stroke:#bc6c25,stroke-width:2px;
    classDef execution fill:#d8f3dc,stroke:#2d6a4f,stroke-width:2px;
    classDef state fill:#f8edeb,stroke:#9d0208,stroke-width:2px;
    classDef runtime fill:#e2eafc,stroke:#3a0ca3,stroke-width:2px;
    classDef perf fill:#fff1e6,stroke:#a5a58d,stroke-width:2px;
    
    class Network,P2P,VCons,RPC,Bootnode network;
    class Consensus,Aura,AlephBFT,Validator consensus;
    class Execution,EVM,WASM,Frame,CustomPallets execution;
    class State,RocksDB,ParityDB,FrontierDB,Blocks state;
    class Runtime,OnChainGov,TokenEcon,CurrentFees,PlannedFees runtime;
    class Perf,TPS1,TPS2,TPS3,Future perf;

## Architecture Components

### 1. Consensus Layer
- **Hybrid Approach**: Combines Aura for block production with AlephBFT for deterministic finality
- **Block Time**: 1-second intervals for block production
- **Finality**: Transactions reach finality within 2-3 seconds
- **Validator Requirements**: Minimum 50,000 SEL tokens stake

### 2. Execution Environment
- **Smart Contracts**: Supports both EVM and WebAssembly
- **EVM Compatibility**: Complete compatibility with Ethereum tools and contracts
- **Core Functionality**: Implemented through Substrate FRAME pallets
- **Custom Pallets**: Specialized pallets for consensus, governance, and economics

### 3. State Management
- **Storage Backends**: Uses either RocksDB or ParityDB
- **State Retention**: Maintains at least 901 most recent blocks
- **EVM State**: Separate Frontier database for Ethereum-compatible state
- **Pruning**: Older state data is pruned for efficiency

### 4. Network Layer
- **P2P Protocol**: Uses Clique for node communication
- **Specialized Networking**: Optimized for validator consensus messages
- **Block Propagation**: Optimized to reduce network overhead
- **Port Assignments**: Dedicated port ranges for different functions

### 5. Runtime Framework
- **Modular Design**: Built from interconnected Substrate pallets
- **Seamless Upgrades**: Runtime updates through on-chain governance without hard forks
- **Economic Model**: 21 million SEL tokens distributed annually
- **Fee Structure**: Current implementation - 80% burned, 20% to validators; Planned model - 50% burned, 30% to validators, 20% to treasury

## Performance Characteristics
- **Simple Transfers**: 2,000-2,500 TPS
- **Basic Smart Contracts**: 1,000-1,300 TPS
- **Complex Contracts**: 400-600 TPS
- **Target**: Path to 10,000+ TPS through optimizations 

> **Note on Development Focus**: While Selendra has a technical pathway to significantly higher TPS, raw performance metrics are not the sole development priority. When current TPS levels satisfy present use cases, resources will be balanced between performance optimization, cross-chain connectivity, and dapp development to serve businesses and users. This ensures that technical development remains aligned with actual ecosystem needs.

*Note: This diagram represents the current architecture of Selendra Network. For detailed information about future enhancements, scalability plans, and the four-phase development roadmap, please refer to the Selendra whitepaper.* 