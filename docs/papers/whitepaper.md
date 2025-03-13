# Selendra Network: Architecture, Design, and Technical Roadmap

## 1. Introduction

Selendra Network is an EVM-compatible Layer 1 blockchain built on Substrate with AlephBFT consensus and a Proof-of-Stake security model. Selendra provides fast 1-second block times with deterministic finality within 2-3 seconds. The platform aims to deliver 5,000 TPS in optimal conditions, with a path to 10,000+ TPS through future optimizations. Selendra focuses on serving enterprise and individual users across Southeast Asia, with initial emphasis on Cambodia.

## 2. Technical Architecture

### 2.1 Core Components

Selendra's architecture consists of five integrated components that form a complete blockchain system.

The Consensus Layer uses a hybrid approach combining Aura for block production with AlephBFT for deterministic finality. Aura creates blocks in 1-second slots, while AlephBFT committees of validators finalize these blocks. This combination provides both predictable block production and fast transaction finality.

The Execution Environment supports both EVM and WebAssembly smart contracts. The EVM implementation provides complete compatibility with Ethereum tools and contracts. Substrate FRAME pallets implement core blockchain functionality including staking, governance, and treasury management.

State Management stores blockchain data using either RocksDB or ParityDB backends. The system maintains the most recent 901 blocks at minimum while pruning older state data for efficiency. A separate Frontier database maintains Ethereum-compatible state for EVM contracts.

The Network Layer uses the Clique peer-to-peer protocol for node communication. It implements specialized networking for validators to minimize consensus message latency. Block propagation optimization reduces network overhead when sharing new blocks across the network.

The Runtime Framework provides a modular system built from Substrate pallets. This architecture allows seamless runtime upgrades through on-chain governance without hard forks. The economic model distributes 21 million SEL tokens annually as inflation to validators and the treasury.

### 2.2 System Parameters

Selendra operates with precisely defined system parameters that determine its performance and security characteristics. Each block is produced at 1-second intervals and can contain up to 5 MB of data. Transaction processing consumes approximately 400ms of each block's time budget. Validators must stake at least 50,000 SEL tokens to participate in consensus.

### 2.3 Consensus Mechanism

Selendra uses a hybrid consensus mechanism that combines speed with security. Aura assigns validators to 1-second slots for block production in a deterministic rotation. AlephBFT provides fast finality by having validator committees reach consensus on block finalization. The system remains secure as long as less than one-third of validators are malicious. Validator performance tracking ensures that consistently underperforming validators are removed from the active set.

### 2.4 Technical Dependencies

Selendra builds upon established blockchain frameworks with specific version requirements. The core blockchain functionality is based on a customized fork of Polkadot SDK (branch "selendra-1.6.0"). Ethereum compatibility is implemented through a modified version of Frontier (branch "selendra-1.6.0"). The AlephBFT consensus integration leverages multiple component libraries (versions 0.9 through 0.14). For advanced cryptography needs, the system incorporates halo2_proofs for zero-knowledge capabilities.

### 2.5 Node Types and Networking

Selendra operates with two primary node types: validator nodes and RPC nodes. The networking architecture assigns specific port ranges to different functions: P2P communication (30333-30343), validator consensus (30343-30353), and RPC interfaces (9944-9954). A bootnode mechanism facilitates network discovery using libp2p. Each node maintains its own keystore with session keys for consensus participation and network identity.

### 2.6 Fee Structure and Economics

Selendra implements a deflationary fee model that balances network security with token value preservation. Transaction fees follow a three-way distribution mechanism. Fifty percent of all transaction fees are burned, permanently removing them from circulation. Thirty percent of fees are distributed to validators as additional rewards beyond block rewards. The remaining twenty percent flows to the on-chain treasury for funding ecosystem development and governance decisions.

Fee calculation combines a base fee with a dynamic component that adjusts to network demand. The base fee represents the minimum cost for transaction inclusion and scales with computational complexity. The dynamic component uses a congestion-based algorithm that increases during high network utilization and decreases during periods of low activity. This mechanism prevents spam attacks while ensuring reasonable fees during normal operation.

For EVM transactions, gas pricing follows a dynamic fee model with Selendra-specific parameters. The network defines minimum, default, and maximum base fee parameters: 10 Gwei default base fee (10,000,000,000 Wei), 80 Gwei minimum (80,000,000,000 Wei), and 8,000 Gwei maximum (8,000,000,000,000 Wei).

The base fee calculation uses the formula:
`base_fee_per_gas = adjustment_factor * weight_factor * 25 / 98974`

Where:
- The adjustment factor relates to block fullness and changes between blocks
- The weight factor converts consumed weight to fee (set at 10,000,000,000,000)
- The result is clamped between the minimum and maximum values
- A step limit ratio (93/1,000,000) prevents large fee changes between consecutive blocks

This model allows for gas prices to adjust based on network demand while preventing extreme fluctuations. When network utilization increases, the adjustment factor grows, causing the base fee to rise. During periods of lower activity, the base fee decreases accordingly, creating an economically efficient fee market.

*Note on current implementation: As of the current release, the fee distribution mechanism uses a two-way split with 80% of transaction fees burned and 20% distributed to block authors. Tips follow a different distribution with 20% burned and 80% to block authors. The three-way distribution described earlier (50% burn, 30% to validators, 20% to treasury) represents the planned model that will be implemented in an upcoming network upgrade. This change will better align with Selendra's long-term economic vision by adding direct treasury funding while maintaining strong deflationary pressure through fee burning.*

Native Substrate transactions use a weight-based fee calculation. Each operation has an assigned weight based on its computational complexity, storage requirements, and state impact. The weight is multiplied by a configurable coefficient to determine the final fee amount. This system ensures that complex operations requiring more resources also pay proportionally higher fees.

## 3. Current Development Status

### 3.1 Completed Features

Selendra has implemented several critical components that form the foundation of the network. The core consensus mechanism integrates AlephBFT with Substrate and achieves 1-second block times. EVM compatibility provides full support for Ethereum contracts and tools including all standard precompiles. The economic model implements a fixed inflation rate with funds distributed to validators and the treasury. Multiple node types support different use cases from lightweight access to full validation.

### 3.2 Current Performance
Selendra currently processes approximately 2,000-2,500 TPS for simple token transfers. Smart contracts operate at 1,000-1,300 TPS for basic operations and 400-600 TPS for complex contracts. Transactions reach finality within 2-3 seconds, providing fast confirmation for users and applications.

### 3.3 Ongoing Development

Several features remain under active development to enhance Selendra's capabilities. Transaction throughput scaling aims to increase performance through parallel processing and network optimizations. Privacy infrastructure will enable confidential transactions and private smart contracts. Cross-chain connectivity focuses on bridges to Ethereum and other networks to enable seamless asset transfers.

### 3.4 Custom Pallets and Components

Selendra extends Substrate's functionality through several custom pallets developed specifically for its architecture. The aleph pallet manages integration with the AlephBFT consensus protocol. Committee-management implements validator selection and rotation mechanisms. The elections pallet provides on-chain validator selection logic. Custom-signatures enables alternative signature schemes beyond standard ed25519 and sr25519. Dynamic-evm-base-fee implements adaptive gas pricing for the EVM environment based on network demand.

## 4. Technical Challenges

### 4.1 Performance Constraints

Selendra faces several inherent challenges that affect its performance potential. Consensus messaging creates overhead as validators communicate to reach agreement on block finality. State access during transaction processing creates bottlenecks, especially for contracts that interact with large data sets. Sequential transaction execution limits throughput when processing interdependent operations.

### 4.2 Network Limitations

The network layer imposes certain constraints on system performance. Large blocks require more time to propagate across geographically distributed validators. Network latency varies based on validator locations and internet infrastructure quality. Global network partitions can temporarily affect block finality until connectivity is restored.

### 4.3 Security Considerations

Security remains a constant focus with several areas requiring attention. Validator distribution both geographically and by stake concentration affects system decentralization. Smart contracts inherit security challenges from the EVM model, requiring careful auditing and verification. Advanced privacy features will add complexity to the security model as they are implemented.

## 5. Technical Roadmap

> **Note on Development Priorities**: While this roadmap outlines a path to achieving higher transaction throughput, Selendra recognizes that raw TPS isn't the sole measure of network value. When current performance levels adequately serve present use cases, development resources will be balanced between performance optimization, cross-chain connectivity, and supporting business-oriented dapps. This flexible approach ensures that technical development always aligns with actual ecosystem needs rather than pursuing performance metrics for their own sake.

### 5.1 Scaling to 10,000+ TPS

Selendra will implement parallel transaction processing to significantly increase throughput. Independent transactions will be grouped and executed simultaneously across multiple CPU cores. Block structure will evolve to support transaction sharding within blocks for better parallelization. State access patterns will be optimized through caching, prefetching, and data layout enhancements. Consensus message exchange will become more efficient through pipelining and committee size optimization. Network propagation will improve through erasure coding and regional peer grouping.

### 5.2 Enhancing Decentralization

The validator system will evolve to encourage broader participation. Stake requirements will adjust dynamically based on network growth and token value. Light client protocols will become more efficient to support validation on resource-constrained devices. Governance will implement more inclusive voting methods to prevent concentration of decision-making power.

### 5.3 Strengthening Security

Formal verification will ensure critical system components meet their security requirements. Validators will benefit from improved hardware security through remote attestation and tamper detection. Smart contract security will improve through specialized analysis tools and standard security modules. Security audits will become part of the standard release process for all major updates.

### 5.4 Implementing Transaction Privacy

Selendra will build a comprehensive privacy infrastructure starting with fundamental zero-knowledge primitives. Confidential transactions will hide transfer amounts while proving their validity. Private smart contracts will enable business logic to execute without revealing sensitive data. Regulatory compliance features will allow selective disclosure for auditing while maintaining privacy for normal operations.

### 5.5 Build and Deployment Infrastructure

Selendra's development workflow emphasizes reproducibility and optimization. The build system uses Docker-based compilation with the paritytech/ci-linux image for consistency across environments. Release builds enable Link Time Optimization (LTO) and single codegen units to maximize runtime performance. Automated test environments support local development with configurable validator counts and network parameters. This infrastructure enables rapid iteration while maintaining stability for production deployments.

## 6. Implementation Timeline

### 6.1 Phase 1: Performance Foundation

The first six months will focus on performance enhancements to the existing architecture. Parallel transaction processing will increase throughput for independent operations. Block propagation will become more efficient through network optimizations. State access patterns will be redesigned to reduce database bottlenecks. Performance benchmarking will identify and eliminate remaining bottlenecks.

### 6.2 Phase 2: Scaling Infrastructure

The next six months will implement advanced scaling features to reach higher transaction throughput. Sharded transaction processing will enable linear scaling with hardware resources. Speculative execution will hide latency during transaction processing. Validator networks will gain dedicated fast paths for consensus messages. Light clients will become more efficient through optimized state proofs.

### 6.3 Phase 3: Decentralization and Security

The following six months will strengthen the foundation for long-term network health. Dynamic validator requirements will adapt to network growth and token value. Formal verification tools will ensure runtime safety during upgrades. Governance mechanisms will become more inclusive through graduated voting rights. Security features will better protect validators from attacks.

### 6.4 Phase 4: Privacy Technology

The final phase will introduce comprehensive privacy features into the platform. Zero-knowledge infrastructure will provide the foundation for all privacy operations. Confidential transaction support will protect financial data from public exposure. Private smart contracts will enable confidential business logic execution. Regulatory compliance tools will balance privacy with legal requirements.

## 7. Conclusion

Selendra Network provides a high-performance EVM-compatible blockchain with fast finality through AlephBFT consensus. Current performance of 2,000-2,500 TPS for simple transfers will increase to 10,000+ TPS through the implementation of parallel processing, state optimizations, and consensus enhancements. The platform will evolve to improve decentralization and security while adding privacy features for enterprise use cases.

The phased development approach ensures stable network operation throughout this evolution. Each phase builds logically on previous work, creating a clear path from current capabilities to future goals. This structured approach allows Selendra to serve Southeast Asian enterprises and individuals with increasingly sophisticated blockchain applications while maintaining the performance, security, and usability they require.
