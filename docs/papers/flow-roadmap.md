# Selendra Network Development Roadmap

This diagram visualizes Selendra's four-phase technical development roadmap as described in the whitepaper.

> **Development Priorities**: This roadmap outlines a systematic path to higher performance, but Selendra's development focus is flexible and responsive to ecosystem needs. When current TPS levels adequately serve existing use cases, development resources will shift toward cross-chain integration and business-oriented dapp development rather than pursuing raw performance metrics. This balanced approach ensures that technical development always serves the practical needs of businesses and users in Southeast Asia and beyond.

graph LR
    subgraph "Selendra Network Roadmap"
    
    Start((Current<br>Architecture)) --> Phase1
    
    subgraph "Phase 1: Performance Foundation"
        Phase1[Performance Foundation<br>Months 1-6]
        Phase1 --> |Optimize| Parallel[Parallel Transaction<br>Processing]
        Phase1 --> |Enhance| BlockProp[Optimized Block<br>Propagation]
        Phase1 --> |Improve| StateAccess[Optimized State<br>Access Patterns]
        Phase1 --> |Identify| Bottlenecks[Performance<br>Benchmarking]
    end
    
    Phase1 --> Phase2
    
    subgraph "Phase 2: Scaling Infrastructure"
        Phase2[Scaling Infrastructure<br>Months 7-12]
        Phase2 --> |Scale| Sharding[Sharded Transaction<br>Processing]
        Phase2 --> |Implement| Speculative[Speculative<br>Execution]
        Phase2 --> |Optimize| FastPath[Validator Fast<br>Consensus Paths]
        Phase2 --> |Improve| LightClient[Optimized Light<br>Client Protocols]
    end
    
    Phase2 --> Phase3
    
    subgraph "Phase 3: Decentralization and Security"
        Phase3[Decentralization & Security<br>Months 13-18]
        Phase3 --> |Adapt| DynamicVal[Dynamic Validator<br>Requirements]
        Phase3 --> |Verify| FormalVerify[Formal Verification<br>Tools]
        Phase3 --> |Enhance| InclusiveGov[Inclusive Governance<br>Mechanisms]
        Phase3 --> |Protect| ValidatorSec[Enhanced Validator<br>Security]
    end
    
    Phase3 --> Phase4
    
    subgraph "Phase 4: Privacy Technology"
        Phase4[Privacy Technology<br>Months 19-24]
        Phase4 --> |Build| ZKInfra[Zero-Knowledge<br>Infrastructure]
        Phase4 --> |Implement| ConfTx[Confidential<br>Transactions]
        Phase4 --> |Enable| PrivateContracts[Private Smart<br>Contracts]
        Phase4 --> |Balance| RegCompliance[Regulatory<br>Compliance Tools]
    end
    
    Phase4 --> Target((Target<br>Architecture))
    
    subgraph "Performance Goals"
        style TPS fill:#f9f9f9,stroke:#333,stroke-width:1px
        TPS[TPS Milestones]
        TPS --> Current[Current: 2,000-2,500 TPS]
        TPS --> Phase1TPS[After Phase 1: ~4,000 TPS]
        TPS --> Phase2TPS[After Phase 2: ~7,000 TPS]
        TPS --> Phase3TPS[After Phase 3: ~8,500 TPS]
        TPS --> Phase4TPS[Target: 10,000+ TPS]
    end
    
    end
    
    classDef phase1 fill:#d0f0c0,stroke:#2d6a4f,stroke-width:2px;
    classDef phase2 fill:#b8e0d2,stroke:#2d6a4f,stroke-width:2px;
    classDef phase3 fill:#95d5b2,stroke:#2d6a4f,stroke-width:2px;
    classDef phase4 fill:#74c69d,stroke:#2d6a4f,stroke-width:2px;
    classDef milestone fill:#d8f3dc,stroke:#2d6a4f,stroke-width:1px;
    classDef endpoint fill:#e9ecef,stroke:#212529,stroke-width:2px;
    
    class Phase1,Parallel,BlockProp,StateAccess,Bottlenecks phase1;
    class Phase2,Sharding,Speculative,FastPath,LightClient phase2;
    class Phase3,DynamicVal,FormalVerify,InclusiveGov,ValidatorSec phase3;
    class Phase4,ZKInfra,ConfTx,PrivateContracts,RegCompliance phase4;
    class Current,Phase1TPS,Phase2TPS,Phase3TPS,Phase4TPS milestone;
    class Start,Target endpoint;

## Phase Details

### Phase 1: Performance Foundation (Months 1-6)
- **Parallel Transaction Processing**: Group independent transactions for concurrent execution
- **Block Propagation Optimization**: Reduce network overhead for block distribution
- **State Access Optimization**: Redesign database patterns to reduce I/O bottlenecks
- **Performance Benchmarking**: Systematic identification of processing bottlenecks

### Phase 2: Scaling Infrastructure (Months 7-12)
- **Sharded Transaction Processing**: Partition transactions for linear scaling with hardware
- **Speculative Execution**: Pre-execute likely transactions to hide latency
- **Validator Fast Paths**: Dedicated networking for consensus messages
- **Light Client Optimization**: More efficient state proofs for resource-constrained devices

### Phase 3: Decentralization and Security (Months 13-18)
- **Dynamic Validator Requirements**: Automatically adjust stake requirements based on network growth
- **Formal Verification**: Mathematical proof of runtime safety properties
- **Inclusive Governance**: Graduated voting systems to prevent stake centralization
- **Validator Security**: Enhanced protection against attacks and compromise

### Phase 4: Privacy Technology (Months 19-24)
- **Zero-Knowledge Infrastructure**: Fundamental cryptographic primitives for privacy
- **Confidential Transactions**: Hide transaction amounts while proving validity
- **Private Smart Contracts**: Execute business logic without revealing sensitive data
- **Regulatory Compliance**: Selective disclosure capabilities for auditing requirements

## Performance Evolution
Selendra's performance will systematically increase through these development phases:
- **Current Performance**: 2,000-2,500 TPS for simple transfers
- **Phase 1 Target**: ~4,000 TPS through initial optimizations
- **Phase 2 Target**: ~7,000 TPS with advanced scaling features
- **Phase 3 Target**: ~8,500 TPS with enhanced network structure
- **Phase 4 Target**: 10,000+ TPS while maintaining privacy capabilities

Each phase builds upon previous work, with continuous optimization throughout the development cycle. This progressive approach ensures that Selendra maintains stable operations while systematically increasing performance capabilities. 