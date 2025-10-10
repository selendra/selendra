# Selendra Chain Development: Technical Tasks & Implementation Guide

**Document Version:** 1.0
**Last Updated:** October 2025
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
2. [Critical Issues & Fixes](#critical-issues--fixes)
3. [Phase 1: Foundation Tasks (Month 1-6)](#phase-1-foundation-tasks-month-1-6)
4. [Phase 2: DeFi Infrastructure (Month 6-12)](#phase-2-defi-infrastructure-month-6-12)
5. [Phase 3: Cross-Chain (Month 12-18)](#phase-3-cross-chain-month-12-18)
6. [Phase 4: Native Innovations (Month 18-24)](#phase-4-native-innovations-month-18-24)
7. [Technical Specifications](#technical-specifications)

---

## Current Technical State

### Architecture

**Consensus:**
- **Block Production:** Aura (Authority Round, 1-second slots)
- **Finality:** AlephBFT (Byzantine Fault Tolerant)
- **Block Time:** 1000ms (MILLISECS_PER_BLOCK)
- **Network Type:** Standalone Layer 1 blockchain

**Runtime:**
- **Spec Version:** 20004
- **Spec Name:** `selendra`
- **State Version:** 2
- **Base Framework:** Cardinal Cryptography's Polkadot SDK fork (aleph-v1.6.0)

**EVM Integration:**
- **Chain ID:** 1961
- **Gas Limit:** ~15M gas/block
- **Framework:** Frontier-based
- **Precompiles:** 7 (addresses 1, 2, 3, 4, 5, 1024, 1025)

### Runtime Pallets (30 Total)

**Core System (11 pallets):**
1. `frame_system` (index 0)
2. `pallet_aura` (index 1)
3. `pallet_aleph` (index 2)
4. `pallet_timestamp` (index 3)
5. `pallet_balances` (index 4)
6. `pallet_transaction_payment` (index 5)
7. `pallet_scheduler` (index 6)
8. `pallet_insecure_randomness_collective_flip` (index 7) ⚠️
9. `pallet_safe_mode` (index 100)
10. `pallet_tx_pause` (index 101)
11. `pallet_authorship` (index 10)

**Staking & Governance (7 pallets):**
12. `pallet_staking` (index 11)
13. `pallet_session::historical` (index 12)
14. `pallet_session` (index 13)
15. `pallet_elections` (index 14) - Custom DPoS
16. `pallet_committee_management` (index 15)
17. `pallet_treasury` (index 16)
18. `pallet_nomination_pools` (index 18)

**Utility (5 pallets):**
19. `pallet_utility` (index 50)
20. `pallet_multisig` (index 51)
21. `pallet_identity` (index 52)
22. `pallet_vesting` (index 53)
23. `pallet_proxy` (index 59)

**EVM Integration (4 pallets):**
24. `pallet_ethereum` (index 80)
25. `pallet_evm` (index 81)
26. `pallet_dynamic_evm_base_fee` (index 83)
27. `pallet_unified_accounts` (index 87)

**Smart Contracts (1 pallet):**
28. `pallet_contracts` (index 90)

**Administrative (2 pallets):**
29. `pallet_operations` (index 155)
30. `pallet_sudo` (index 200)

### EVM Precompiles

**Standard Ethereum Precompiles:**
1. `0x0000000000000000000000000000000000000001` - ECRecover
2. `0x0000000000000000000000000000000000000002` - Sha256
3. `0x0000000000000000000000000000000000000003` - Ripemd160
4. `0x0000000000000000000000000000000000000004` - Identity
5. `0x0000000000000000000000000000000000000005` - Modexp

**Custom Precompiles:**
6. `0x0000000000000000000000000000000000000400` (1024) - Sha3FIPS256
7. `0x0000000000000000000000000000000000000401` (1025) - ECRecoverPublicKey

---

## Critical Issues & Fixes

### P0 - Security Critical

#### TASK-001: Remove Insecure Randomness
**File:** `bin/runtime/src/lib.rs:413, 784, 1020`

**Current Issue:**
```rust
// Line 413
impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

// Line 784
type Randomness = RandomnessCollectiveFlip;

// Line 1020
RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 7,
```

**Problem:**
- `pallet_insecure_randomness_collective_flip` is validator-manipulable
- Not suitable for gambling/gaming contracts
- Marked as insecure in the pallet name itself

**Solution Options:**
1. **Moonbeam's pallet-randomness** (VRF + VDF)
2. **Chainlink VRF** (oracle-based)
3. **Disable randomness** (interim solution)

**Implementation Task:**
```rust
// Step 1: Remove from Contracts config
impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = (); // Disable temporarily
    // ... rest of config
}

// Step 2: Add dependency in Cargo.toml
[dependencies]
pallet-randomness = { git = "https://github.com/moonbeam-foundation/moonbeam", branch = "master" }

// Step 3: Configure pallet-randomness
parameter_types! {
    pub const Deposit: Balance = 1 * TOKEN;
}

impl pallet_randomness::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AddressMapping = UnifiedAccounts;
    type Currency = Balances;
    type Deposit = Deposit;
    type MaxRandomWords = ConstU8<100>;
    type MinBlockDelay = ConstU32<2>;
    type MaxBlockDelay = ConstU32<2000>;
    type BlockExpirationDelay = ConstU32<10000>;
}

// Step 4: Add to construct_runtime!
Randomness: pallet_randomness = 7,

// Step 5: Update Contracts config
type Randomness = Randomness;
```

**Acceptance Criteria:**
- [ ] Insecure randomness removed from all configs
- [ ] New randomness source integrated and tested
- [ ] Contracts can call randomness functions
- [ ] Unit tests pass
- [ ] Integration tests verify randomness quality

**Files to Modify:**
- `bin/runtime/src/lib.rs`
- `bin/runtime/Cargo.toml`

**Estimated Effort:** 2-3 weeks

---

#### TASK-002: Expand Contract Call Filter
**File:** `bin/runtime/src/lib.rs:771-779`

**Current Issue:**
```rust
pub enum ContractsCallRuntimeFilter {}

impl Contains<RuntimeCall> for ContractsCallRuntimeFilter {
    fn contains(call: &RuntimeCall) -> bool {
        matches!(
            call,
            RuntimeCall::Staking(_) | RuntimeCall::NominationPools(_)
        )
    }
}
```

**Problem:**
- Wasm contracts can ONLY call Staking and NominationPools
- Cannot transfer tokens (no Balances access)
- Cannot interact with assets
- Kills DeFi composability

**Implementation Task:**
```rust
pub enum ContractsCallRuntimeFilter {}

impl Contains<RuntimeCall> for ContractsCallRuntimeFilter {
    fn contains(call: &RuntimeCall) -> bool {
        matches!(
            call,
            // Allow balance transfers
            RuntimeCall::Balances(
                pallet_balances::Call::transfer { .. } |
                pallet_balances::Call::transfer_keep_alive { .. } |
                pallet_balances::Call::transfer_all { .. }
            ) |
            // Allow staking operations
            RuntimeCall::Staking(_) |
            RuntimeCall::NominationPools(_) |
            // Allow utility batch operations
            RuntimeCall::Utility(
                pallet_utility::Call::batch { .. } |
                pallet_utility::Call::batch_all { .. } |
                pallet_utility::Call::force_batch { .. }
            )
            // Explicitly blocked: Sudo, Treasury, Operations, Democracy
        )
    }
}
```

**Acceptance Criteria:**
- [ ] Contracts can call Balances::transfer
- [ ] Contracts can use Utility::batch
- [ ] Contracts CANNOT call Sudo
- [ ] Contracts CANNOT call Treasury
- [ ] Unit tests cover all allowed/denied calls
- [ ] Example contract demonstrates new functionality

**Files to Modify:**
- `bin/runtime/src/lib.rs` (lines 771-779)

**Test Cases:**
```rust
#[test]
fn test_contract_can_transfer_balances() {
    let call = RuntimeCall::Balances(pallet_balances::Call::transfer {
        dest: AccountId::from([1u8; 32]),
        value: 100,
    });
    assert!(ContractsCallRuntimeFilter::contains(&call));
}

#[test]
fn test_contract_cannot_call_sudo() {
    let call = RuntimeCall::Sudo(pallet_sudo::Call::sudo { /* ... */ });
    assert!(!ContractsCallRuntimeFilter::contains(&call));
}
```

**Estimated Effort:** 1 week

---

### P1 - High Priority

#### TASK-003: Fix Unbounded Storage in pallet-operations
**File:** `pallets/operations/src/lib.rs:53`

**Current Issue:**
```rust
#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
#[pallet::without_storage_info]  // ⚠️ This bypasses storage verification
pub struct Pallet<T>(_);
```

**Problem:**
- `#[pallet::without_storage_info]` disables compile-time storage bounds checking
- Potential for storage exhaustion attacks
- Fails substrate storage best practices

**Implementation Task:**
```rust
// Step 1: Remove the attribute
#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
// REMOVED: #[pallet::without_storage_info]
pub struct Pallet<T>(_);

// Step 2: If there are storage items, bound them
// Example if there's a Vec<AccountId>:
parameter_types! {
    pub const MaxAccounts: u32 = 10000;
}

#[pallet::storage]
pub type Accounts<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, MaxAccounts>,
    ValueQuery
>;
```

**Acceptance Criteria:**
- [ ] `#[pallet::without_storage_info]` removed
- [ ] All storage items have defined bounds
- [ ] Pallet compiles without warnings
- [ ] Storage migration tested (if needed)
- [ ] Runtime upgrade simulation successful

**Files to Modify:**
- `pallets/operations/src/lib.rs`

**Estimated Effort:** 1 week

---

#### TASK-004: Fix Unbounded Storage in pallet-elections
**File:** `pallets/elections/src/lib.rs:72`

**Implementation Task:**
```rust
// Find all unbounded storage items and add bounds
parameter_types! {
    pub const MaxValidators: u32 = 1000;
    pub const MaxCandidates: u32 = 100;
}

// Example fix:
#[pallet::storage]
pub type Validators<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, MaxValidators>,
    ValueQuery
>;

#[pallet::storage]
pub type Candidates<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, MaxCandidates>,
    ValueQuery
>;
```

**Acceptance Criteria:**
- [ ] Remove `#[pallet::without_storage_info]`
- [ ] All storage bounded with reasonable limits
- [ ] Election logic works with bounded storage
- [ ] Tests pass

**Files to Modify:**
- `pallets/elections/src/lib.rs`

**Estimated Effort:** 1 week

---

#### TASK-005: Fix Unbounded Storage in pallet-committee-management
**File:** `pallets/committee-management/src/lib.rs:101`

**Implementation Task:**
```rust
parameter_types! {
    pub const MaxCommitteeSize: u32 = 100;
    pub const MaxBannedValidators: u32 = 1000;
}

#[pallet::storage]
pub type Committee<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, MaxCommitteeSize>,
    ValueQuery
>;

#[pallet::storage]
pub type BannedValidators<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, MaxBannedValidators>,
    ValueQuery
>;
```

**Acceptance Criteria:**
- [ ] Remove `#[pallet::without_storage_info]`
- [ ] Committee and banned list bounded
- [ ] Committee rotation logic updated
- [ ] Tests verify bounds enforcement

**Files to Modify:**
- `pallets/committee-management/src/lib.rs`

**Estimated Effort:** 1 week

---

#### TASK-006: Fix Unbounded Storage in pallet-aleph
**File:** `pallets/aleph/src/lib.rs:78`

**Implementation Task:**
```rust
parameter_types! {
    pub const MaxAuthorities: u32 = 100;
    pub const MaxSessionValidators: u32 = 100;
}

#[pallet::storage]
pub type Authorities<T: Config> = StorageValue<
    _,
    BoundedVec<T::AuthorityId, MaxAuthorities>,
    ValueQuery
>;
```

**Acceptance Criteria:**
- [ ] Remove `#[pallet::without_storage_info]`
- [ ] Session authorities bounded
- [ ] AlephBFT finality works with bounds
- [ ] Validator set changes tested

**Files to Modify:**
- `pallets/aleph/src/lib.rs`

**Estimated Effort:** 1 week

---

#### TASK-007: Remove Sudo and Implement Governance
**File:** `bin/runtime/src/lib.rs:333-337, 1048`

**Current State:**
```rust
// Line 333
impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

// Line 1048
Sudo: pallet_sudo = 200,
```

**Phase 1: Add Council (Months 1-2)**
```rust
// Add dependency
pallet-collective = { version = "4.0.0-dev", default-features = false }

// Configure Council
parameter_types! {
    pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 7;
}

impl pallet_collective::Config<CouncilCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = CouncilMotionDuration;
    type MaxProposals = CouncilMaxProposals;
    type MaxMembers = CouncilMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = ();
}

// Add to construct_runtime!
Council: pallet_collective::<Instance1> = 16,
```

**Phase 2: Add Democracy (Months 3-4)**
```rust
// Add dependencies
pallet-democracy = { version = "4.0.0-dev", default-features = false }
pallet-conviction-voting = { version = "4.0.0-dev", default-features = false }

// Configure Democracy
parameter_types! {
    pub const LaunchPeriod: BlockNumber = 7 * DAYS;
    pub const VotingPeriod: BlockNumber = 14 * DAYS;
    pub const EnactmentPeriod: BlockNumber = 2 * DAYS;
    pub const MinimumDeposit: Balance = 100 * TOKEN;
}

impl pallet_democracy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EnactmentPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type MinimumDeposit = MinimumDeposit;
    // More config...
}

// Add to construct_runtime!
Democracy: pallet_democracy = 17,
```

**Phase 3: Remove Sudo (Month 6)**
```rust
// Remove from construct_runtime!
// Sudo: pallet_sudo = 200,  // REMOVED

// Update Treasury governance
impl pallet_treasury::Config for Runtime {
    type ApproveOrigin = EnsureRootOrHalfCouncil; // Was: EnsureSignedBy<TreasuryGovernance>
    type RejectOrigin = EnsureRootOrHalfCouncil;
    // ... rest of config
}
```

**Acceptance Criteria:**
- [ ] Council pallet added and configured
- [ ] 7 initial council members elected
- [ ] Council can approve/reject treasury proposals
- [ ] Democracy pallet integrated
- [ ] Community can submit referenda
- [ ] Voting mechanism tested on testnet
- [ ] Sudo removed from runtime
- [ ] Sudo key burned on mainnet
- [ ] All admin functions migrated to council/democracy

**Files to Modify:**
- `bin/runtime/src/lib.rs`
- `bin/runtime/Cargo.toml`

**Estimated Effort:** 6 months (phased approach)

---

## Phase 1: Foundation Tasks (Month 1-6)

### TASK-100: TypeScript SDK Development
**Repository:** Create new `selendra-sdk-ts` repo

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

**Core Modules:**

```typescript
// 1. API Module
import { ApiPromise, WsProvider } from '@polkadot/api';
import { typesBundleForPolkadot } from '@selendra/types-bundle';

export class SelendraApi {
    private api: ApiPromise;

    async connect(endpoint: string): Promise<void> {
        const provider = new WsProvider(endpoint);
        this.api = await ApiPromise.create({
            provider,
            typesBundle: typesBundleForPolkadot,
        });
    }

    async getBalance(address: string): Promise<string> {
        const account = await this.api.query.system.account(address);
        return account.data.free.toString();
    }
}

// 2. Unified Accounts Module
export class UnifiedAccountsManager {
    async linkAccounts(
        nativeAddress: string,
        evmAddress: string,
        signature: string
    ): Promise<void> {
        // Call pallet_unified_accounts::link_account
    }

    async getMappedAccount(address: string): Promise<string | null> {
        // Query account mapping
    }
}

// 3. Transaction Builder
export class TransactionBuilder {
    async transfer(
        from: string,
        to: string,
        amount: string
    ): Promise<SubmittableExtrinsic> {
        return this.api.tx.balances.transfer(to, amount);
    }

    async stake(
        validator: string,
        amount: string
    ): Promise<SubmittableExtrinsic> {
        return this.api.tx.staking.bond(validator, amount, 'Staked');
    }
}

// 4. Contract Interaction
export class ContractManager {
    async deployWasm(
        wasm: Uint8Array,
        constructor: string,
        args: any[]
    ): Promise<string> {
        // Deploy Wasm contract
    }

    async callContract(
        address: string,
        method: string,
        args: any[]
    ): Promise<any> {
        // Call contract method
    }
}
```

**Acceptance Criteria:**
- [ ] Package published to npm as `@selendra/sdk`
- [ ] Full TypeScript types from chain metadata
- [ ] Native account operations (transfer, stake, etc.)
- [ ] EVM account operations via Unified Accounts
- [ ] Wasm contract deployment and calls
- [ ] 10+ example scripts
- [ ] API documentation (TypeDoc)
- [ ] 80%+ test coverage

**Files to Create:**
- New repository: `selendra-sdk-ts/`
- Published package: `@selendra/sdk`

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

#### 1. Staking Precompile (0x0403)
```rust
// File: bin/runtime/src/evm/precompiles/staking.rs

use pallet_evm_precompile_simple::Precompile;

pub struct StakingPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for StakingPrecompile<Runtime>
where
    Runtime: pallet_staking::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let selector = handle.read_selector()?;

        match selector {
            // stake(address validator, uint256 amount)
            [0x12, 0x34, 0x56, 0x78] => {
                let validator = handle.read_address()?;
                let amount = handle.read_u256()?;
                // Call pallet_staking::bond
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: vec![],
                })
            }
            // unstake(uint256 amount)
            [0x87, 0x65, 0x43, 0x21] => {
                let amount = handle.read_u256()?;
                // Call pallet_staking::unbond
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: vec![],
                })
            }
            // claimRewards()
            [0xaa, 0xbb, 0xcc, 0xdd] => {
                // Call pallet_staking::payout_stakers
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: vec![],
                })
            }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}
```

**Solidity Interface:**
```solidity
// contracts/interfaces/IStaking.sol
interface IStaking {
    function stake(address validator, uint256 amount) external;
    function unstake(uint256 amount) external;
    function claimRewards() external;
    function getStakedAmount(address staker) external view returns (uint256);
}

// Usage
IStaking staking = IStaking(0x0000000000000000000000000000000000000403);
staking.stake(validator, 1000 ether);
```

#### 2. Governance Precompile (0x0404)
```rust
// File: bin/runtime/src/evm/precompiles/governance.rs

pub struct GovernancePrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for GovernancePrecompile<Runtime>
where
    Runtime: pallet_democracy::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let selector = handle.read_selector()?;

        match selector {
            // propose(bytes32 proposalHash, uint256 value)
            [0x11, 0x22, 0x33, 0x44] => {
                let proposal_hash = handle.read_h256()?;
                let value = handle.read_u256()?;
                // Call pallet_democracy::propose
            }
            // vote(uint256 proposalId, bool aye)
            [0x55, 0x66, 0x77, 0x88] => {
                let proposal_id = handle.read_u32()?;
                let aye = handle.read_bool()?;
                // Call pallet_democracy::vote
            }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}
```

#### 3. Unified Accounts Precompile (0x0405)
```rust
// File: bin/runtime/src/evm/precompiles/unified_accounts.rs

pub struct UnifiedAccountsPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for UnifiedAccountsPrecompile<Runtime>
where
    Runtime: pallet_unified_accounts::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let selector = handle.read_selector()?;

        match selector {
            // linkAccount(bytes32 nativeAddress, address evmAddress, bytes signature)
            [0xaa, 0xbb, 0xcc, 0xdd] => {
                let native_address = handle.read_bytes(32)?;
                let evm_address = handle.read_address()?;
                let signature = handle.read_bytes(65)?;
                // Call pallet_unified_accounts::link_account
            }
            // getLinkedAccount(address account) returns (bytes32)
            [0xee, 0xff, 0x00, 0x11] => {
                let account = handle.read_address()?;
                // Query pallet_unified_accounts for mapping
            }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}
```

**Integration:**
```rust
// Update bin/runtime/src/evm/precompiles.rs

pub fn used_addresses() -> [H160; 10] {
    [
        hash(1), hash(2), hash(3), hash(4), hash(5),    // Standard
        hash(1024), hash(1025),                          // Custom
        hash(1027), hash(1028), hash(1029),             // New
    ]
}

fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
    match handle.code_address() {
        // Existing precompiles...
        a if a == hash(1027) => Some(StakingPrecompile::<Runtime>::execute(handle)),
        a if a == hash(1028) => Some(GovernancePrecompile::<Runtime>::execute(handle)),
        a if a == hash(1029) => Some(UnifiedAccountsPrecompile::<Runtime>::execute(handle)),
        _ => None,
    }
}
```

**Acceptance Criteria:**
- [ ] 3 new precompiles implemented
- [ ] Solidity interfaces created
- [ ] Unit tests for each precompile
- [ ] Integration tests with real contracts
- [ ] Gas benchmarks completed
- [ ] Documentation with examples

**Files to Create/Modify:**
- `bin/runtime/src/evm/precompiles/staking.rs`
- `bin/runtime/src/evm/precompiles/governance.rs`
- `bin/runtime/src/evm/precompiles/unified_accounts.rs`
- `bin/runtime/src/evm/precompiles.rs` (update)
- `contracts/interfaces/*.sol` (Solidity interfaces)

**Estimated Effort:** 4 weeks

---

### TASK-102: Integration Test Suite
**Directory:** `tests/integration/`

**Test Structure:**
```rust
// tests/integration/src/lib.rs

mod tests {
    use sp_runtime::AccountId32;
    use frame_support::assert_ok;

    #[test]
    fn test_native_to_evm_transfer() {
        ExtBuilder::default().build().execute_with(|| {
            // Setup accounts
            let native_account = AccountId32::from([1u8; 32]);
            let evm_address = H160::from_low_u64_be(2);

            // Link accounts
            assert_ok!(UnifiedAccounts::link_account(
                native_account.clone(),
                evm_address,
            ));

            // Transfer from native to EVM
            assert_ok!(Balances::transfer(
                Origin::signed(native_account),
                evm_address.into(),
                1000 * TOKEN,
            ));

            // Verify balance
            assert_eq!(
                Balances::free_balance(&evm_address.into()),
                1000 * TOKEN
            );
        });
    }

    #[test]
    fn test_contract_call_runtime() {
        ExtBuilder::default().build().execute_with(|| {
            // Deploy contract
            let contract = deploy_test_contract();

            // Contract calls Balances::transfer
            let result = Contracts::call(
                contract,
                "transfer".into(),
                vec![/* args */],
            );

            assert_ok!(result);
        });
    }

    #[test]
    fn test_staking_via_precompile() {
        ExtBuilder::default().build().execute_with(|| {
            // Call staking precompile from EVM
            let result = EVM::call(
                H160::from_low_u64_be(1027), // Staking precompile
                encode_call("stake", [validator, amount]),
                /* ... */
            );

            assert_ok!(result);

            // Verify staked amount
            assert_eq!(
                Staking::ledger(&staker),
                Some(/* expected ledger */)
            );
        });
    }
}
```

**Test Categories:**

1. **Native Runtime Tests**
   - Balance transfers
   - Staking operations
   - Governance voting
   - Treasury proposals

2. **EVM Tests**
   - EVM contract deployment
   - EVM contract calls
   - Gas calculations
   - EVM events

3. **Unified Accounts Tests**
   - Account linking
   - Cross-runtime transfers
   - Permission checks

4. **Precompile Tests**
   - Each precompile function
   - Gas consumption
   - Error handling

5. **Cross-Runtime Tests**
   - Wasm contracts calling native runtime
   - EVM contracts calling precompiles
   - Native runtime calling contracts

**Acceptance Criteria:**
- [ ] 100+ integration tests
- [ ] Tests cover all critical paths
- [ ] Tests run in CI/CD
- [ ] Test documentation
- [ ] Performance benchmarks included

**Files to Create:**
- `tests/integration/Cargo.toml`
- `tests/integration/src/lib.rs`
- `tests/integration/src/evm_tests.rs`
- `tests/integration/src/unified_accounts_tests.rs`
- `tests/integration/src/precompile_tests.rs`

**Estimated Effort:** 4 weeks

---

## Phase 2: DeFi Infrastructure (Month 6-12)

### TASK-200: Chainlink Integration
**Files:** New pallet + runtime integration

**Implementation:**

```rust
// pallets/chainlink/src/lib.rs

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of oracles per feed
        #[pallet::constant]
        type MaxOracles: Get<u32>;

        /// Minimum number of oracle responses required
        #[pallet::constant]
        type MinResponses: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Price feeds: AssetId => (Price, Timestamp)
    #[pallet::storage]
    pub type PriceFeeds<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // AssetId
        (u128, T::BlockNumber), // (Price, LastUpdate)
        OptionQuery,
    >;

    /// Oracle operators for each feed
    #[pallet::storage]
    pub type Oracles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // AssetId
        BoundedVec<T::AccountId, T::MaxOracles>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PriceUpdated { asset_id: u32, price: u128, timestamp: T::BlockNumber },
        OracleAdded { asset_id: u32, oracle: T::AccountId },
        OracleRemoved { asset_id: u32, oracle: T::AccountId },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn submit_price(
            origin: OriginFor<T>,
            asset_id: u32,
            price: u128,
        ) -> DispatchResult {
            let oracle = ensure_signed(origin)?;

            // Verify oracle is authorized
            let oracles = Oracles::<T>::get(asset_id);
            ensure!(oracles.contains(&oracle), Error::<T>::UnauthorizedOracle);

            // Update price feed
            let now = frame_system::Pallet::<T>::block_number();
            PriceFeeds::<T>::insert(asset_id, (price, now));

            Self::deposit_event(Event::PriceUpdated {
                asset_id,
                price,
                timestamp: now,
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn add_oracle(
            origin: OriginFor<T>,
            asset_id: u32,
            oracle: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Oracles::<T>::try_mutate(asset_id, |oracles| {
                oracles.try_push(oracle.clone())
                    .map_err(|_| Error::<T>::TooManyOracles)?;
                Ok::<_, Error<T>>(())
            })?;

            Self::deposit_event(Event::OracleAdded { asset_id, oracle });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get latest price for an asset
        pub fn get_price(asset_id: u32) -> Option<u128> {
            PriceFeeds::<T>::get(asset_id).map(|(price, _)| price)
        }

        /// Get price with timestamp
        pub fn get_price_with_timestamp(asset_id: u32) -> Option<(u128, T::BlockNumber)> {
            PriceFeeds::<T>::get(asset_id)
        }
    }
}
```

**Oracle Precompile (0x0402):**
```rust
// bin/runtime/src/evm/precompiles/oracle.rs

pub struct OraclePrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for OraclePrecompile<Runtime>
where
    Runtime: pallet_chainlink::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let selector = handle.read_selector()?;

        match selector {
            // getPrice(uint32 assetId) returns (uint256)
            [0x41, 0x97, 0x6e, 0x09] => {
                let asset_id = handle.read_u32()?;

                let price = pallet_chainlink::Pallet::<Runtime>::get_price(asset_id)
                    .ok_or(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Price not found".into()),
                    })?;

                let output = ethabi::encode(&[ethabi::Token::Uint(price.into())]);

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output,
                })
            }
            // getTimestamp(uint32 assetId) returns (uint256)
            [0x56, 0x78, 0x9a, 0xbc] => {
                let asset_id = handle.read_u32()?;

                let (_, timestamp) = pallet_chainlink::Pallet::<Runtime>::get_price_with_timestamp(asset_id)
                    .ok_or(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Price not found".into()),
                    })?;

                let output = ethabi::encode(&[ethabi::Token::Uint(timestamp.into())]);

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output,
                })
            }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}
```

**Solidity Interface:**
```solidity
// contracts/interfaces/IOracle.sol
interface IOracle {
    function getPrice(uint32 assetId) external view returns (uint256);
    function getTimestamp(uint32 assetId) external view returns (uint256);
}

// Usage in DeFi contracts
contract LendingProtocol {
    IOracle oracle = IOracle(0x0000000000000000000000000000000000000402);

    function calculateCollateral(uint32 assetId, uint256 amount)
        public
        view
        returns (uint256)
    {
        uint256 price = oracle.getPrice(assetId);
        return (amount * price) / 1e18;
    }
}
```

**Acceptance Criteria:**
- [ ] Chainlink pallet implemented
- [ ] Oracle precompile (0x0402) functional
- [ ] 5+ price feeds active (SEL/USD, ETH/USD, BTC/USD, etc.)
- [ ] Minimum 3 oracle operators per feed
- [ ] Price updates every 10 minutes
- [ ] Solidity interface and examples
- [ ] Integration tests with DeFi contracts

**Files to Create:**
- `pallets/chainlink/src/lib.rs`
- `pallets/chainlink/Cargo.toml`
- `bin/runtime/src/evm/precompiles/oracle.rs`
- `contracts/interfaces/IOracle.sol`

**Estimated Effort:** 8 weeks

---

### TASK-201: Native DEX Pallet (Optional - Advanced)
**File:** `pallets/dex/src/lib.rs`

**Purpose:** Native Substrate DEX for better performance than EVM DEX

**Core Functionality:**
```rust
#[frame_support::pallet]
pub mod pallet {
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum liquidity for a pool
        #[pallet::constant]
        type MinimumLiquidity: Get<u128>;

        /// Trading fee (in basis points)
        #[pallet::constant]
        type TradingFee: Get<u32>;
    }

    /// Liquidity pools: (AssetA, AssetB) => Pool
    #[pallet::storage]
    pub type Pools<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::AssetId, T::AssetId),
        Pool<T::AccountId, T::Balance>,
        OptionQuery,
    >;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
    pub struct Pool<AccountId, Balance> {
        pub reserve_a: Balance,
        pub reserve_b: Balance,
        pub total_supply: Balance,
        pub lp_token_id: u32,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new liquidity pool
        #[pallet::weight(10_000)]
        pub fn create_pool(
            origin: OriginFor<T>,
            asset_a: T::AssetId,
            asset_b: T::AssetId,
            amount_a: T::Balance,
            amount_b: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure pool doesn't exist
            let pair = Self::sort_assets(asset_a, asset_b);
            ensure!(!Pools::<T>::contains_key(&pair), Error::<T>::PoolAlreadyExists);

            // Transfer assets
            T::Currency::transfer(&who, &Self::account_id(), amount_a, ExistenceRequirement::KeepAlive)?;
            T::Currency::transfer(&who, &Self::account_id(), amount_b, ExistenceRequirement::KeepAlive)?;

            // Calculate initial liquidity
            let liquidity = Self::calculate_initial_liquidity(amount_a, amount_b)?;

            // Create LP token
            let lp_token_id = Self::next_lp_token_id();

            // Store pool
            let pool = Pool {
                reserve_a: amount_a,
                reserve_b: amount_b,
                total_supply: liquidity,
                lp_token_id,
            };
            Pools::<T>::insert(pair, pool);

            Ok(())
        }

        /// Swap exact amount of asset A for asset B
        #[pallet::weight(10_000)]
        pub fn swap_exact_tokens_for_tokens(
            origin: OriginFor<T>,
            asset_in: T::AssetId,
            asset_out: T::AssetId,
            amount_in: T::Balance,
            amount_out_min: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Get pool
            let pair = Self::sort_assets(asset_in, asset_out);
            let mut pool = Pools::<T>::get(&pair)
                .ok_or(Error::<T>::PoolNotFound)?;

            // Calculate output amount (constant product formula)
            let amount_out = Self::get_amount_out(
                amount_in,
                if asset_in == pair.0 { pool.reserve_a } else { pool.reserve_b },
                if asset_in == pair.0 { pool.reserve_b } else { pool.reserve_a },
            )?;

            // Slippage check
            ensure!(amount_out >= amount_out_min, Error::<T>::InsufficientOutputAmount);

            // Transfer tokens
            T::Currency::transfer(&who, &Self::account_id(), amount_in, ExistenceRequirement::KeepAlive)?;
            T::Currency::transfer(&Self::account_id(), &who, amount_out, ExistenceRequirement::AllowDeath)?;

            // Update reserves
            if asset_in == pair.0 {
                pool.reserve_a += amount_in;
                pool.reserve_b -= amount_out;
            } else {
                pool.reserve_b += amount_in;
                pool.reserve_a -= amount_out;
            }
            Pools::<T>::insert(pair, pool);

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Calculate output amount using constant product formula
        /// amount_out = (amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)
        fn get_amount_out(
            amount_in: T::Balance,
            reserve_in: T::Balance,
            reserve_out: T::Balance,
        ) -> Result<T::Balance, DispatchError> {
            ensure!(amount_in > 0u32.into(), Error::<T>::InsufficientInputAmount);
            ensure!(reserve_in > 0u32.into() && reserve_out > 0u32.into(), Error::<T>::InsufficientLiquidity);

            let amount_in_with_fee = amount_in * 997u32.into();
            let numerator = amount_in_with_fee * reserve_out;
            let denominator = reserve_in * 1000u32.into() + amount_in_with_fee;
            let amount_out = numerator / denominator;

            Ok(amount_out)
        }

        /// Sort two assets to get consistent pair key
        fn sort_assets(a: T::AssetId, b: T::AssetId) -> (T::AssetId, T::AssetId) {
            if a < b { (a, b) } else { (b, a) }
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Native DEX pallet implemented
- [ ] Constant product AMM (Uniswap V2 formula)
- [ ] Create pool, add/remove liquidity, swap functions
- [ ] LP token minting and burning
- [ ] Trading fee collection (0.3%)
- [ ] Price oracle (TWAP)
- [ ] Integration tests
- [ ] Benchmarks show 10x cheaper gas than EVM DEX

**Files to Create:**
- `pallets/dex/src/lib.rs`
- `pallets/dex/Cargo.toml`
- `pallets/dex/src/tests.rs`
- `pallets/dex/src/benchmarking.rs`

**Estimated Effort:** 12 weeks

---

## Phase 3: Cross-Chain (Month 12-18)

### TASK-300: LayerZero Integration
**Directory:** `contracts/bridge/`

**LayerZero Endpoint Contract:**
```solidity
// contracts/bridge/SelendraEndpoint.sol
pragma solidity ^0.8.0;

import "@layerzerolabs/solidity-examples/contracts/lzApp/NonblockingLzApp.sol";

contract SelendraEndpoint is NonblockingLzApp {
    uint16 public constant ETHEREUM_CHAIN_ID = 101;

    mapping(address => uint256) public bridgedBalances;

    event TokensBridged(
        address indexed sender,
        uint16 dstChainId,
        bytes toAddress,
        uint256 amount
    );

    constructor(address _lzEndpoint) NonblockingLzApp(_lzEndpoint) {}

    /// Bridge tokens from Selendra to another chain
    function bridgeTokens(
        uint16 _dstChainId,
        bytes memory _toAddress,
        uint256 _amount
    ) public payable {
        require(_amount > 0, "Amount must be > 0");
        require(msg.value > 0, "Must send gas for LayerZero");

        // Lock tokens
        bridgedBalances[msg.sender] += _amount;

        // Encode payload
        bytes memory payload = abi.encode(msg.sender, _toAddress, _amount);

        // Send via LayerZero
        _lzSend(
            _dstChainId,
            payload,
            payable(msg.sender),
            address(0),
            bytes(""),
            msg.value
        );

        emit TokensBridged(msg.sender, _dstChainId, _toAddress, _amount);
    }

    /// Receive tokens from another chain
    function _nonblockingLzReceive(
        uint16 _srcChainId,
        bytes memory _srcAddress,
        uint64 _nonce,
        bytes memory _payload
    ) internal override {
        (address from, bytes memory toAddress, uint256 amount) =
            abi.decode(_payload, (address, bytes, uint256));

        // Convert bytes to address
        address to;
        assembly {
            to := mload(add(toAddress, 20))
        }

        // Release tokens
        bridgedBalances[from] -= amount;
        payable(to).transfer(amount);
    }

    /// Estimate LayerZero fee
    function estimateFee(
        uint16 _dstChainId,
        bytes memory _toAddress,
        uint256 _amount
    ) public view returns (uint256 nativeFee, uint256 zroFee) {
        bytes memory payload = abi.encode(msg.sender, _toAddress, _amount);
        return lzEndpoint.estimateFees(
            _dstChainId,
            address(this),
            payload,
            false,
            bytes("")
        );
    }
}
```

**Bridge UI Integration:**
```typescript
// Example bridge transaction
import { ethers } from 'ethers';

async function bridgeToEthereum(
    amount: string,
    toAddress: string
) {
    const contract = new ethers.Contract(
        SELENDRA_ENDPOINT_ADDRESS,
        SelendraEndpointABI,
        signer
    );

    // Estimate fee
    const [nativeFee] = await contract.estimateFee(
        101, // Ethereum
        ethers.utils.defaultAbiCoder.encode(['address'], [toAddress]),
        ethers.utils.parseEther(amount)
    );

    // Bridge tokens
    const tx = await contract.bridgeTokens(
        101, // Ethereum
        ethers.utils.defaultAbiCoder.encode(['address'], [toAddress]),
        ethers.utils.parseEther(amount),
        { value: nativeFee }
    );

    await tx.wait();
    console.log('Bridged successfully');
}
```

**Acceptance Criteria:**
- [ ] LayerZero endpoint deployed on Selendra
- [ ] Ethereum endpoint deployed
- [ ] SEL <-> ETH bridging functional
- [ ] ERC-20 token bridging (USDC, USDT, WBTC)
- [ ] Message passing for cross-chain calls
- [ ] Multi-sig security (2-of-3)
- [ ] 24-hour timelock for large transfers (>$1M)
- [ ] Frontend bridge UI
- [ ] Bridge monitoring dashboard
- [ ] Emergency pause mechanism

**Files to Create:**
- `contracts/bridge/SelendraEndpoint.sol`
- `contracts/bridge/test/SelendraEndpoint.test.js`
- `scripts/deploy-bridge.ts`
- `frontend/bridge-ui/` (React app)

**Estimated Effort:** 14 weeks

---

## Phase 4: Native Innovations (Month 18-24)

### TASK-400: Advanced Account Abstraction
**File:** Extension to `pallets/unified-accounts/`

**Social Recovery:**
```rust
// pallets/unified-accounts/src/recovery.rs

#[pallet::storage]
pub type Guardians<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<T::AccountId, T::MaxGuardians>,
    ValueQuery,
>;

#[pallet::storage]
pub type RecoveryConfig<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    RecoveryConfiguration<T::AccountId, T::BlockNumber>,
    OptionQuery,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct RecoveryConfiguration<AccountId, BlockNumber> {
    pub guardians: Vec<AccountId>,
    pub threshold: u32,
    pub delay_period: BlockNumber,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Setup social recovery
    #[pallet::weight(10_000)]
    pub fn setup_recovery(
        origin: OriginFor<T>,
        guardians: Vec<T::AccountId>,
        threshold: u32,
        delay_period: T::BlockNumber,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        ensure!(guardians.len() >= threshold as usize, Error::<T>::InvalidThreshold);
        ensure!(threshold > 0, Error::<T>::ZeroThreshold);

        let config = RecoveryConfiguration {
            guardians: guardians.clone(),
            threshold,
            delay_period,
        };

        RecoveryConfig::<T>::insert(&who, config);

        Ok(())
    }

    /// Initiate account recovery
    #[pallet::weight(10_000)]
    pub fn initiate_recovery(
        origin: OriginFor<T>,
        lost_account: T::AccountId,
        new_account: T::AccountId,
    ) -> DispatchResult {
        let guardian = ensure_signed(origin)?;

        let config = RecoveryConfig::<T>::get(&lost_account)
            .ok_or(Error::<T>::NoRecoveryConfig)?;

        ensure!(config.guardians.contains(&guardian), Error::<T>::NotGuardian);

        // Add guardian approval
        // If threshold reached, schedule recovery after delay

        Ok(())
    }
}
```

**Session Keys:**
```rust
// Allow temporary keys for dApp interactions

#[pallet::storage]
pub type SessionKeys<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, T::AccountId,     // Master account
    Blake2_128Concat, T::AccountId,     // Session key
    SessionKeyInfo<T::BlockNumber>,
    OptionQuery,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct SessionKeyInfo<BlockNumber> {
    pub expires_at: BlockNumber,
    pub permissions: Vec<Permission>,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Create a session key with limited permissions
    #[pallet::weight(10_000)]
    pub fn create_session_key(
        origin: OriginFor<T>,
        session_key: T::AccountId,
        duration: T::BlockNumber,
        permissions: Vec<Permission>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        let expires_at = frame_system::Pallet::<T>::block_number() + duration;

        let info = SessionKeyInfo {
            expires_at,
            permissions,
        };

        SessionKeys::<T>::insert(&who, &session_key, info);

        Ok(())
    }
}
```

**Acceptance Criteria:**
- [ ] Social recovery implemented
- [ ] Session keys with expiration
- [ ] Permission-based session keys
- [ ] EVM precompile for account abstraction (0x0406)
- [ ] Frontend integration
- [ ] Security audit
- [ ] Documentation and examples

**Files to Modify/Create:**
- `pallets/unified-accounts/src/recovery.rs`
- `pallets/unified-accounts/src/session_keys.rs`
- `bin/runtime/src/evm/precompiles/account_abstraction.rs`

**Estimated Effort:** 12 weeks

---

## Technical Specifications

### Gas and Weight Configuration

**Current Block Configuration:**
```rust
// bin/runtime/src/lib.rs

pub const MILLISECS_PER_BLOCK: u64 = 1000;
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(90);
pub const MAX_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_MILLIS.saturating_mul(400), 0);
```

**EVM Gas Configuration:**
```rust
// Target: Increase from 15M to 50M gas/block

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(50_000_000); // Increased
    pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
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
```

### Network Parameters

```rust
// Staking
pub const BondingDuration: EraIndex = 14; // 14 eras
pub const SessionsPerEra: EraIndex = 6;   // 6 sessions per era
pub const SessionPeriod: u32 = 900;        // 900 blocks = 15 minutes

// Governance (when implemented)
pub const LaunchPeriod: BlockNumber = 7 * DAYS;
pub const VotingPeriod: BlockNumber = 14 * DAYS;
pub const EnactmentPeriod: BlockNumber = 2 * DAYS;
```

### Precompile Addresses

| Address | Decimal | Precompile | Status |
|---------|---------|------------|--------|
| 0x0000...0001 | 1 | ECRecover | ✅ Active |
| 0x0000...0002 | 2 | Sha256 | ✅ Active |
| 0x0000...0003 | 3 | Ripemd160 | ✅ Active |
| 0x0000...0004 | 4 | Identity | ✅ Active |
| 0x0000...0005 | 5 | Modexp | ✅ Active |
| 0x0000...0400 | 1024 | Sha3FIPS256 | ✅ Active |
| 0x0000...0401 | 1025 | ECRecoverPublicKey | ✅ Active |
| 0x0000...0402 | 1026 | Oracle | 🔨 To Build |
| 0x0000...0403 | 1027 | Staking | 🔨 To Build |
| 0x0000...0404 | 1028 | Governance | 🔨 To Build |
| 0x0000...0405 | 1029 | UnifiedAccounts | 🔨 To Build |
| 0x0000...0406 | 1030 | AccountAbstraction | 🔨 To Build |

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

### CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Test
        run: cargo test --workspace
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Clippy
        run: cargo clippy -- -D warnings
```

---

## Priority Task List

### Immediate (Week 1-4)
1. ✅ **TASK-002**: Expand Contract Call Filter (1 week)
2. ✅ **TASK-001**: Remove Insecure Randomness (3 weeks)
3. ✅ **TASK-003**: Fix Unbounded Storage - Operations (1 week)
4. ✅ **TASK-004**: Fix Unbounded Storage - Elections (1 week)

### Short-term (Month 2-6)
5. ✅ **TASK-005**: Fix Unbounded Storage - Committee (1 week)
6. ✅ **TASK-006**: Fix Unbounded Storage - Aleph (1 week)
7. ✅ **TASK-100**: TypeScript SDK (4 weeks)
8. ✅ **TASK-101**: EVM Precompile Expansion (4 weeks)
9. ✅ **TASK-102**: Integration Test Suite (4 weeks)
10. ✅ **TASK-007**: Governance Implementation Phase 1-3 (6 months)

### Medium-term (Month 6-12)
11. ✅ **TASK-200**: Chainlink Integration (8 weeks)
12. ✅ **TASK-201**: Native DEX Pallet (12 weeks - optional)

### Long-term (Month 12-24)
13. ✅ **TASK-300**: LayerZero Bridge (14 weeks)
14. ✅ **TASK-400**: Advanced Account Abstraction (12 weeks)

---

## Getting Help

**Documentation:**
- Substrate Docs: https://docs.substrate.io
- Polkadot SDK: https://paritytech.github.io/polkadot-sdk/
- Frontier Docs: https://github.com/paritytech/frontier

**Community:**
- Substrate Stack Exchange: https://substrate.stackexchange.com
- Polkadot Discord: https://dot.li/discord

**Code Reviews:**
- All PRs require 2 approvals
- Security-critical changes require 3 approvals
- Benchmarks required for new extrinsics

---

**Last Updated:** October 2025
**Maintainers:** Selendra Core Development Team
