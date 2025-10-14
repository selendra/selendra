# Requirements Document

## Introduction

This specification outlines the development requirements for Selendra v4.0, a major upgrade that addresses critical security issues, enhances developer experience, and introduces advanced DeFi infrastructure. The upgrade focuses on removing security vulnerabilities, implementing proper governance, expanding EVM capabilities, and creating the most developer-friendly blockchain protocol.

Based on the current v3.0 mainnet state, this version will transform Selendra from a functional blockchain into a production-ready, developer-centric platform that can compete with leading Layer 1 solutions while maintaining unique advantages in unified account architecture and native-EVM integration.

## Requirements

### Requirement 1: Security Critical Fixes

**User Story:** As a blockchain user and developer, I want all critical security vulnerabilities resolved so that my funds and contracts are safe from exploitation.

#### Acceptance Criteria

1. WHEN the runtime is upgraded THEN the insecure randomness pallet SHALL be completely removed and replaced with a secure VRF-based solution
2. WHEN smart contracts request randomness THEN the system SHALL provide cryptographically secure, non-manipulable random values
3. WHEN contracts attempt to call runtime functions THEN the system SHALL allow safe operations (transfers, staking) while blocking administrative functions (sudo, treasury)
4. WHEN storage operations are performed THEN all pallets SHALL have bounded storage to prevent unbounded growth attacks
5. IF a pallet uses storage collections THEN it SHALL define maximum limits for all Vec, BTreeMap, and similar unbounded types
6. WHEN the runtime compiles THEN no pallet SHALL use the `#[pallet::without_storage_info]` attribute

### Requirement 2: Governance Implementation

**User Story:** As a network stakeholder, I want decentralized governance to replace sudo access so that the network is truly community-controlled.

#### Acceptance Criteria

1. WHEN the governance system is deployed THEN a council of 7 elected members SHALL be established with multi-signature capabilities
2. WHEN community members want to propose changes THEN they SHALL be able to submit referenda through a democratic process
3. WHEN treasury proposals are submitted THEN the council SHALL have authority to approve or reject spending
4. WHEN the governance transition is complete THEN the sudo pallet SHALL be completely removed from the runtime
5. WHEN administrative functions are needed THEN they SHALL only be executable through council approval or democratic referendum
6. IF emergency situations arise THEN the safe mode pallet SHALL provide temporary network protection without centralized control

### Requirement 3: Enhanced Developer Experience

**User Story:** As a developer, I want comprehensive SDK and runtime capabilities so that I can build on Selendra with powerful tools and clear error handling.

#### Acceptance Criteria

1. WHEN smart contract errors occur THEN the runtime SHALL provide helpful error messages with specific error codes and context
2. WHEN developers use the SDK THEN they SHALL get comprehensive TypeScript support with auto-generated types
3. WHEN setting up local development THEN a single command SHALL start a complete development node with proper configuration
4. WHEN runtime errors occur THEN they SHALL include detailed context and suggested fixes in the SDK
5. IF developers need runtime integration THEN the SDK SHALL provide seamless native and EVM account operations
6. WHEN precompiles are used THEN they SHALL have comprehensive Solidity interfaces with proper gas estimation

### Requirement 4: EVM Precompile Expansion

**User Story:** As a Solidity developer, I want native access to Substrate functionality through precompiles so that I can build advanced dApps without learning new languages.

#### Acceptance Criteria

1. WHEN EVM contracts need staking functionality THEN they SHALL access it through a dedicated staking precompile at address 0x0403
2. WHEN contracts need governance interaction THEN they SHALL use the governance precompile at address 0x0404 for proposals and voting
3. WHEN unified account management is needed THEN contracts SHALL use the unified accounts precompile at address 0x0405
4. WHEN price data is required THEN contracts SHALL access oracle feeds through the oracle precompile at address 0x0402
5. IF advanced account features are needed THEN the account abstraction precompile at 0x0406 SHALL provide social recovery and session keys
6. WHEN precompiles are called THEN they SHALL have comprehensive Solidity interfaces and gas-optimized implementations

### Requirement 5: TypeScript SDK Development

**User Story:** As a frontend developer, I want a comprehensive TypeScript SDK so that I can easily integrate Selendra functionality into web applications.

#### Acceptance Criteria

1. WHEN connecting to Selendra THEN the SDK SHALL provide simple API connection with automatic type generation from chain metadata
2. WHEN managing accounts THEN the SDK SHALL support both native Substrate and EVM account operations seamlessly
3. WHEN deploying contracts THEN the SDK SHALL support both Wasm and Solidity contract deployment and interaction
4. WHEN building transactions THEN the SDK SHALL provide type-safe transaction builders for all common operations
5. IF unified accounts are used THEN the SDK SHALL handle account linking and cross-runtime operations transparently
6. WHEN managing DeFi THEN the SDK SHALL provide managers for DEX, staking, unified accounts, and cross-chain operations
7. WHEN handling three stablecoins THEN the SDK SHALL seamlessly work with STAR ($0.01), sUSD ($1.00), and KHRt (1 KHR)
8. WHEN errors occur THEN the SDK SHALL provide detailed error information with suggested fixes and relevant documentation links

### Requirement 6: DeFi Infrastructure

**User Story:** As a DeFi developer, I want reliable price oracles and DEX infrastructure so that I can build sophisticated financial applications.

#### Acceptance Criteria

1. WHEN DeFi protocols need price data THEN Chainlink oracle integration SHALL provide 5+ major price feeds (SEL/USD, ETH/USD, BTC/USD, USDC/USD, USDT/USD)
2. WHEN price updates occur THEN they SHALL happen at least every 10 minutes with minimum 3 oracle operators per feed
3. WHEN high-performance trading is needed THEN a native Substrate DEX pallet SHALL provide 10x cheaper gas costs than EVM alternatives
4. WHEN liquidity operations are performed THEN the DEX SHALL support pool creation, liquidity provision, and token swaps with 0.3% trading fees
5. WHEN supporting stablecoins THEN the system SHALL handle three distinct tokens: STAR (sports), sUSD (bridge), KHRt (local currency)
6. WHEN sUSD is used THEN it SHALL operate as simple 1:1 wrapped USDT/USDC (not over-collateralized) for capital efficiency
7. IF price manipulation is attempted THEN the oracle system SHALL use multiple data sources and aggregation to ensure accuracy
8. WHEN DeFi contracts integrate oracles THEN they SHALL access price data through standardized precompile interfaces

### Requirement 7: Cross-Chain Integration

**User Story:** As a multi-chain developer, I want robust cross-chain infrastructure so that I can build applications that work across multiple blockchains.

#### Acceptance Criteria

1. WHEN bridging to Ethereum THEN LayerZero integration SHALL enable secure SEL <-> ETH transfers
2. WHEN major tokens are bridged THEN support SHALL include USDC, USDT, and WBTC with proper token standards
3. WHEN large transfers occur THEN amounts over $1M SHALL have 24-hour timelocks for additional security
4. WHEN bridge operations happen THEN a 2-of-3 multi-signature system SHALL secure all cross-chain transactions
5. IF bridge issues arise THEN an emergency pause mechanism SHALL protect user funds
6. WHEN developers integrate bridges THEN they SHALL access cross-chain functionality through precompiles and SDK

### Requirement 8: Advanced Account Features

**User Story:** As a user concerned about security and usability, I want advanced account features so that I can recover lost accounts and use temporary permissions safely.

#### Acceptance Criteria

1. WHEN setting up account recovery THEN users SHALL be able to designate trusted guardians with configurable thresholds
2. WHEN account recovery is needed THEN guardians SHALL initiate recovery with appropriate delay periods for security
3. WHEN temporary access is required THEN session keys SHALL provide time-limited permissions for specific operations
4. WHEN dApp interactions occur THEN session keys SHALL allow seamless user experience without repeated transaction signing
5. IF account abstraction features are used THEN they SHALL be accessible through both native Substrate and EVM interfaces
6. WHEN recovery or session operations happen THEN they SHALL be auditable and include comprehensive security logging

### Requirement 9: Performance and Scalability

**User Story:** As a high-volume application developer, I want improved network performance so that my dApps can handle significant transaction throughput.

#### Acceptance Criteria

1. WHEN block production occurs THEN the network SHALL maintain 1-second block times with AlephBFT finality
2. WHEN EVM transactions are processed THEN the gas limit SHALL be increased from 15M to 50M gas per block
3. WHEN storage operations happen THEN all pallets SHALL use bounded collections to prevent state bloat
4. WHEN benchmarking is performed THEN new extrinsics SHALL have proper weight calculations for accurate fee estimation
5. IF network congestion occurs THEN the dynamic base fee mechanism SHALL adjust gas prices appropriately
6. WHEN measuring performance THEN the network SHALL support at least 1000 TPS for simple transfers

### Requirement 10: Integration Testing and Quality Assurance

**User Story:** As a network operator, I want comprehensive testing coverage so that upgrades are deployed safely without breaking existing functionality.

#### Acceptance Criteria

1. WHEN new features are developed THEN they SHALL have 100+ integration tests covering all critical paths
2. WHEN cross-runtime functionality is tested THEN tests SHALL verify native-to-EVM, EVM-to-native, and precompile interactions
3. WHEN runtime upgrades occur THEN migration tests SHALL ensure smooth transitions without data loss
4. WHEN performance testing happens THEN benchmarks SHALL verify gas costs and transaction throughput meet targets
5. IF breaking changes are introduced THEN comprehensive migration guides SHALL be provided for developers
6. WHEN CI/CD runs THEN all tests SHALL pass before any code can be merged to main branches