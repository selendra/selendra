# Selendra Chain Development: Technical Tasks & Implementation Guide

**Document Version:** 2.0
**Last Updated:** November 2025
**Current Status:** v3.0 Mainnet (launched 2025)
**Purpose:** Technical implementation guide for core developers

---

## Project Timeline

- **2019**: Project started
- **2020**: First testnet launched
- **2022**: v1 mainnet launch
- **2025**: v3 mainnet launch (current)

---

## Table of Contents

1. [Current Technical State](#current-technical-state)
2. [Completed Implementations](#completed-implementations)
3. [Critical Issues & Fixes](#critical-issues--fixes)
4. [Phase 1: Foundation Tasks (Month 1-6)](#phase-1-foundation-tasks-month-1-6)
5. [Phase 2: DeFi Infrastructure (Month 6-12)](#phase-2-defi-infrastructure-month-6-12)
6. [Phase 3: Cross-Chain (Month 12-18)](#phase-3-cross-chain-month-12-18)
7. [Phase 4: Native Innovations (Month 18-24)](#phase-4-native-innovations-month-18-24)
8. [Technical Specifications](#technical-specifications)

---

## Current Technical State

### Architecture

**Consensus:**

- **Block Production:** Aura (Authority Round, 1-second slots)
- **Finality:** AlephBFT (Byzantine Fault Tolerant)
- **Block Time:** 1000ms (MILLISECS_PER_BLOCK)
- **Network Type:** Standalone Layer 1 blockchain

**Runtime:**

- **Spec Version:** 20006
- **Spec Name:** `selendra`
- **State Version:** 2
- **Base Framework:** Cardinal Cryptography's Polkadot SDK fork (aleph-v1.6.0)

**EVM Integration:**

- **Chain ID:** 1961
- **Gas Limit:** ~15M gas/block (based on NORMAL_DISPATCH_RATIO \* WEIGHT_REF_TIME_PER_SECOND / WEIGHT_PER_GAS)
- **Framework:** Frontier-based
- **Precompiles:** 7 (addresses 1, 2, 3, 4, 5, 1024, 1025)

### Runtime Pallets (36 Total)

**Core System (11 pallets):**

1. `frame_system` (index 0)
2. `pallet_aura` (index 1)
3. `pallet_aleph` (index 2)
4. `pallet_timestamp` (index 3)
5. `pallet_balances` (index 4)
6. `pallet_transaction_payment` (index 5)
7. `pallet_scheduler` (index 6)
8. `pallet_authorship` (index 10)
9. `pallet_safe_mode` (index 100)
10. `pallet_tx_pause` (index 101)
11. `pallet_operations` (index 155)

**Staking & Validators (5 pallets):** 12. `pallet_staking` (index 11) 13. `pallet_session::historical` (index 12) 14. `pallet_session` (index 13) 15. `pallet_elections` (index 14) - Custom DPoS 16. `pallet_committee_management` (index 15) 17. `pallet_nomination_pools` (index 18)

**Governance (6 pallets):** ✅ IMPLEMENTED 18. `pallet_treasury` (index 16) 19. `pallet_collective::<Instance1>` - Council (index 30) 20. `pallet_collective::<Instance2>` - TechnicalCommittee (index 31) 21. `pallet_democracy` (index 32) 22. `pallet_elections_phragmen` - CouncilElections (index 33) 23. `pallet_preimage` (index 34)

**Utility (5 pallets):** 24. `pallet_utility` (index 50) 25. `pallet_multisig` (index 51) 26. `pallet_identity` (index 52) 27. `pallet_vesting` (index 53) 28. `pallet_proxy` (index 59)

**EVM Integration (6 pallets):** 29. `pallet_ethereum` (index 80) 30. `pallet_evm` (index 81) 31. `pallet_dynamic_evm_base_fee` (index 83) 32. `pallet_unified_accounts` (index 87) 33. `pallet_ethereum_checked` (index 88) 34. `pallet_xvm` (index 89)

**Smart Contracts (1 pallet):** 35. `pallet_contracts` (index 90)

**Administrative (1 pallet):** 36. `pallet_sudo` (index 200) - ⚠️ Still present, to be deprecated

### EVM Precompiles

**Standard Ethereum Precompiles:**

1. `0x0000000000000000000000000000000000000001` - ECRecover
2. `0x0000000000000000000000000000000000000002` - Sha256
3. `0x0000000000000000000000000000000000000003` - Ripemd160
4. `0x0000000000000000000000000000000000000004` - Identity
5. `0x0000000000000000000000000000000000000005` - Modexp

**Custom Precompiles:** 6. `0x0000000000000000000000000000000000000400` (1024) - Sha3FIPS256 7. `0x0000000000000000000000000000000000000401` (1025) - ECRecoverPublicKey

---

## Completed Implementations

### ✅ TASK-007: Governance Implementation (COMPLETED)

The governance system has been fully implemented with the following components:

**Council (pallet_collective Instance1):**

```rust
parameter_types! {
    pub const CouncilMotionDuration: SelendraBlockNumber = 3 * DAYS;
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 13;
}
```

**Technical Committee (pallet_collective Instance2):**

```rust
parameter_types! {
    pub const TechnicalMotionDuration: SelendraBlockNumber = 3 * DAYS;
    pub const TechnicalMaxProposals: u32 = 100;
    pub const TechnicalMaxMembers: u32 = 7;
}
```

**Democracy:**

```rust
parameter_types! {
    pub const LaunchPeriod: SelendraBlockNumber = 7 * DAYS;
    pub const VotingPeriod: SelendraBlockNumber = 7 * DAYS;
    pub const FastTrackVotingPeriod: SelendraBlockNumber = 3 * BLOCKS_PER_HOUR;
    pub const EnactmentPeriod: SelendraBlockNumber = 1 * DAYS;
    pub const CooloffPeriod: SelendraBlockNumber = 7 * DAYS;
    pub const MinimumDeposit: Balance = 100 * TOKEN;
    pub const MaxVotes: u32 = 100;
    pub const MaxProposals: u32 = 100;
}
```

**Council Elections (Phragmen):**

```rust
parameter_types! {
    pub const CandidacyBond: Balance = 1000 * TOKEN;
    pub const VotingBondBase: Balance = 10 * TOKEN;
    pub const VotingBondFactor: Balance = TOKEN;
    pub const TermDuration: SelendraBlockNumber = 7 * DAYS;
    pub const DesiredMembers: u32 = 13;
    pub const DesiredRunnersUp: u32 = 7;
}
```

**Governance Origins:**

```rust
pub type EnsureThreeFifthsCouncil = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>;
pub type EnsureThreeFifthsTechnicalCommittee = pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 3, 5>;
pub type EnsureUnanimousTechnicalCommittee = pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>;
```

**Pallets Updated to Use Governance Origins:**

- `pallet_treasury::Config::ApproveOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_treasury::Config::RejectOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_staking::Config::AdminOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_aleph::Config::AdminOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_elections::Config::AdminOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_committee_management::Config::AdminOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_identity::Config::ForceOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_scheduler::Config::ScheduleOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsCouncil>
- `pallet_democracy::Config::FastTrackOrigin` - EitherOfDiverse<Root, EnsureThreeFifthsTechnicalCommittee>
- `pallet_democracy::Config::InstantOrigin` - EitherOfDiverse<Root, EnsureUnanimousTechnicalCommittee>
- `pallet_tx_pause::Config::PauseOrigin` - EitherOfDiverse<Root, EnsureProportionAtLeast<2/3 Council>>

---

### ✅ TASK-001: Randomness Issue (RESOLVED)

The insecure randomness has been addressed by using a dummy implementation that prevents usage:

```rust
/// Codes using the randomness functionality cannot be uploaded. Neither can contracts
/// be instantiated from existing codes that use this deprecated functionality.
///
/// But since some `Randomness` config type is still required for `pallet-contracts`, we provide this dummy type.
pub struct DummyDeprecatedRandomness;
impl Randomness<Hash, SelendraBlockNumber> for DummyDeprecatedRandomness {
    fn random(_: &[u8]) -> (Hash, SelendraBlockNumber) {
        (Default::default(), Zero::zero())
    }
}

impl pallet_contracts::Config for Runtime {
    type Randomness = DummyDeprecatedRandomness;
    // ... rest of config
}
```

**Status:** ✅ Randomness is disabled - contracts using randomness cannot be deployed

---

### ✅ TASK-002: Contract Call Filter (RESOLVED)

The `pallet_contracts::Config::CallFilter` is now set to `()`, which allows all calls:

```rust
impl pallet_contracts::Config for Runtime {
    type CallFilter = ();  // Allows all runtime calls
    // ... rest of config
}
```

**Note:** The filter was changed from a restrictive filter to open. If you need to restrict certain calls, implement a custom filter.

---

### ✅ TASK-003 to TASK-006: Storage Bounds (RESOLVED)

All custom pallets now use proper bounded storage:

**pallet-operations (index 155):**

- No `#[pallet::without_storage_info]` attribute
- Storage version: 0

**pallet-elections (index 14):**

- No `#[pallet::without_storage_info]` attribute
- Storage version: 5
- Uses `BoundedVec<T::AccountId, T::MaxValidators>` for validator lists
- `MaxValidators` is configurable (currently 1000)

**pallet-committee-management (index 15):**

- No `#[pallet::without_storage_info]` attribute
- Storage version: 2
- Uses `BoundedVec` and `BoundedBTreeMap` for all collections
- `MaxValidators` and `MaxValidatorRewards` are bounded

**pallet-aleph (index 2):**

- No `#[pallet::without_storage_info]` attribute
- Storage version: 2
- Uses `BoundedVec<T::AuthorityId, T::MaxAuthorities>` for authorities
- Uses `BoundedVec<T::AccountId, T::MaxCommitteeSize>` for finality committee

---

## Critical Issues & Fixes

### P0 - Security Critical

#### TASK-010: Remove Sudo Pallet (PENDING)

**File:** `bin/runtime/src/lib.rs`

**Current State:**

```rust
// Sudo is still present at index 200
Sudo: pallet_sudo = 200,
```

**Problem:**

- Sudo pallet still exists even though governance is implemented
- Creates single point of failure
- Should be removed once governance is fully operational

**Solution:**

1. Ensure all administrative functions are covered by governance
2. Transfer any remaining sudo-only operations to council/democracy
3. Remove sudo pallet from runtime
4. Burn sudo key on mainnet

**Migration Plan:**

```rust
// Remove from construct_runtime!
// Sudo: pallet_sudo = 200,  // REMOVED

// Ensure TreasuryGovernance no longer includes sudo key
pub struct TreasuryGovernance;
impl SortedMembers<AccountId> for TreasuryGovernance {
    fn sorted_members() -> Vec<AccountId> {
        // Only council members, no sudo
        Council::members()
    }
}
```

**Acceptance Criteria:**

- [ ] All sudo operations migrated to governance
- [ ] Sudo pallet removed from runtime
- [ ] Runtime upgrade tested on testnet
- [ ] Mainnet sudo key burned

**Estimated Effort:** 2-4 weeks

---

### P1 - High Priority

#### TASK-011: Vendor Frontier Storage Bounds

**Files:**

- `vendors/frontier/frame/evm/src/lib.rs:124`
- `vendors/frontier/frame/ethereum/src/lib.rs:186`

**Current Issue:**

```rust
#[pallet::without_storage_info]  // In vendor code
```

**Problem:**

- Frontier pallets use unbounded storage
- This is in vendored code, not our custom pallets
- Could lead to storage bloat over time

**Solution Options:**

1. Fork and fix Frontier (significant maintenance burden)
2. Wait for upstream fix
3. Accept risk with monitoring

**Recommendation:** Monitor storage growth, wait for upstream fix unless critical

---

## Phase 1: Foundation Tasks (Month 1-6)

### TASK-100: TypeScript SDK Development

**Repository:** Create new `selendra-sdk-ts` repo

**Status:** 🔨 To Build

**Requirements:**

```typescript
// Package structure
@selendra/sdk/
├── src/
│   ├── api/           // API connection & calls
│   ├── accounts/      // Account management (native + EVM)
│   ├── contracts/     // Contract interaction (Wasm + Solidity)
│   ├── transactions/  // Transaction builders
│   ├── types/         // Generated types from metadata
│   └── utils/         // Helper functions
├── examples/
│   ├── transfer.ts
│   ├── stake.ts
│   ├── contract-call.ts
│   └── unified-accounts.ts
└── tests/
```

**Key Integration Points:**

- EVM Chain ID: 1961
- Unified Accounts for address mapping
- pallet_xvm for cross-VM calls
- pallet_ethereum_checked for validated transactions

**Acceptance Criteria:**

- [ ] Package published to npm as `@selendra/sdk`
- [ ] Full TypeScript types from chain metadata
- [ ] Native account operations (transfer, stake, etc.)
- [ ] EVM account operations via Unified Accounts
- [ ] Wasm contract deployment and calls
- [ ] 10+ example scripts
- [ ] API documentation (TypeDoc)
- [ ] 80%+ test coverage

**Estimated Effort:** 4 weeks

---

### TASK-101: EVM Precompile Expansion

**File:** `bin/runtime/src/evm/precompiles.rs`

**Current Precompiles (7):**

```rust
pub fn used_addresses() -> [H160; 7] {
    [hash(1), hash(2), hash(3), hash(4), hash(5), hash(1024), hash(1025)]
}
```

**New Precompiles to Add:**

| Address       | Decimal | Precompile      | Description              |
| ------------- | ------- | --------------- | ------------------------ |
| 0x0000...0402 | 1026    | Oracle          | Price feed access        |
| 0x0000...0403 | 1027    | Staking         | Stake/unstake from EVM   |
| 0x0000...0404 | 1028    | Governance      | Vote/propose from EVM    |
| 0x0000...0405 | 1029    | UnifiedAccounts | Account linking from EVM |

**Implementation Pattern:**

```rust
// Example: Staking Precompile
pub struct StakingPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for StakingPrecompile<Runtime>
where
    Runtime: pallet_staking::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let selector = handle.read_selector()?;
        match selector {
            // stake(address validator, uint256 amount)
            [0x12, 0x34, 0x56, 0x78] => { /* ... */ }
            // unstake(uint256 amount)
            [0x87, 0x65, 0x43, 0x21] => { /* ... */ }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}
```

**Acceptance Criteria:**

- [ ] 4 new precompiles implemented
- [ ] Solidity interfaces created
- [ ] Unit tests for each precompile
- [ ] Integration tests with real contracts
- [ ] Gas benchmarks completed
- [ ] Documentation with examples

**Estimated Effort:** 6 weeks

---

### TASK-102: Integration Test Suite

**Directory:** `tests/integration/`

**Test Categories:**

1. **Governance Tests** (Already partially implemented in `bin/runtime/src/lib.rs`)

   - Council configuration
   - Technical committee configuration
   - Democracy configuration
   - Phragmen elections configuration
   - Origin checks

2. **EVM Tests**

   - Contract deployment
   - Contract calls
   - Gas calculations
   - Events

3. **Cross-VM Tests**

   - XVM calls from Wasm to EVM
   - XVM calls from EVM to Wasm
   - Unified account operations

4. **Staking Tests**
   - Bond/unbond
   - Nominate
   - Payout rewards
   - Era transitions

**Acceptance Criteria:**

- [ ] 100+ integration tests
- [ ] Tests cover all critical paths
- [ ] Tests run in CI/CD
- [ ] Test documentation
- [ ] Performance benchmarks included

**Estimated Effort:** 4 weeks

---

## Phase 2: DeFi Infrastructure (Month 6-12)

### TASK-200: Oracle Integration

**Type:** Pallet + EVM Precompile

**Implementation Requirements:**

- Native pallet for oracle data storage
- Precompile at address 0x0402 (1026) for EVM access
- Support for multiple price feeds
- Whitelisted oracle operators
- Configurable update frequency

**Acceptance Criteria:**

- [ ] Oracle pallet implemented
- [ ] Oracle precompile functional
- [ ] 5+ price feeds active
- [ ] Integration tests with DeFi contracts

**Estimated Effort:** 8 weeks

---

### TASK-201: Native DEX Pallet (Optional)

**File:** `pallets/dex/src/lib.rs`

**Purpose:** High-performance native DEX vs EVM-based DEX

**Core Features:**

- Constant product AMM (Uniswap V2 formula)
- Create pool, add/remove liquidity, swap
- LP token minting/burning
- Trading fee (0.3%)
- TWAP oracle

**Estimated Effort:** 12 weeks

---

## Phase 3: Cross-Chain (Month 12-18)

### TASK-300: Bridge Integration

**Options:**

- LayerZero
- Axelar
- Wormhole
- Custom IBC

**Requirements:**

- Token bridging (SEL, stablecoins)
- Message passing
- Multi-sig security
- Emergency pause mechanism

**Estimated Effort:** 14 weeks

---

## Phase 4: Native Innovations (Month 18-24)

### TASK-400: Advanced Account Abstraction

**Extension to:** `pallets/unified-accounts/`

**Features:**

- Social recovery
- Session keys with permissions
- Spending limits
- Multi-sig accounts

**Estimated Effort:** 12 weeks

---

## Technical Specifications

### Gas and Weight Configuration

**Current Block Configuration:**

```rust
pub const MILLISECS_PER_BLOCK: u64 = 1000;
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(90);
pub const MAX_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_MILLIS.saturating_mul(400), 0);
```

**EVM Gas Configuration:**

```rust
/// Approximate ratio of the amount of Weight per Gas.
pub const GAS_PER_SECOND: u64 = 40_000_000;
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND.saturating_div(GAS_PER_SECOND);

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(
        NORMAL_DISPATCH_RATIO * WEIGHT_REF_TIME_PER_SECOND / WEIGHT_PER_GAS
    );
    pub ChainId: u64 = 1961;
}
```

### Storage Constants

```rust
// Contract storage deposits
pub const CONTRACT_DEPOSIT_PER_BYTE: Balance = 4 * (TOKEN / 100_000); // 0.00004 SEL

// Legacy deposits (Identity, Multisig)
pub const LEGACY_DEPOSIT_PER_BYTE: Balance = MILLI_SEL; // 0.001 SEL

// Existential deposit
pub const ExistentialDeposit: Balance = 500 * PICO_SEL; // 0.0000005 SEL

// Account mapping storage fee
pub const AccountMappingStorageFee: Balance = TOKEN / 100; // 0.01 SEL
```

### Network Parameters

```rust
// Session/Era timing (Production)
pub const DEFAULT_SESSION_PERIOD: u32 = 900;        // 900 blocks = 15 minutes
pub const DEFAULT_SESSIONS_PER_ERA: SessionIndex = 96; // 96 sessions = 24 hours

// Staking
pub const BondingDuration: EraIndex = 14; // 14 eras = 14 days
pub const SlashDeferDuration: EraIndex = 13;
pub const SessionsPerEra: EraIndex = DEFAULT_SESSIONS_PER_ERA;
pub const HistoryDepth: u32 = 84;

// Governance (Implemented)
pub const LaunchPeriod: BlockNumber = 7 * DAYS;
pub const VotingPeriod: BlockNumber = 7 * DAYS;
pub const EnactmentPeriod: BlockNumber = 1 * DAYS;
pub const CooloffPeriod: BlockNumber = 7 * DAYS;
pub const FastTrackVotingPeriod: BlockNumber = 3 * BLOCKS_PER_HOUR;

// Council
pub const CouncilMotionDuration: BlockNumber = 3 * DAYS;
pub const CouncilMaxMembers: u32 = 13;
pub const DesiredMembers: u32 = 13;
pub const DesiredRunnersUp: u32 = 7;
pub const TermDuration: BlockNumber = 7 * DAYS;

// Technical Committee
pub const TechnicalMaxMembers: u32 = 7;
```

### Precompile Addresses

| Address       | Decimal | Precompile         | Status      |
| ------------- | ------- | ------------------ | ----------- |
| 0x0000...0001 | 1       | ECRecover          | ✅ Active   |
| 0x0000...0002 | 2       | Sha256             | ✅ Active   |
| 0x0000...0003 | 3       | Ripemd160          | ✅ Active   |
| 0x0000...0004 | 4       | Identity           | ✅ Active   |
| 0x0000...0005 | 5       | Modexp             | ✅ Active   |
| 0x0000...0400 | 1024    | Sha3FIPS256        | ✅ Active   |
| 0x0000...0401 | 1025    | ECRecoverPublicKey | ✅ Active   |
| 0x0000...0402 | 1026    | Oracle             | 🔨 To Build |
| 0x0000...0403 | 1027    | Staking            | 🔨 To Build |
| 0x0000...0404 | 1028    | Governance         | 🔨 To Build |
| 0x0000...0405 | 1029    | UnifiedAccounts    | 🔨 To Build |

---

## Development Workflow

### Local Development Setup

```bash
# 1. Clone repository
git clone https://github.com/selendra/selendra
cd selendra

# 2. Install dependencies
curl https://sh.rustup.rs -sSf | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# 3. Build runtime
cargo build --release

# 4. Run local node
./target/release/selendra-node --dev --tmp

# 5. Run tests
cargo test --workspace
```

### Testing Strategy

**Unit Tests:**

```bash
# Test specific pallet
cargo test -p pallet-operations
cargo test -p pallet-elections
cargo test -p pallet-committee-management
cargo test -p pallet-aleph

# Test runtime
cargo test -p selendra-runtime
```

**Integration Tests:**

```bash
# Run integration tests
cargo test -p integration-tests
```

**Benchmarking:**

```bash
# Benchmark pallet
cargo build --release --features runtime-benchmarks
./target/release/selendra-node benchmark pallet \
    --pallet pallet_staking \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20
```

---

## Priority Task List

### Completed ✅

1. ✅ **TASK-007**: Governance Implementation (Council, TechCommittee, Democracy)
2. ✅ **TASK-001**: Remove Insecure Randomness (Disabled via DummyDeprecatedRandomness)
3. ✅ **TASK-002**: Contract Call Filter (Set to `()` - open)
4. ✅ **TASK-003**: Fix Unbounded Storage - Operations
5. ✅ **TASK-004**: Fix Unbounded Storage - Elections
6. ✅ **TASK-005**: Fix Unbounded Storage - Committee Management
7. ✅ **TASK-006**: Fix Unbounded Storage - Aleph

### Immediate (Week 1-4)

1. 🔨 **TASK-010**: Remove Sudo Pallet (once governance is stable)
2. 🔨 **TASK-100**: TypeScript SDK (4 weeks)

### Short-term (Month 2-6)

3. 🔨 **TASK-101**: EVM Precompile Expansion (6 weeks)
4. 🔨 **TASK-102**: Integration Test Suite (4 weeks)

### Medium-term (Month 6-12)

5. 🔨 **TASK-200**: Oracle Integration (8 weeks)
6. 🔨 **TASK-201**: Native DEX Pallet (12 weeks - optional)

### Long-term (Month 12-24)

7. 🔨 **TASK-300**: Bridge Integration (14 weeks)
8. 🔨 **TASK-400**: Advanced Account Abstraction (12 weeks)

---

## Getting Help

**Documentation:**

- Substrate Docs: https://docs.substrate.io
- Polkadot SDK: https://paritytech.github.io/polkadot-sdk/
- Frontier Docs: https://github.com/polkadot-evm/frontier

**Community:**

- Substrate Stack Exchange: https://substrate.stackexchange.com
- Polkadot Discord: https://dot.li/discord

**Code Reviews:**

- All PRs require 2 approvals
- Security-critical changes require 3 approvals
- Benchmarks required for new extrinsics

---

**Last Updated:** November 2025
**Maintainers:** Selendra Core Development Team
