# Implementation Plan - Focused on Critical Path to Market

## Phase 1: Security Critical Fixes (Weeks 1-8)

- [ ] 1. Remove Insecure Randomness and Implement Secure Alternative
  - Replace `pallet_insecure_randomness_collective_flip` with Moonbeam's `pallet-randomness`
  - Add VRF+VDF based randomness generation with proper configuration
  - Update contracts pallet to use new secure randomness source
  - Write comprehensive tests for randomness quality and non-manipulability
  - _Requirements: 1.1, 1.2_

- [ ] 2. Expand Contract Runtime Call Filter
  - Modify `ContractsCallRuntimeFilter` to allow `Balances::transfer` operations
  - Add support for `Utility::batch` operations from contracts
  - Maintain security by explicitly blocking `Sudo`, `Treasury`, and `Operations` calls
  - Create unit tests covering all allowed and denied runtime calls
  - Deploy example contract demonstrating new transfer capabilities
  - _Requirements: 1.3_

- [ ] 3. Fix Unbounded Storage in Core Pallets
  - Remove `#[pallet::without_storage_info]` from `pallet-operations`
  - Add bounded storage with `MaxAccounts: u32 = 10000` parameter
  - Implement storage migration for existing unbounded data
  - Apply same fixes to `pallet-elections`, `pallet-committee-management`, and `pallet-aleph`
  - Run storage migration tests on testnet before mainnet deployment
  - _Requirements: 1.4, 1.5, 1.6_

## Phase 2: Governance Implementation (Weeks 9-20)

- [ ] 4. Implement Council Governance System
  - Add `pallet-collective` dependency to runtime
  - Configure 7-member council with motion duration and voting thresholds
  - Create council election mechanism through existing elections pallet
  - Implement council powers for treasury approval and runtime upgrades
  - Test council proposal creation, voting, and execution processes
  - _Requirements: 2.1, 2.2, 2.3_

- [ ] 5. Integrate Democracy Pallet
  - Add `pallet-democracy` and `pallet-conviction-voting` dependencies
  - Configure referendum parameters (launch, voting, enactment periods)
  - Implement public proposal submission with minimum deposit requirements
  - Create voting mechanisms with conviction-based weighting
  - Test end-to-end referendum process from proposal to execution
  - _Requirements: 2.2, 2.4_

- [ ] 6. Remove Sudo and Complete Governance Transition
  - Update all administrative origins to use council or democracy approval
  - Migrate treasury governance from sudo to council control
  - Remove sudo pallet from runtime completely
  - Implement emergency safe mode controls without centralized access
  - Execute sudo key burning ceremony on mainnet
  - _Requirements: 2.4, 2.5, 2.6_

## Phase 3: Stablecoin Infrastructure (Weeks 21-32)

- [ ] 7. Implement sUSD Bridge Token
  - Create `pallet-bridge-token` for 1:1 USDT/USDC backing
  - Implement mint/burn functions with bridge operator controls
  - Add reserve management with automated backing verification
  - Create bridge monitoring with real-time reserve ratio tracking
  - Write comprehensive tests for mint/burn operations and reserve management
  - _Requirements: 6.1, 6.2_

- [ ] 8. Build KHRt Stablecoin Foundation
  - Create `pallet-khrt` with Cambodian Riel backing mechanism
  - Implement banking API integration for ABA, ACLEDA, WING deposits
  - Add KYC/AML compliance with user verification workflows
  - Create reserve management with 110% over-collateralization
  - Build automated reserve monitoring and alerting system
  - _Requirements: 6.3, 6.4, 6.5_

- [ ] 9. Implement KHRt Mint/Burn Operations
  - Create secure mint function with bank deposit verification
  - Implement burn function with automatic bank transfer initiation
  - Add daily/monthly limits with progressive KYC requirements
  - Create emergency pause mechanism with governance controls
  - Write comprehensive tests for all mint/burn scenarios
  - _Requirements: 6.3, 6.4, 6.5_

- [ ] 10. Add Stablecoin Precompiles
  - Create `StablecoinPrecompile` at address `0x0407` for EVM access
  - Implement mint/burn functions accessible from Solidity contracts
  - Add balance and reserve checking functions
  - Create comprehensive Solidity interface documentation
  - Write integration tests for EVM stablecoin operations
  - _Requirements: 6.6_

## Phase 4: Core DeFi Infrastructure (Weeks 33-44)

- [ ] 11. Build Native DEX Pallet
  - Implement `pallet-dex` with constant product AMM (Uniswap V2 formula)
  - Add pool creation, liquidity provision, and token swap functionality
  - Implement LP token minting and burning with proper accounting
  - Add 0.3% trading fee collection and TWAP price oracle
  - Benchmark DEX operations to achieve 10x gas savings vs EVM alternatives
  - _Requirements: 6.3, 6.4_

- [ ] 12. Implement Staking Precompile
  - Create `StakingPrecompile` at address `0x0403` with bond, unbond, and claim functions
  - Implement Solidity interface `IStaking.sol` with comprehensive function signatures
  - Add gas benchmarking for all staking operations through precompile
  - Write integration tests verifying EVM contracts can stake and claim rewards
  - Create example Solidity contract demonstrating staking integration
  - _Requirements: 4.1_

- [ ] 13. Create DEX Precompile
  - Build `DEXPrecompile` at address `0x0408` for EVM contract access to native DEX
  - Implement swap, add liquidity, and remove liquidity functions
  - Add price quote and pool information query functions
  - Create comprehensive Solidity interfaces for all DEX operations
  - Write integration tests for DEX precompile interactions
  - _Requirements: 6.6_

- [ ] 14. Build Core TypeScript SDK
  - Create new repository `selendra-sdk-ts` with proper TypeScript configuration
  - Implement `ApiManager` for WebSocket connections and metadata handling
  - Build `StablecoinManager` for sUSD and KHRt operations
  - Add `DEXManager` for trading and liquidity operations
  - Publish initial SDK package to npm as `@selendra/sdk`
  - _Requirements: 5.1, 5.2, 5.3_

## Phase 5: Cross-Chain Bridge Infrastructure (Weeks 45-56)

- [ ] 15. Implement LayerZero Bridge Infrastructure
  - Deploy LayerZero endpoint contract on Selendra with proper configuration
  - Implement Ethereum endpoint for SEL <-> ETH bridging
  - Add support for major ERC-20 tokens (USDC, USDT, WBTC)
  - Create 2-of-3 multi-signature security system for bridge operations
  - Implement 24-hour timelock for transfers over $1M value
  - _Requirements: 7.1, 7.2, 7.4, 7.5_

- [ ] 16. Build Bridge Monitoring and Security Infrastructure
  - Implement bridge monitoring pallet with health checks and metrics collection
  - Create emergency pause mechanism with proper access controls and governance integration
  - Add bridge operation logging and audit trail functionality
  - Build automated security checks and anomaly detection for bridge operations
  - Test bridge operations with comprehensive security scenarios and stress testing
  - _Requirements: 7.4, 7.5_

## Phase 6: Testing and Quality Assurance (Weeks 57-64)

- [ ] 17. Build Comprehensive Integration Test Suite
  - Create 100+ integration tests covering all critical functionality paths
  - Implement cross-runtime testing (native-to-EVM, EVM-to-native, precompiles)
  - Add runtime upgrade testing scenarios with migration validation
  - Build automated testing pipeline with comprehensive coverage reporting
  - Test all upgrade scenarios and migration paths
  - _Requirements: 10.1, 10.2_

- [ ] 18. Conduct Security Audits and Penetration Testing
  - Perform comprehensive security audit of all new runtime components
  - Conduct penetration testing of bridge and stablecoin systems
  - Test randomness quality and manipulation resistance
  - Verify access controls and permission systems in runtime
  - Document security findings and implement fixes
  - _Requirements: 10.3_

## Phase 7: Final Integration and Deployment (Weeks 65-72)

- [ ] 19. Create Developer Migration Documentation
  - Write comprehensive migration guides for developers upgrading from v3
  - Create breaking change documentation with workarounds
  - Build SDK migration guides with code examples
  - Add troubleshooting guides for common runtime integration issues
  - Create technical documentation for all new precompiles and pallets
  - _Requirements: 10.5_

- [ ] 20. Final Integration and Deployment Preparation
  - Conduct final integration testing on testnet with full feature set
  - Perform load testing to verify performance targets
  - Create deployment runbooks for mainnet upgrade
  - Prepare rollback procedures for emergency scenarios
  - Coordinate community testing and feedback collection
  - _Requirements: 10.4, 10.6_

## Success Metrics - Focused on Market Readiness

**Security & Stability:**
- Zero critical vulnerabilities in security audit
- All storage bounded with appropriate limits
- Secure randomness passes cryptographic analysis
- KHRt maintains 110%+ reserve ratio at all times

**Stablecoin Infrastructure:**
- sUSD achieves 1:1 peg with USDT/USDC
- KHRt maintains stable 1:1 peg with Cambodian Riel
- Bridge processes $100K+ monthly volume without issues
- Banking integration handles 1000+ KHRt mint/burn operations monthly

**DeFi Readiness:**
- Native DEX achieves 10x gas savings vs EVM alternatives
- DEX handles $100K+ monthly trading volume
- All precompiles gas-optimized and benchmarked
- SDK enables seamless stablecoin and DEX integration

**Developer Experience:**
- SDK provides comprehensive TypeScript support for stablecoins and DEX
- All precompiles have complete Solidity interfaces
- 95%+ test coverage across all runtime components
- Clear migration path from v3 to v4 documented

**Timeline Achievement:**
- Phase 1-2 complete by Q1 2025 (governance + security)
- Phase 3-4 complete by Q2 2025 (stablecoins + DEX)
- Phase 5-6 complete by Q3 2025 (bridge + testing)
- Full deployment ready by Q4 2025