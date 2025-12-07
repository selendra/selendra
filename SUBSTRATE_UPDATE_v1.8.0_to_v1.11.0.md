# Selendra Substrate Update Plan
## Polkadot SDK: v1.8.0 → v1.11.0 + Frontier: polkadot-v1.8.0 → polkadot-v1.11.0

**Date:** December 7, 2025
**Target:** Major version upgrade for Selendra blockchain
**Current Version:** Polkadot SDK v1.8.0, Frontier polkadot-v1.8.0
**Target Version:** Polkadot SDK v1.11.0, Frontier polkadot-v1.11.0

**Repositories:**
- Polkadot SDK: https://github.com/paritytech/polkadot-sdk
  - Current: `release-polkadot-v1.8.0`
  - Target: `release-polkadot-v1.11.0`
- Frontier: https://github.com/polkadot-evm/frontier
  - Current: `polkadot-v1.8.0`
  - Target: `polkadot-v1.11.0`

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Pre-Upgrade Preparation](#pre-upgrade-preparation)
3. [Polkadot SDK Updates](#polkadot-sdk-updates)
4. [Frontier Updates](#frontier-updates)
5. [Runtime Migrations](#runtime-migrations)
6. [Breaking Changes](#breaking-changes)
7. [Upgrade Steps](#upgrade-steps)
8. [Testing Strategy](#testing-strategy)
9. [Rollback Plan](#rollback-plan)
10. [Post-Upgrade Verification](#post-upgrade-verification)

---

## Executive Summary

This document outlines the comprehensive upgrade plan for Selendra from Polkadot SDK v1.8.0 to v1.11.0, including the corresponding Frontier upgrade from polkadot-v1.8.0 to polkadot-v1.11.0.

### Key Highlights

**Version Jump:** v1.8.0 → v1.11.0 (3 major versions)

**Estimated Changes:**
- Polkadot SDK: ~500+ commits across multiple releases
- Frontier: ~100+ commits with EVM improvements
- Runtime migrations: Multiple required (detailed below)
- API changes: Moderate breaking changes expected

**Critical Points:**
- ⚠️ **MULTIPLE RUNTIME MIGRATIONS REQUIRED**
- ⚠️ **Frontier MUST be updated in sync with Polkadot SDK**
- ⚠️ **Test on local testnet before mainnet deployment**
- ⚠️ **Backup all chain data before upgrade**

### Migration Status Overview

| Component | Migration Required | Type | Complexity |
|-----------|-------------------|------|------------|
| Pallet Staking | ✅ Yes | Multi-version | High |
| Pallet Nomination Pools | ✅ Yes | Storage migration | Medium |
| Pallet Balances | ✅ Yes | Storage update | Low |
| Pallet Democracy | ⚠️ Deprecated | Replaced by Gov2 | High |
| Pallet Collective | ⚠️ Check | Config changes | Medium |
| Pallet Contracts | ❌ No | API changes only | Low |
| Frontier EVM | ✅ Yes | Config + Storage | Medium |
| Pallet Ethereum | ✅ Yes | Schema changes | Medium |
| XCM Integration | ⚠️ Check | Version update | Medium |

---

## Pre-Upgrade Preparation

### 1. Backup Strategy

```bash
# Backup chain database
cd /home/msi/Project/selendra
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz run-nodes-local/*/chains/

# Backup runtime state
# Export current runtime state snapshot
./target/release/selendra export-state --chain=local > state-backup-v1.8.0.json

# Git backup
git checkout -b backup-v1.8.0-$(date +%Y%m%d)
git push origin backup-v1.8.0-$(date +%Y%m%d)
```

### 2. Environment Preparation

```bash
# Update Rust toolchain
rustup update stable
rustup update nightly

# Check current versions
rustc --version
cargo --version

# Clean build artifacts
cargo clean
```

### 3. Dependency Audit

Check all custom pallets compatibility:
- `pallet-aleph`
- `pallet-committee-management`
- `pallet-elections`
- `pallet-operations`
- `pallet-dynamic-evm-base-fee`
- `pallet-unified-accounts`
- `pallet-ethereum-checked`
- `pallet-xvm`

---

## Polkadot SDK Updates

### Version History

#### v1.9.0 Highlights
- **Frame System**: Improved transaction payment mechanisms
- **Pallet Staking**: Enhanced validator selection logic
- **XCM v3**: Complete XCM v3 implementation
- **Weights v2**: New weight calculation system
- **Migration**: Staking pallet storage migration required

#### v1.10.0 Highlights
- **Gov2 (OpenGov)**: Full Governance v2 implementation
- **Pallet Democracy**: Deprecated, migrate to Referenda + Conviction Voting
- **Scheduler v4**: Enhanced task scheduling
- **Asset Hub**: Improved asset management
- **Migration**: Democracy → Gov2 migration path

#### v1.11.0 Highlights
- **Async Backing**: Improved parachain block production
- **Elastic Scaling**: Dynamic parachain scalability
- **BEEFY**: Enhanced bridge security
- **Storage Improvements**: Optimized state trie
- **Runtime Upgrades**: Improved migration framework

### Key Breaking Changes

#### 1. Transaction Payment API Changes

**Before (v1.8.0):**
```rust
impl pallet_transaction_payment::Config for Runtime {
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}
```

**After (v1.11.0):**
```rust
impl pallet_transaction_payment::Config for Runtime {
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    // Or use more advanced multiplier
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
}
```

#### 2. Weight System V2

**Before (v1.8.0):**
```rust
#[pallet::weight(10_000)]
pub fn example_call(origin: OriginFor<T>) -> DispatchResult {
    // ...
}
```

**After (v1.11.0):**
```rust
#[pallet::call_index(0)]
#[pallet::weight(Weight::from_parts(10_000, 0))]
pub fn example_call(origin: OriginFor<T>) -> DispatchResult {
    // ...
}
```

#### 3. Staking Changes

Enhanced validator selection and nomination pool improvements:

```rust
impl pallet_staking::Config for Runtime {
    // New field in v1.10+
    type VoterList = VoterList;
    type TargetList = pallet_staking::UseValidatorsMap<Self>;

    // Updated in v1.11
    type MaxNominations = MaxNominations;
    type MaxExposurePageSize = ConstU32<256>;
}
```

---

## Frontier Updates

### Version History

#### polkadot-v1.9.0 Highlights
- **EVM Precompiles**: Enhanced precompile support
- **Gas Metering**: Improved gas calculation accuracy
- **RPC Improvements**: Better Ethereum RPC compatibility
- **Storage Optimization**: Reduced EVM storage overhead

#### polkadot-v1.10.0 Highlights
- **EIP-1559**: Full support for dynamic base fee
- **Block Building**: Optimized Ethereum block construction
- **Trace API**: Enhanced transaction tracing
- **Filter Improvements**: Better event filtering

#### polkadot-v1.11.0 Highlights
- **Shanghai Support**: EIP-3855, EIP-3860 implementation
- **Performance**: 20% improvement in EVM execution
- **RPC V2**: New JSON-RPC specifications
- **Contract Migration**: Storage layout improvements

### Key Changes

#### 1. EVM Config Updates

**Updated Config (v1.11.0):**
```rust
impl pallet_evm::Config for Runtime {
    type FeeCalculator = DynamicEvmBaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;

    // New in v1.11
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type OnChargeTransaction = EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
    type OnCreate = ();

    // Updated precompiles
    type PrecompilesType = Precompiles<Self>;
    type PrecompilesValue = PrecompilesValue;
}
```

#### 2. Base Fee Mechanism

**Before (v1.8.0):**
```rust
type FeeCalculator = BaseFee;
```

**After (v1.11.0):**
```rust
// Use dynamic base fee with EIP-1559 support
type FeeCalculator = DynamicEvmBaseFee;

impl pallet_dynamic_evm_base_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
    type MinBaseFeePerGas = MinBaseFeePerGas;
    type MaxBaseFeePerGas = MaxBaseFeePerGas;
    type StepLimitRatio = StepLimitRatio;
}
```

#### 3. Ethereum Block Schema

Migration required for Ethereum block storage:

```rust
// Runtime migration code
pub mod frontier_migration {
    use frame_support::traits::OnRuntimeUpgrade;

    pub struct MigrateEthereumSchema<T>(sp_std::marker::PhantomData<T>);

    impl<T: pallet_ethereum::Config> OnRuntimeUpgrade for MigrateEthereumSchema<T> {
        fn on_runtime_upgrade() -> Weight {
            // Migration logic for Ethereum schema v2
            pallet_ethereum::migrations::v2::migrate::<T>()
        }
    }
}
```

#### 4. RPC Updates

**New RPC Endpoints (v1.11.0):**
- `eth_maxPriorityFeePerGas` - EIP-1559 support
- `eth_feeHistory` - Historical fee data
- `trace_block` - Enhanced block tracing
- `debug_traceCall` - Call simulation

**Updated in node/src/rpc.rs:**
```rust
use fc_rpc::{
    EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride,
    SchemaV1Override, SchemaV2Override, SchemaV3Override, StorageOverride,
};

// Use SchemaV3Override for v1.11.0
let overrides = Arc::new(OverrideHandle {
    schemas: sp_api::StateBackendFor::<B, C>::default(),
    fallback: Box::new(SchemaV3Override::new(client.clone())),
});
```

---

## Runtime Migrations

### Migration Order

Runtime migrations must be executed in the following order during the first block after upgrade:

```rust
// In bin/runtime/src/lib.rs

pub type Migrations = (
    // 1. Frame System migrations
    frame_support::migrations::RemovePallet<OldDemocracyPalletName, RocksDbWeight>,

    // 2. Pallet-specific migrations
    pallet_nomination_pools::migration::v7::MigrateToV7<Runtime>,
    pallet_staking::migrations::v14::MigrateToV14<Runtime>,
    pallet_balances::migration::MigrateManyToTrackInactive<Runtime, CheckAccounts>,

    // 3. Frontier migrations
    pallet_ethereum::migrations::v2::Migration<Runtime>,

    // 4. Custom pallet migrations
    pallet_committee_management::migration::MigrateToV2<Runtime>,

    // 5. Cleanup
    frame_support::migrations::RemoveStorage<RemovedStoragePrefix, Runtime>,
);

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations, // Apply migrations
>;
```

### Detailed Migration Requirements

#### 1. Pallet Staking Migration

**Version:** v13 → v14 (Multi-step migration through v1.9, v1.10, v1.11)

**Changes:**
- Exposures pagination
- Validator prefs storage layout
- Nomination pool integration

**Implementation:**
```rust
pub struct MigrateStaking;

impl OnRuntimeUpgrade for MigrateStaking {
    fn on_runtime_upgrade() -> Weight {
        let mut weight = Weight::zero();

        // Check version
        let onchain_version = Pallet::<T>::on_chain_storage_version();
        if onchain_version < 14 {
            // Migrate exposures to paged exposures
            weight += pallet_staking::migrations::v14::migrate::<Runtime>();

            // Update version
            StorageVersion::new(14).put::<Pallet<T>>();

            log::info!("✅ Staking migrated to v14");
        }

        weight
    }
}
```

**Estimated Time:** ~5-10 minutes on mainnet (depends on validator count)

#### 2. Nomination Pools Migration

**Version:** v6 → v7

**Changes:**
- Commission support
- Delegation improvements
- Reward calculation updates

**Implementation:**
```rust
// Automatic via pallet
pallet_nomination_pools::migration::v7::MigrateToV7<Runtime>

// Manual check:
#[cfg(feature = "try-runtime")]
fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
    let pool_count = pallet_nomination_pools::BondedPools::<Runtime>::iter().count();
    log::info!("Migrating {} nomination pools", pool_count);
    Ok(pool_count.encode())
}
```

**Estimated Time:** ~2-5 minutes

#### 3. Balances Migration

**Changes:**
- Tracking inactive balances
- Hold/freeze separation
- Multi-asset support preparation

**Implementation:**
```rust
pallet_balances::migration::MigrateManyToTrackInactive<
    Runtime,
    CheckAccounts,
>

// Define accounts to check
pub struct CheckAccounts;
impl frame_support::traits::Get<Vec<AccountId>> for CheckAccounts {
    fn get() -> Vec<AccountId> {
        // Add critical accounts to verify
        vec![
            // Treasury
            // Staking reward pot
            // etc.
        ]
    }
}
```

**Estimated Time:** ~30 seconds

#### 4. Democracy → Gov2 Migration

**Action Required:** Manual migration needed if using democracy

**Steps:**
1. Export all active proposals and referenda
2. Disable democracy pallet
3. Initialize Gov2 pallets (Referenda, ConvictionVoting)
4. Optionally migrate active proposals

**Implementation:**
```rust
// 1. Add to runtime
construct_runtime!(
    pub struct Runtime {
        // Remove: Democracy
        // Add:
        ConvictionVoting: pallet_conviction_voting,
        Referenda: pallet_referenda,
        Whitelist: pallet_whitelist,
        FellowshipCollective: pallet_ranked_collective::<Instance1>,
    }
);

// 2. Configure new pallets
impl pallet_referenda::Config for Runtime {
    type WeightInfo = pallet_referenda::weights::SubstrateWeight<Runtime>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Currency = Balances;
    // ... additional config
}
```

**Estimated Time:** Requires governance vote + 1-2 days preparation

#### 5. Frontier EVM Migration

**Version:** Schema v2 → Schema v3

**Changes:**
- Block hash mapping optimization
- Transaction index improvements
- Receipt storage layout

**Implementation:**
```rust
pub struct FrontierSchemaV3Migration;

impl OnRuntimeUpgrade for FrontierSchemaV3Migration {
    fn on_runtime_upgrade() -> Weight {
        log::info!("🔧 Migrating Frontier to Schema V3...");

        // This migration is mostly handled by fc-db
        // Ensure RPC node is configured for Schema V3

        // Update pallet-ethereum if needed
        let weight = pallet_ethereum::migrations::v2::migrate::<Runtime>();

        log::info!("✅ Frontier migration complete");
        weight
    }
}
```

**Estimated Time:** ~5 minutes (database migration may continue in background)

#### 6. Custom Pallet Migrations

**Committee Management Migration:**

```rust
// In pallets/committee-management/src/migration.rs
pub mod v2 {
    use super::*;
    use frame_support::traits::OnRuntimeUpgrade;

    pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version < 2 {
                // Perform migration
                // Update committee structure for v1.11 compatibility

                StorageVersion::new(2).put::<Pallet<T>>();
                T::DbWeight::get().reads_writes(1, 1)
            } else {
                T::DbWeight::get().reads(1)
            }
        }
    }
}
```

### Migration Testing

**Test Script:**
```bash
#!/bin/bash
# test-migrations.sh

echo "🧪 Testing runtime migrations..."

# Build with try-runtime
cargo build --release --features try-runtime

# Run try-runtime checks
./target/release/selendra try-runtime \
    --runtime ./target/release/wbuild/selendra-runtime/selendra_runtime.wasm \
    on-runtime-upgrade \
    --checks all \
    live --uri ws://localhost:9944

echo "✅ Migration tests complete"
```

---

## Breaking Changes

### API Breaking Changes

#### 1. Runtime API Changes

**Removed:**
- `DemocracyApi` - Use `ReferendaApi` + `ConvictionVotingApi`
- Old staking exposure API - Use paginated version

**Added:**
- `NominationPoolsApi::pending_rewards_paged()`
- `StakingApi::eras_stakers_page_count()`

**Modified:**
```rust
// Before
sp_api::decl_runtime_apis! {
    pub trait AccountNonceApi {
        fn account_nonce(account: AccountId) -> Index;
    }
}

// After
sp_api::decl_runtime_apis! {
    pub trait AccountNonceApi {
        fn account_nonce(account: AccountId) -> Nonce;
    }
}
```

#### 2. RPC Changes

**Ethereum RPC:**
- `eth_gasPrice` - Now returns EIP-1559 base fee
- `eth_getBlockByNumber` - Includes baseFeePerGas
- New: `eth_feeHistory`, `eth_maxPriorityFeePerGas`

**Substrate RPC:**
- `state_getMetadata` - Updated metadata version
- `system_chain` - Enhanced chain info

#### 3. Client API Changes

**Before (v1.8.0):**
```rust
let client = sc_service::new_full_parts::<Block, Runtime, _>(
    &config,
    telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
)?;
```

**After (v1.11.0):**
```rust
let client = sc_service::new_full_parts::<Block, Runtime, _>(
    &config,
    telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
    executor,
)?;
```

### Configuration Changes

#### Cargo.toml Updates

**Required dependency version updates:**

```toml
[workspace.dependencies]
# Updated versions for v1.11.0
parity-scale-codec = { version = "3.6.9" }  # Was 3.6.1
scale-info = { version = "2.11.1" }         # Was 2.11.0

# Additional dependencies for Gov2
pallet-referenda = { path = "vendors/polkadot-sdk/substrate/frame/referenda", default-features = false }
pallet-conviction-voting = { path = "vendors/polkadot-sdk/substrate/frame/conviction-voting", default-features = false }
pallet-whitelist = { path = "vendors/polkadot-sdk/substrate/frame/whitelist", default-features = false }
pallet-ranked-collective = { path = "vendors/polkadot-sdk/substrate/frame/ranked-collective", default-features = false }
```

---

## Upgrade Steps

### Phase 1: Vendor Updates

#### Step 1.1: Update Polkadot SDK

```bash
cd /home/msi/Project/selendra/vendors/polkadot-sdk

# Fetch latest updates
git fetch origin

# Create backup branch
git checkout -b backup-v1.8.0

# Switch to target version
git checkout release-polkadot-v1.11.0

# Or use specific tag
# git checkout polkadot-v1.11.3  # Use latest stable patch version
```

#### Step 1.2: Update Frontier

```bash
cd /home/msi/Project/selendra/vendors/frontier

# Fetch latest updates
git fetch origin

# Create backup
git checkout -b backup-v1.8.0

# Switch to target version
git checkout polkadot-v1.11.0

# Verify compatibility
git log --oneline -20
```

#### Step 1.3: Verify Compatibility

```bash
# Check for breaking changes
cd /home/msi/Project/selendra

# Look for API changes
git diff backup-v1.8.0..release-polkadot-v1.11.0 \
    vendors/polkadot-sdk/substrate/frame/*/src/lib.rs | less

# Check Frontier changes
git diff backup-v1.8.0..polkadot-v1.11.0 \
    vendors/frontier/frame/*/src/lib.rs | less
```

### Phase 2: Update Cargo Dependencies

#### Step 2.1: Update Root Cargo.toml

```bash
# The workspace dependencies are already using path-based references
# They will automatically use the new versions
# Just verify no version conflicts exist

cd /home/msi/Project/selendra
cargo tree --duplicates
```

#### Step 2.2: Add Gov2 Dependencies (if migrating from Democracy)

```toml
# In Cargo.toml [workspace.dependencies]
pallet-referenda = { path = "vendors/polkadot-sdk/substrate/frame/referenda", default-features = false }
pallet-conviction-voting = { path = "vendors/polkadot-sdk/substrate/frame/conviction-voting", default-features = false }
pallet-whitelist = { path = "vendors/polkadot-sdk/substrate/frame/whitelist", default-features = false }
pallet-ranked-collective = { path = "vendors/polkadot-sdk/substrate/frame/ranked-collective", default-features = false }
```

### Phase 3: Update Runtime Configuration

#### Step 3.1: Update Runtime Cargo.toml

```toml
# In bin/runtime/Cargo.toml

[dependencies]
# Add new dependencies if using Gov2
pallet-referenda = { workspace = true }
pallet-conviction-voting = { workspace = true }
pallet-whitelist = { workspace = true }
pallet-ranked-collective = { workspace = true }

# Keep existing dependencies
# pallet-democracy will be phased out
```

#### Step 3.2: Update Runtime Configuration

**File:** `bin/runtime/src/lib.rs`

```rust
// Update spec version
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("selendra"),
    impl_name: create_runtime_str!("selendra-node"),
    authoring_version: 1,
    spec_version: 211,  // Increment for v1.11 upgrade
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 2,
    state_version: 1,
};

// Update construct_runtime macro
construct_runtime!(
    pub struct Runtime {
        // System
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,

        // Governance v2 (New)
        ConvictionVoting: pallet_conviction_voting,
        Referenda: pallet_referenda,
        Whitelist: pallet_whitelist,
        FellowshipCollective: pallet_ranked_collective::<Instance1>,

        // Existing pallets
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        // ... rest of pallets

        // Frontier
        Ethereum: pallet_ethereum,
        EVM: pallet_evm,
        DynamicEvmBaseFee: pallet_dynamic_evm_base_fee,
        // ... rest of frontier pallets
    }
);
```

#### Step 3.3: Configure New Pallets

```rust
// Configure Referenda
parameter_types! {
    pub const SubmitOrigin: PalletsOrigin = PalletsOrigin::system(frame_system::RawOrigin::Signed);
}

impl pallet_referenda::Config for Runtime {
    type WeightInfo = pallet_referenda::weights::SubstrateWeight<Runtime>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Currency = Balances;
    type SubmitOrigin = frame_system::EnsureSigned<AccountId>;
    type CancelOrigin = frame_system::EnsureRoot<AccountId>;
    type KillOrigin = frame_system::EnsureRoot<AccountId>;
    type Slash = Treasury;
    type Votes = pallet_conviction_voting::VotesOf<Runtime>;
    type Tally = pallet_conviction_voting::TallyOf<Runtime>;
    type SubmissionDeposit = SubmissionDeposit;
    type MaxQueued = ConstU32<100>;
    type UndecidingTimeout = UndecidingTimeout;
    type AlarmInterval = AlarmInterval;
    type Tracks = TracksInfo;
    type Preimages = Preimage;
}

// Configure ConvictionVoting
impl pallet_conviction_voting::Config for Runtime {
    type WeightInfo = pallet_conviction_voting::weights::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type VoteLockingPeriod = VoteLockingPeriod;
    type MaxVotes = ConstU32<512>;
    type MaxTurnout = frame_support::traits::TotalIssuanceOf<Balances, Self::AccountId>;
    type Polls = Referenda;
}
```

#### Step 3.4: Update EVM Configuration

```rust
// Update pallet_evm configuration
impl pallet_evm::Config for Runtime {
    type FeeCalculator = DynamicEvmBaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
    type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
    type AddressMapping = pallet_unified_accounts::EvmAddressMapping<Runtime>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = Precompiles<Self>;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = EVMChainId;
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, DealWithFees<Runtime>>;
    type OnCreate = ();
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type SuicideQuickClearLimit = ConstU32<0>;
    type Timestamp = Timestamp;
    type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}
```

#### Step 3.5: Add Runtime Migrations

```rust
// Add migration tuple
pub type Migrations = (
    // Staking migration
    pallet_staking::migrations::v14::MigrateToV14<Runtime>,

    // Nomination pools migration
    pallet_nomination_pools::migration::v7::MigrateToV7<Runtime>,

    // Balances migration
    pallet_balances::migration::MigrateManyToTrackInactive<Runtime, CheckAccounts>,

    // Frontier migrations
    pallet_ethereum::migrations::v2::Migration<Runtime>,

    // Custom migrations
    pallet_committee_management::migration::v2::MigrateToV2<Runtime>,
);

// Update Executive
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;
```

### Phase 4: Update Node Implementation

#### Step 4.1: Update RPC Configuration

**File:** `bin/node/src/rpc.rs`

```rust
use fc_rpc::{
    EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride,
    SchemaV3Override, StorageOverride,
};

// Update to Schema V3
pub fn create_eth_rpc<C, BE>(
    // ... parameters
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    // ... bounds
{
    let mut io = RpcModule::new(());

    // Use Schema V3 for v1.11.0
    let overrides = Arc::new(OverrideHandle {
        schemas: sp_api::StateBackendFor::<BE, C>::default(),
        fallback: Box::new(SchemaV3Override::new(client.clone())),
    });

    // Register Ethereum RPCs
    io.merge(EthApi::new(
        client.clone(),
        pool.clone(),
        graph.clone(),
        Some(TransactionConverter),
        network.clone(),
        vec![],
        overrides.clone(),
        backend,
        is_authority,
        block_data_cache.clone(),
        fee_history_cache,
        fee_history_limit,
        execute_gas_limit_multiplier,
        forced_parent_hashes,
    ).into_rpc())?;

    Ok(io)
}
```

#### Step 4.2: Update Service Configuration

**File:** `bin/node/src/service.rs`

Updates for executor and telemetry initialization.

```rust
// Update imports for v1.11.0
use sc_executor::{WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY};

// Update executor creation
let executor = WasmExecutor::builder()
    .with_execution_method(config.wasm_method)
    .with_onchain_heap_alloc_strategy(DEFAULT_HEAP_ALLOC_STRATEGY)
    .with_offchain_heap_alloc_strategy(DEFAULT_HEAP_ALLOC_STRATEGY)
    .with_max_runtime_instances(config.max_runtime_instances)
    .with_runtime_cache_size(config.runtime_cache_size)
    .build();
```

### Phase 5: Build and Test

#### Step 5.1: Clean Build

```bash
cd /home/msi/Project/selendra

# Clean all build artifacts
cargo clean

# Remove old wasm builds
rm -rf target/debug/wbuild
rm -rf target/release/wbuild

# Build in release mode
cargo build --release
```

#### Step 5.2: Check for Warnings

```bash
# Build with all warnings
cargo build --release 2>&1 | tee build.log

# Check for deprecation warnings
grep -i "deprecated" build.log
grep -i "warning" build.log
```

#### Step 5.3: Run Unit Tests

```bash
# Run all tests
cargo test --workspace --release

# Run specific pallet tests
cargo test -p pallet-aleph --release
cargo test -p pallet-committee-management --release
cargo test -p pallet-elections --release
cargo test -p pallet-operations --release
cargo test -p pallet-dynamic-evm-base-fee --release
cargo test -p pallet-unified-accounts --release
cargo test -p pallet-ethereum-checked --release
cargo test -p pallet-xvm --release
```

#### Step 5.4: Try-Runtime Checks

```bash
# Build with try-runtime feature
cargo build --release --features try-runtime

# Test migrations against local chain
./target/release/selendra try-runtime \
    --runtime ./target/release/wbuild/selendra-runtime/selendra_runtime.wasm \
    on-runtime-upgrade \
    --checks all \
    live --uri ws://localhost:9944

# Expected output:
# ✅ All pre-upgrade checks passed
# ✅ Migration executed successfully
# ✅ All post-upgrade checks passed
```

### Phase 6: Deploy to Local Testnet

#### Step 6.1: Start Fresh Local Network

```bash
cd /home/msi/Project/selendra

# Stop existing nodes
./scripts/stop_extra_validator.sh

# Clean old data
rm -rf run-nodes-local/*/chains/local_testnet/

# Start nodes with new binary
./scripts/run_nodes.sh

# Wait for network to start
sleep 30

# Check logs
tail -f run-nodes-local/*/chains/local_testnet/network/p2p.log
```

#### Step 6.2: Test Basic Operations

```bash
# Test RPC endpoints
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"system_chain"}' \
     http://localhost:9944

# Test Ethereum RPC
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"eth_blockNumber", "params":[]}' \
     http://localhost:9944

# Test new RPC methods
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"eth_feeHistory", "params":["0x5", "latest", []]}' \
     http://localhost:9944
```

#### Step 6.3: Deploy Runtime Upgrade

```bash
# Build runtime wasm
cargo build --release -p selendra-runtime

# The runtime wasm is at:
# target/release/wbuild/selendra-runtime/selendra_runtime.compact.compressed.wasm

# Submit runtime upgrade via sudo
# Use Polkadot.js Apps or custom script
```

**Via Polkadot.js Apps:**
1. Connect to ws://localhost:9944
2. Developer → Extrinsics
3. `sudo` → `sudoUncheckedWeight`
4. Call: `system` → `setCode`
5. Upload: `selendra_runtime.compact.compressed.wasm`
6. Submit transaction

#### Step 6.4: Monitor Migration

```bash
# Watch for migration logs
tail -f run-nodes-local/*/selendra.log | grep -i migration

# Expected logs:
# 🔧 Migrating staking to v14...
# ✅ Staking migrated to v14
# 🔧 Migrating nomination pools to v7...
# ✅ Nomination pools migrated to v7
# 🔧 Migrating Frontier to Schema V3...
# ✅ Frontier migration complete
# ✅ All migrations completed successfully
```

### Phase 7: Comprehensive Testing

#### Step 7.1: Functional Testing

**Test Staking:**
```bash
# Test validator operations
# - Add validator
# - Nominate
# - Check rewards
# - Unbond

# Use Polkadot.js or CLI tools
```

**Test Governance:**
```bash
# Test new Gov2 functionality
# - Submit referendum
# - Vote with conviction
# - Check referendum status
```

**Test EVM:**
```bash
# Deploy test contract
# Execute transactions
# Check gas calculations
# Verify event logs

# Example: Deploy ERC20 contract
node scripts/deploy-test-contract.js
```

#### Step 7.2: Performance Testing

```bash
# Load testing script
# Generate 1000 transactions
for i in {1..1000}; do
    # Submit test transaction
    curl -X POST \
         -H "Content-Type: application/json" \
         -d '{"jsonrpc":"2.0","method":"eth_sendRawTransaction","params":["0x..."],"id":1}' \
         http://localhost:9944 &
done

# Monitor block production
# Check for any delays or issues
```

#### Step 7.3: Integration Testing

**Test all custom pallets:**
- pallet-aleph
- pallet-committee-management
- pallet-elections
- pallet-operations
- pallet-dynamic-evm-base-fee
- pallet-unified-accounts
- pallet-ethereum-checked
- pallet-xvm

### Phase 8: Mainnet Preparation

#### Step 8.1: Create Upgrade Proposal

```bash
# Build final runtime
cargo build --release -p selendra-runtime --locked

# Generate proposal metadata
./target/release/selendra export-metadata \
    --chain=mainnet > mainnet-v211-metadata.json

# Create governance proposal with:
# - Runtime upgrade details
# - Migration summary
# - Testing results
# - Rollback plan
```

#### Step 8.2: Community Communication

**Announcement Template:**

```markdown
# Selendra Network Upgrade: v1.11.0

## Overview
Major network upgrade from Polkadot SDK v1.8.0 to v1.11.0

## Changes
- Governance v2 (OpenGov)
- Enhanced staking mechanisms
- Improved EVM performance
- Multiple runtime migrations

## Timeline
- Proposal: [Date]
- Voting Period: [Duration]
- Execution: [Date]
- Expected Downtime: ~30 minutes

## Action Required
- Validators: Update nodes to v2.1.1
- Nominators: No action required (automatic migration)
- dApp Developers: Review RPC changes

## Resources
- Full Documentation: [Link]
- Testing Results: [Link]
- Support: [Discord/Telegram]
```

#### Step 8.3: Validator Coordination

```bash
# Send notifications to all validators
# Provide upgrade binaries
# Schedule maintenance window
# Coordinate upgrade timing
```

---

## Testing Strategy

### 1. Local Development Testing

**Environment Setup:**
```bash
# Clean environment
cargo clean
rm -rf run-nodes-local/*/chains/

# Build fresh
cargo build --release

# Start local network
./scripts/run_nodes.sh
```

**Test Cases:**
- ✅ Node starts successfully
- ✅ Blocks are produced
- ✅ RPC endpoints respond
- ✅ Ethereum RPC works
- ✅ Transactions are processed

### 2. Migration Testing

**Try-Runtime Tests:**
```bash
# Test all migrations
cargo build --release --features try-runtime

./target/release/selendra try-runtime \
    --runtime ./target/release/wbuild/selendra-runtime/selendra_runtime.wasm \
    on-runtime-upgrade \
    --checks all \
    live --uri ws://localhost:9944
```

**Fork Testing:**
```bash
# Fork mainnet state
./target/release/selendra try-runtime \
    --runtime ./target/release/wbuild/selendra-runtime/selendra_runtime.wasm \
    on-runtime-upgrade \
    --checks all \
    live --uri wss://mainnet-rpc.selendra.org
```

### 3. Integration Testing

**Automated Test Suite:**
```bash
#!/bin/bash
# integration-tests.sh

echo "Running integration tests..."

# 1. Test staking
node tests/staking-tests.js

# 2. Test governance
node tests/governance-tests.js

# 3. Test EVM
node tests/evm-tests.js

# 4. Test XCM (if applicable)
node tests/xcm-tests.js

# 5. Test custom pallets
node tests/custom-pallets-tests.js

echo "All tests completed!"
```

### 4. Performance Testing

**Benchmark Tests:**
```bash
# Run runtime benchmarks
cargo build --release --features runtime-benchmarks

./target/release/selendra benchmark pallet \
    --chain=dev \
    --pallet=* \
    --extrinsic=* \
    --steps=50 \
    --repeat=20 \
    --output=./benchmarks/
```

**Load Testing:**
```bash
# Stress test with high transaction volume
# Monitor:
# - Block production time
# - Transaction pool size
# - Memory usage
# - CPU usage
```

### 5. Security Testing

**Audit Checklist:**
- [ ] Review all migration code
- [ ] Check for unsafe operations
- [ ] Verify weight calculations
- [ ] Test access controls
- [ ] Review cryptographic operations
- [ ] Check for potential panics
- [ ] Verify storage version bumps
- [ ] Test try-runtime extensively

---

## Rollback Plan

### Scenario 1: Pre-Deployment Issues

**If issues found during local testing:**

```bash
# Revert vendor updates
cd /home/msi/Project/selendra/vendors/polkadot-sdk
git checkout backup-v1.8.0

cd /home/msi/Project/selendra/vendors/frontier
git checkout backup-v1.8.0

# Rebuild
cd /home/msi/Project/selendra
cargo clean
cargo build --release

# Resume normal operations
```

### Scenario 2: Runtime Upgrade Failure

**If runtime upgrade transaction fails:**

1. **Diagnosis:**
   ```bash
   # Check logs
   tail -f selendra.log | grep -i error

   # Check migration status
   # Use polkadot.js to query storage
   ```

2. **Immediate Actions:**
   - Do NOT upgrade additional validators
   - Keep 2/3 validators on old version
   - Network will continue with old runtime

3. **Recovery:**
   ```bash
   # Fix issues in new runtime
   # Rebuild runtime wasm
   cargo build --release -p selendra-runtime

   # Submit corrected runtime upgrade
   ```

### Scenario 3: Post-Upgrade Critical Issues

**If critical issues found after upgrade:**

1. **Assessment:**
   - Severity of issue
   - Impact on network
   - Number of affected validators

2. **Quick Fix (Preferred):**
   ```bash
   # Fix issue in runtime
   # Create hotfix runtime version
   # Submit emergency runtime upgrade
   ```

3. **Full Rollback (Last Resort):**

   ⚠️ **WARNING:** Rolling back after migrations have run is extremely dangerous and may result in chain halt or data corruption.

   **Only if absolutely necessary:**
   ```bash
   # This requires:
   # 1. Restore from backup before upgrade
   # 2. Coordinate all validators
   # 3. Accept data loss since upgrade

   # DO NOT ATTEMPT without expert guidance
   ```

### Backup and Restore Procedures

**Pre-Upgrade Backup:**
```bash
# Backup script
#!/bin/bash
BACKUP_DIR="backup-$(date +%Y%m%d-%H%M%S)"
mkdir -p $BACKUP_DIR

# Backup database
cp -r run-nodes-local/*/chains/ $BACKUP_DIR/

# Backup runtime state
./target/release/selendra export-state \
    --chain=local > $BACKUP_DIR/state.json

# Backup binary
cp target/release/selendra $BACKUP_DIR/

# Create archive
tar -czf $BACKUP_DIR.tar.gz $BACKUP_DIR/

echo "Backup created: $BACKUP_DIR.tar.gz"
```

**Restore Procedure:**
```bash
# Restore script
#!/bin/bash
BACKUP_FILE=$1

# Extract backup
tar -xzf $BACKUP_FILE

# Stop nodes
./scripts/stop_extra_validator.sh

# Restore database
BACKUP_DIR=$(basename $BACKUP_FILE .tar.gz)
rm -rf run-nodes-local/*/chains/
cp -r $BACKUP_DIR/chains/ run-nodes-local/*/

# Restore binary
cp $BACKUP_DIR/selendra target/release/

# Restart nodes
./scripts/run_nodes.sh

echo "Restore completed from: $BACKUP_FILE"
```

---

## Post-Upgrade Verification

### Immediate Checks (First Hour)

**1. Node Health:**
```bash
# Check all validators are online
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
     http://localhost:9944

# Expected: {"peers":X, "isSyncing":false, "shouldHavePeers":true}
```

**2. Block Production:**
```bash
# Monitor blocks
watch -n 5 'curl -s -H "Content-Type: application/json" \
    -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\":\"chain_getHeader\"}" \
    http://localhost:9944 | jq .result.number'

# Should increment every 6 seconds (or your block time)
```

**3. Runtime Version:**
```bash
# Verify runtime version
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"state_getRuntimeVersion"}' \
     http://localhost:9944 | jq .result.specVersion

# Should return: 211 (or your target version)
```

**4. Migration Completion:**
```bash
# Check migration logs
tail -100 selendra.log | grep -i migration

# Should show all migrations completed successfully
```

### Short-term Monitoring (First 24 Hours)

**1. Transaction Processing:**
```bash
# Submit test transactions
# Monitor transaction pool
# Verify transactions are included in blocks
```

**2. Ethereum RPC:**
```bash
# Test all Ethereum RPC methods
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
     http://localhost:9944

curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"eth_feeHistory","params":["0x5","latest",[]],"id":1}' \
     http://localhost:9944
```

**3. Staking Operations:**
```bash
# Verify staking is working
# Check validator rewards
# Test nominator operations
# Verify nomination pool functions
```

**4. Governance:**
```bash
# Test new Gov2 features
# Submit test referendum
# Test voting mechanism
# Verify conviction voting works
```

### Medium-term Monitoring (First Week)

**1. Performance Metrics:**
```bash
# Monitor:
# - Block production time
# - Transaction throughput
# - Memory usage
# - Database growth rate
# - Network bandwidth
```

**2. Error Rates:**
```bash
# Check for any recurring errors
tail -1000 selendra.log | grep -i error | sort | uniq -c
```

**3. Community Feedback:**
- Monitor Discord/Telegram for user reports
- Track dApp functionality
- Collect validator feedback
- Monitor block explorer

### Long-term Monitoring (First Month)

**1. Stability:**
- No unexpected restarts
- Consistent block times
- Stable peer count
- No memory leaks

**2. Feature Adoption:**
- Gov2 usage statistics
- Nomination pool growth
- EVM transaction volume
- New RPC method usage

**3. Economic Metrics:**
- Staking rewards distribution
- Transaction fees
- Treasury income
- Validator commission

---

## Appendix

### A. Useful Commands

**Build Commands:**
```bash
# Clean build
cargo clean && cargo build --release

# Build specific package
cargo build --release -p selendra-runtime

# Build with features
cargo build --release --features try-runtime,runtime-benchmarks

# Check without building
cargo check --workspace
```

**Testing Commands:**
```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test -p pallet-aleph test_name

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo test --features runtime-benchmarks
```

**Node Commands:**
```bash
# Start node
./target/release/selendra --dev

# Purge chain
./target/release/selendra purge-chain --dev

# Export state
./target/release/selendra export-state --chain=dev

# Try runtime
./target/release/selendra try-runtime --runtime=<WASM> on-runtime-upgrade
```

**RPC Commands:**
```bash
# Get block number
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getHeader"}' \
     http://localhost:9944

# Get runtime version
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"state_getRuntimeVersion"}' \
     http://localhost:9944

# Submit extrinsic
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"author_submitExtrinsic", "params":["0x..."]}' \
     http://localhost:9944
```

### B. Key File Locations

**Configuration Files:**
- Runtime: `bin/runtime/src/lib.rs`
- Node Service: `bin/node/src/service.rs`
- RPC: `bin/node/src/rpc.rs`
- Chain Spec: `bin/node/src/chain_spec.rs`
- Dependencies: `Cargo.toml`

**Build Artifacts:**
- Runtime Wasm: `target/release/wbuild/selendra-runtime/selendra_runtime.compact.compressed.wasm`
- Node Binary: `target/release/selendra`
- Test Data: `run-nodes-local/`

**Vendor Modules:**
- Polkadot SDK: `vendors/polkadot-sdk/`
- Frontier: `vendors/frontier/`

**Custom Pallets:**
- Aleph: `pallets/aleph/`
- Committee Management: `pallets/committee-management/`
- Elections: `pallets/elections/`
- Operations: `pallets/operations/`
- Dynamic EVM Base Fee: `pallets/dynamic-evm-base-fee/`
- Unified Accounts: `pallets/unified-accounts/`
- Ethereum Checked: `pallets/ethereum-checked/`
- XVM: `pallets/xvm/`

### C. Reference Links

**Documentation:**
- Polkadot SDK Docs: https://paritytech.github.io/polkadot-sdk/master/
- Frontier Docs: https://github.com/polkadot-evm/frontier
- Substrate Docs: https://docs.substrate.io/
- Runtime Upgrades: https://docs.substrate.io/maintain/runtime-upgrades/

**Release Notes:**
- Polkadot SDK v1.9.0: https://github.com/paritytech/polkadot-sdk/releases/tag/polkadot-v1.9.0
- Polkadot SDK v1.10.0: https://github.com/paritytech/polkadot-sdk/releases/tag/polkadot-v1.10.0
- Polkadot SDK v1.11.0: https://github.com/paritytech/polkadot-sdk/releases/tag/polkadot-v1.11.0
- Frontier Releases: https://github.com/polkadot-evm/frontier/releases

**Community:**
- Substrate Stack Exchange: https://substrate.stackexchange.com/
- Polkadot Forum: https://forum.polkadot.network/
- Selendra Discord: [Your Discord]
- Selendra Telegram: [Your Telegram]

### D. Troubleshooting

**Common Issues:**

**Issue 1: Build Fails with "feature not found"**
```bash
# Solution: Update Cargo.lock
cargo update
cargo build --release
```

**Issue 2: Migration Timeout**
```bash
# Solution: Increase block time temporarily
# Or split migration into multiple steps
```

**Issue 3: RPC Connection Refused**
```bash
# Solution: Check node is running
ps aux | grep selendra

# Check RPC flags
./target/release/selendra --help | grep rpc
```

**Issue 4: Wasm Build Fails**
```bash
# Solution: Check wasm toolchain
rustup target add wasm32-unknown-unknown

# Clean and rebuild
rm -rf target/release/wbuild
cargo build --release -p selendra-runtime
```

**Issue 5: Migration Not Executing**
```bash
# Solution: Verify migration is in Executive
# Check storage version
# Ensure migration is included in runtime
```

### E. Contact Information

**Technical Support:**
- Lead Developer: [Email]
- DevOps Team: [Email]
- Security Team: [Email]

**Emergency Contacts:**
- 24/7 Hotline: [Phone]
- Emergency Email: [Email]
- Validator Chat: [Link]

---

## Changelog

**Version 1.0 - December 7, 2025**
- Initial document creation
- Comprehensive upgrade plan from v1.8.0 to v1.11.0
- Includes Polkadot SDK and Frontier updates
- Detailed migration procedures
- Testing and rollback strategies

---

## Document Metadata

- **Created:** December 7, 2025
- **Author:** Selendra Development Team
- **Version:** 1.0
- **Status:** Draft
- **Review Date:** TBD
- **Approval:** Pending

---

## Sign-off

**Prepared by:** _______________________
**Reviewed by:** _______________________
**Approved by:** _______________________
**Date:** _______________________

---

**END OF DOCUMENT**
