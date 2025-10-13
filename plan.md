# Plan: Remove `#[pallet::without_storage_info]` from All Pallets

## Status Update (2025-10-13)

### ✅ Completed: pallet-aleph
The `pallet-aleph` has been **successfully fixed**! All compilation errors resolved.

**What was fixed:**
- Added `MaxAuthorities` and `MaxCommitteeSize` config bounds
- Converted `Authorities`, `NextAuthorities`, `NextFinalityCommittee` from `Vec` to `BoundedVec`
- Updated `RawScore` in primitives from `Vec<u16>` to `BoundedVec<u16, ConstU32<1024>>`
- Added `MaxEncodedLen` derives to `VersionChange` and `Score` structs
- Fixed all type conversions throughout codebase (added `.to_vec()` where needed)
- **Removed `#[pallet::without_storage_info]` attribute successfully**

Build Status: ✅ `cargo check --release -p selendra-runtime` **PASSES**

---

## Remaining Pallets to Fix

### 🔴 pallet-committee-management (line 101)
### 🔴 pallet-elections (line 72)
### 🔴 pallet-operations (line 53)

---

## Problem Statement

These pallets currently use `#[pallet::without_storage_info]` attribute. This attribute is a **temporary workaround** that should be avoided in production code because:

1. **Security**: Without storage info, the runtime cannot calculate accurate storage deposit costs
2. **Performance**: The chain cannot optimize storage access patterns
3. **Best Practices**: It's considered a code smell in Substrate/Polkadot SDK
4. **Future-proofing**: Future Substrate versions may deprecate or remove this attribute

## Root Cause Analysis by Pallet

### 1. pallet-committee-management

**Location:** `pallets/committee-management/src/lib.rs:101`

**Problematic Storage Items:**
1. ✅ `LenientThreshold<T>` - `Perquintill` (implements `MaxEncodedLen`)
2. ✅ `SessionValidatorBlockCount<T>` - `StorageMap<..., BlockCount, ...>` (bounded)
3. ❌ `ValidatorEraTotalReward<T>` - `ValidatorTotalRewards<T::AccountId>` contains `BTreeMap<T, TotalReward>` (unbounded)
4. ✅ `ProductionBanConfig<T>` - `ProductionBanConfigStruct` (implements `MaxEncodedLen`)
5. ✅ `UnderperformedValidatorSessionCount<T>` - `StorageMap<..., SessionCount, ...>` (bounded)
6. ✅ `Banned<T>` - `StorageMap<..., BanInfo>` (bounded if BanInfo is bounded)
7. ❌ `CurrentAndNextSessionValidatorsStorage<T>` - `CurrentAndNextSessionValidators<T::AccountId>` contains `SessionValidators<T>` which has `Vec<T>` fields
8. ✅ `UnderperformedFinalizerSessionCount<T>` - `StorageMap<..., SessionCount, ...>` (bounded)
9. ✅ `FinalityBanConfig<T>` - `FinalityBanConfigStruct` (implements `MaxEncodedLen`)

**Problem Types:**
- `ValidatorTotalRewards<T>` - Contains `BTreeMap<T, TotalReward>` (unbounded)
- `CurrentAndNextSessionValidators<T>` - Contains `SessionValidators<T>` which has:
  - `producers: Vec<T>` (unbounded)
  - `finalizers: Vec<T>` (unbounded)
  - `non_committee: Vec<T>` (unbounded)

**Types from primitives that need updating:**
```rust
// In primitives/src/lib.rs
pub struct SessionValidators<T> {
    pub producers: Vec<T>,      // ❌ unbounded
    pub finalizers: Vec<T>,     // ❌ unbounded
    pub non_committee: Vec<T>,  // ❌ unbounded
}

pub struct ValidatorTotalRewards<T>(pub BTreeMap<T, TotalReward>); // ❌ unbounded
```

---

### 2. pallet-elections

**Location:** `pallets/elections/src/lib.rs:72`

**Problematic Storage Items:**
1. ✅ `CommitteeSize<T>` - `CommitteeSeats` (implements `MaxEncodedLen`)
2. ✅ `NextEraCommitteeSize<T>` - `CommitteeSeats` (implements `MaxEncodedLen`)
3. ❌ `NextEraReservedValidators<T>` - `Vec<T::AccountId>` (unbounded)
4. ❌ `CurrentEraValidators<T>` - `EraValidators<T::AccountId>` contains `Vec<T>` fields
5. ❌ `NextEraNonReservedValidators<T>` - `Vec<T::AccountId>` (unbounded)
6. ✅ `Openness<T>` - `ElectionOpenness` (implements `MaxEncodedLen`)

**Problem Types:**
- `Vec<T::AccountId>` in `NextEraReservedValidators` and `NextEraNonReservedValidators`
- `EraValidators<T>` which contains:
  - `reserved: Vec<AccountId>` (unbounded)
  - `non_reserved: Vec<AccountId>` (unbounded)

**Types from primitives that need updating:**
```rust
// In primitives/src/lib.rs
pub struct EraValidators<AccountId> {
    pub reserved: Vec<AccountId>,      // ❌ unbounded
    pub non_reserved: Vec<AccountId>,  // ❌ unbounded
}
```

---

### 3. pallet-operations

**Location:** `pallets/operations/src/lib.rs:53`

**Analysis needed:** Need to inspect this pallet's storage items (appears to have no storage based on grep results, may only use `without_storage_info` for other reasons)

## Solution Approach

The solution requires updates in **two layers**:

### Layer 1: Primitives (`primitives/src/lib.rs`)

Update shared types used across multiple pallets:

1. **SessionValidators<T>** - Change all `Vec<T>` fields to `BoundedVec<T, S>`
2. **EraValidators<T>** - Change all `Vec<T>` fields to `BoundedVec<T, S>`
3. **ValidatorTotalRewards<T>** - Change `BTreeMap<T, U>` to `BoundedBTreeMap<T, U, S>`

### Layer 2: Individual Pallets

Update each pallet's:
- Config trait (add max bound constants)
- Storage declarations
- Code logic (handle `BoundedVec`/`BoundedBTreeMap` operations)
- Tests and mocks

---

## Step 1: Determine Appropriate Bounds

From the codebase analysis:
- `MaxAuthorities` is already defined in runtime: 100,000 (used in pallet-aleph)
- Committee size is typically smaller (dozens to hundreds)
- Validator sets are similar in size to committee
- Need to balance:
  - **Too small**: System can't handle growth → runtime errors
  - **Too large**: Wastes storage deposits and memory

**Recommended bounds:**

| Bound Name | Value | Usage |
|------------|-------|-------|
| `MaxAuthorities` | 100,000 | Already exists, reuse for all authority lists |
| `MaxCommitteeSize` | 1,000 | Committee members (producers + finalizers) |
| `MaxReservedValidators` | 500 | Reserved validator list |
| `MaxNonReservedValidators` | 500 | Non-reserved validator list |
| `MaxValidatorRewards` | 1,000 | Max entries in ValidatorTotalRewards map |

---

## Implementation Plan

### Phase 1: Update Primitives (Foundation)

**File:** `primitives/src/lib.rs`

This must be done **first** as these types are used by multiple pallets.

#### 1.1: Update `SessionValidators<T>`

```rust
// Before:
pub struct SessionValidators<T> {
    pub producers: Vec<T>,
    pub finalizers: Vec<T>,
    pub non_committee: Vec<T>,
}

// After:
use frame_support::BoundedVec;
use codec::MaxEncodedLen;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
pub struct SessionValidators<T, S: Get<u32>> {
    pub producers: BoundedVec<T, S>,
    pub finalizers: BoundedVec<T, S>,
    pub non_committee: BoundedVec<T, S>,
}

impl<T, S: Get<u32>> Default for SessionValidators<T, S> {
    fn default() -> Self {
        Self {
            producers: BoundedVec::default(),
            finalizers: BoundedVec::default(),
            non_committee: BoundedVec::default(),
        }
    }
}
```

#### 1.2: Update `EraValidators<T>`

```rust
// Before:
pub struct EraValidators<AccountId> {
    pub reserved: Vec<AccountId>,
    pub non_reserved: Vec<AccountId>,
}

// After:
#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
pub struct EraValidators<AccountId, S: Get<u32>> {
    pub reserved: BoundedVec<AccountId, S>,
    pub non_reserved: BoundedVec<AccountId, S>,
}

impl<AccountId, S: Get<u32>> Default for EraValidators<AccountId, S> {
    fn default() -> Self {
        Self {
            reserved: BoundedVec::default(),
            non_reserved: BoundedVec::default(),
        }
    }
}
```

#### 1.3: Update `ValidatorTotalRewards<T>` (committee-management only)

```rust
// Before:
pub struct ValidatorTotalRewards<T>(pub BTreeMap<T, TotalReward>);

// After:
use frame_support::BoundedBTreeMap;

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
pub struct ValidatorTotalRewards<T: Ord, S: Get<u32>>(
    pub BoundedBTreeMap<T, TotalReward, S>
);
```

**Note:** `ValidatorTotalRewards` is defined in both `pallets/committee-management/src/lib.rs` and `pallets/elections/src/lib.rs`. Consider moving to primitives or keep separate (elections version may not need bounds).

---

### Phase 2: Fix pallet-committee-management

**File:** `pallets/committee-management/src/lib.rs`

#### 2.1: Add Config Bounds

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    // ... existing config ...

    /// Maximum number of validators in a session
    #[pallet::constant]
    type MaxValidators: Get<u32>;

    /// Maximum number of validator rewards entries
    #[pallet::constant]
    type MaxValidatorRewards: Get<u32>;
}
```

#### 2.2: Update Type Aliases and Structs

```rust
// Update CurrentAndNextSessionValidators to use bounded version
pub struct CurrentAndNextSessionValidators<T, S: Get<u32>> {
    pub next: SessionValidators<T, S>,
    pub current: SessionValidators<T, S>,
}

// Update ValidatorTotalRewards (if keeping local)
pub struct ValidatorTotalRewards<T: Ord, S: Get<u32>>(
    pub BoundedBTreeMap<T, TotalReward, S>
);
```

#### 2.3: Update Storage Declarations

```rust
/// SessionValidators in the current session.
#[pallet::storage]
#[pallet::getter(fn current_session_validators)]
pub(super) type CurrentAndNextSessionValidatorsStorage<T: Config> = StorageValue<
    _,
    CurrentAndNextSessionValidators<T::AccountId, T::MaxValidators>,
    ValueQuery
>;

/// Total possible reward per validator for the current era.
#[pallet::storage]
pub type ValidatorEraTotalReward<T: Config> = StorageValue<
    _,
    ValidatorTotalRewards<T::AccountId, T::MaxValidatorRewards>,
    OptionQuery
>;
```

#### 2.4: Update Runtime Config

In `bin/runtime/src/lib.rs`:

```rust
impl pallet_committee_management::Config for Runtime {
    // ... existing config ...
    type MaxValidators = ConstU32<1000>;
    type MaxValidatorRewards = ConstU32<1000>;
}
```

#### 2.5: Remove Attribute

Remove line 101: `#[pallet::without_storage_info]`

---

### Phase 3: Fix pallet-elections

**File:** `pallets/elections/src/lib.rs`

#### 3.1: Add Config Bounds

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    // ... existing config ...

    /// Maximum number of reserved validators
    #[pallet::constant]
    type MaxReservedValidators: Get<u32>;

    /// Maximum number of non-reserved validators
    #[pallet::constant]
    type MaxNonReservedValidators: Get<u32>;
}
```

#### 3.2: Update Storage Declarations

```rust
/// Next era's list of reserved validators.
#[pallet::storage]
pub type NextEraReservedValidators<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, T::MaxReservedValidators>,
    ValueQuery
>;

/// Current era's list of reserved validators.
#[pallet::storage]
#[pallet::getter(fn current_era_validators)]
pub type CurrentEraValidators<T: Config> = StorageValue<
    _,
    EraValidators<T::AccountId, T::MaxReservedValidators>,  // Use same bound for both
    ValueQuery
>;

/// Next era's list of non reserved validators.
#[pallet::storage]
pub type NextEraNonReservedValidators<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, T::MaxNonReservedValidators>,
    ValueQuery
>;
```

#### 3.3: Update Runtime Config

In `bin/runtime/src/lib.rs`:

```rust
impl pallet_elections::Config for Runtime {
    // ... existing config ...
    type MaxReservedValidators = ConstU32<500>;
    type MaxNonReservedValidators = ConstU32<500>;
}
```

#### 3.4: Remove Attribute

Remove line 72: `#[pallet::without_storage_info]`

---

### Phase 4: Fix pallet-operations

**File:** `pallets/operations/src/lib.rs`

**Investigation needed:** Check if this pallet actually has problematic storage or if it can simply remove the attribute.

If no unbounded types in storage, simply remove line 53: `#[pallet::without_storage_info]`

---

## Migration Considerations

### Storage Migration Needed? **NO** (for Vec → BoundedVec)

`BoundedVec<T, S>` and `BoundedBTreeMap<K, V, S>` have the **same SCALE encoding** as their unbounded counterparts, so no storage migration is needed. The bound is enforced at compile-time and runtime when inserting, but existing data is compatible.

**However:**
- If current chain state has validator counts exceeding the new bounds, the chain will panic on next runtime upgrade
- Before upgrading, check current chain state and set bounds with appropriate headroom

### Pre-Upgrade Checks

```bash
# Query current validator counts
polkadot-js-api query.palletElections.currentEraValidators
polkadot-js-api query.palletCommitteeManagement.currentSessionValidators
```

---

## Testing Strategy

### Per-Pallet Unit Tests

For each pallet after fixes:

```bash
# Run unit tests
cargo test -p pallet-committee-management
cargo test -p pallet-elections
cargo test -p pallet-operations

# Verify storage info is generated
cargo check -p pallet-committee-management
cargo check -p pallet-elections
cargo check -p pallet-operations
```

### Runtime Integration Test

```bash
# Build and check entire runtime
cargo check --release -p selendra-runtime

# Run all tests
cargo test --workspace

# Test on dev chain
./target/release/selendra-node --dev --tmp --rpc-cors=all
# Monitor for panics during session/era changes
```

---

## Implementation Checklist

### ✅ Phase 0: pallet-aleph (COMPLETED)
- [x] Added `MaxAuthorities` and `MaxCommitteeSize` config bounds
- [x] Converted storage from `Vec` to `BoundedVec`
- [x] Updated primitives `RawScore` type
- [x] Fixed all type conversions
- [x] Removed `#[pallet::without_storage_info]` attribute
- [x] All tests pass, runtime builds successfully

---

### 🔲 Phase 1: Update Primitives
- [ ] Update `SessionValidators<T>` to `SessionValidators<T, S: Get<u32>>`
  - [ ] Change `producers: Vec<T>` to `BoundedVec<T, S>`
  - [ ] Change `finalizers: Vec<T>` to `BoundedVec<T, S>`
  - [ ] Change `non_committee: Vec<T>` to `BoundedVec<T, S>`
  - [ ] Add `#[derive(MaxEncodedLen)]`
  - [ ] Update `Default` impl
- [ ] Update `EraValidators<T>` to `EraValidators<T, S: Get<u32>>`
  - [ ] Change `reserved: Vec<T>` to `BoundedVec<T, S>`
  - [ ] Change `non_reserved: Vec<T>` to `BoundedVec<T, S>`
  - [ ] Add `#[derive(MaxEncodedLen)]`
  - [ ] Update `Default` impl
- [ ] Test compilation of primitives: `cargo check -p primitives`

---

### 🔲 Phase 2: pallet-committee-management
- [ ] **2.1 Config Bounds**
  - [ ] Add `type MaxValidators: Get<u32>` to Config trait
  - [ ] Add `type MaxValidatorRewards: Get<u32>` to Config trait
- [ ] **2.2 Update Local Types**
  - [ ] Update `CurrentAndNextSessionValidators<T>` to include bound parameter
  - [ ] Update `ValidatorTotalRewards<T>` to use `BoundedBTreeMap`
- [ ] **2.3 Storage Declarations**
  - [ ] Update `CurrentAndNextSessionValidatorsStorage<T>` type
  - [ ] Update `ValidatorEraTotalReward<T>` type
- [ ] **2.4 Update Code Logic**
  - [ ] Fix all `.into()` and `.try_into()` conversions
  - [ ] Add `.to_vec()` where APIs expect `Vec`
  - [ ] Handle potential bound overflow errors
- [ ] **2.5 Runtime Config**
  - [ ] Add `MaxValidators = ConstU32<1000>` to runtime config
  - [ ] Add `MaxValidatorRewards = ConstU32<1000>` to runtime config
- [ ] **2.6 Tests & Mocks**
  - [ ] Update mock config with bounds
  - [ ] Update test fixtures
  - [ ] Run `cargo test -p pallet-committee-management`
- [ ] **2.7 Remove Attribute**
  - [ ] Delete line 101: `#[pallet::without_storage_info]`
  - [ ] Verify: `cargo check -p pallet-committee-management`

---

### 🔲 Phase 3: pallet-elections
- [ ] **3.1 Config Bounds**
  - [ ] Add `type MaxReservedValidators: Get<u32>` to Config trait
  - [ ] Add `type MaxNonReservedValidators: Get<u32>` to Config trait
- [ ] **3.2 Storage Declarations**
  - [ ] Update `NextEraReservedValidators<T>` to use `BoundedVec`
  - [ ] Update `CurrentEraValidators<T>` to use bounded `EraValidators`
  - [ ] Update `NextEraNonReservedValidators<T>` to use `BoundedVec`
- [ ] **3.3 Update Code Logic**
  - [ ] Fix all `.into()` and `.try_into()` conversions
  - [ ] Add `.to_vec()` where needed
  - [ ] Handle potential bound overflow errors
- [ ] **3.4 Runtime Config**
  - [ ] Add `MaxReservedValidators = ConstU32<500>` to runtime config
  - [ ] Add `MaxNonReservedValidators = ConstU32<500>` to runtime config
- [ ] **3.5 Tests & Mocks**
  - [ ] Update mock config with bounds
  - [ ] Update test fixtures
  - [ ] Run `cargo test -p pallet-elections`
- [ ] **3.6 Remove Attribute**
  - [ ] Delete line 72: `#[pallet::without_storage_info]`
  - [ ] Verify: `cargo check -p pallet-elections`

---

### 🔲 Phase 4: pallet-operations
- [ ] **4.1 Investigate**
  - [ ] Check if pallet has any storage items
  - [ ] Identify why `without_storage_info` is used
- [ ] **4.2 Fix or Remove**
  - [ ] If no problematic storage: simply remove attribute
  - [ ] If has unbounded storage: apply same pattern as above
- [ ] **4.3 Verify**
  - [ ] Run `cargo test -p pallet-operations`
  - [ ] Run `cargo check -p pallet-operations`

---

### 🔲 Phase 5: Final Integration
- [ ] **5.1 Build Entire Runtime**
  - [ ] Run `cargo check --release -p selendra-runtime`
  - [ ] Run `cargo build --release`
  - [ ] Fix any remaining compilation errors
- [ ] **5.2 Run All Tests**
  - [ ] Run `cargo test --workspace`
  - [ ] Fix any test failures
- [ ] **5.3 Dev Chain Test**
  - [ ] Start dev node: `./target/release/selendra-node --dev --tmp`
  - [ ] Monitor logs for panics
  - [ ] Test session transitions
  - [ ] Test era transitions
- [ ] **5.4 Code Quality**
  - [ ] Run `cargo clippy --workspace`
  - [ ] Run `cargo fmt --all`
- [ ] **5.5 Final Verification**
  - [ ] Verify no `#[pallet::without_storage_info]` remains in custom pallets
  - [ ] Document any remaining instances (e.g., in Frontier vendor code)
  - [ ] Update this plan.md with completion status

---

## Risks & Mitigations

### Risk 1: Bounds Too Small
**Impact**: Runtime panics when trying to add more validators than the bound

**Mitigation**:
- Set generous bounds (1,000 for validators is very safe for current network size)
- Check current mainnet/testnet state before setting bounds
- Add defensive error handling instead of `.expect()` where possible
- Log warnings when approaching limits
- Monitor chain state after upgrade

### Risk 2: Breaking Changes Across Multiple Pallets
**Impact**: Changes to primitives affect multiple pallets simultaneously

**Mitigation**:
- Update primitives first, then fix all affected pallets in order
- Test each pallet individually before final integration
- Use compiler errors as a checklist
- Have comprehensive test coverage

### Risk 3: BoundedBTreeMap Complexity
**Impact**: `ValidatorTotalRewards` uses `BTreeMap` which is more complex than `Vec`

**Mitigation**:
- `BoundedBTreeMap` is well-tested in FRAME
- Same SCALE encoding as `BTreeMap`
- Performance impact is negligible
- Alternative: Could use `BoundedVec<(T, TotalReward), S>` if needed

### Risk 4: Test Fixtures Need Updates
**Impact**: Many tests may fail due to type changes

**Mitigation**:
- Update mock configs systematically
- Use smaller bounds in tests (e.g., 100 instead of 1000)
- Update test helper functions
- Add new tests for boundary conditions

---

## Key Differences from pallet-aleph

### Shared Types in Primitives
Unlike pallet-aleph which had self-contained types, `SessionValidators` and `EraValidators` are shared types in `primitives/`. This means:

1. **Must add generic bound parameter** - e.g., `SessionValidators<T, S: Get<u32>>`
2. **Affects multiple pallets** - Changes cascade to both pallet-committee-management and pallet-elections
3. **Can't use pallet-specific bounds** - Must use generic parameter that each pallet configures

### BTreeMap vs Vec
`ValidatorTotalRewards` uses `BTreeMap` instead of `Vec`:
- Use `BoundedBTreeMap` from `frame_support`
- Requires `T: Ord` trait bound
- Same pattern: `BTreeMap<K, V>` → `BoundedBTreeMap<K, V, S>`

---

## Success Criteria

✅ **For each pallet**, the fix is complete when:
1. `#[pallet::without_storage_info]` attribute is removed
2. Pallet compiles: `cargo check -p pallet-name`
3. All pallet tests pass: `cargo test -p pallet-name`
4. No clippy warnings: `cargo clippy -p pallet-name`

✅ **For the entire project**, success means:
1. All three custom pallets fixed (committee-management, elections, operations)
2. Runtime builds: `cargo build --release -p selendra-runtime`
3. All workspace tests pass: `cargo test --workspace`
4. Dev chain runs through session/era transitions without panics
5. Storage info is properly generated for all pallets
6. No remaining `#[pallet::without_storage_info]` in custom code
   - Note: Frontier vendor code may still have it (not our concern)

---

## Timeline Estimate

| Phase | Task | Estimated Time |
|-------|------|----------------|
| ✅ Phase 0 | pallet-aleph | **6 hours (DONE)** |
| 🔲 Phase 1 | Update primitives | 2-3 hours |
| 🔲 Phase 2 | pallet-committee-management | 4-5 hours |
| 🔲 Phase 3 | pallet-elections | 3-4 hours |
| 🔲 Phase 4 | pallet-operations | 1-2 hours |
| 🔲 Phase 5 | Integration & testing | 3-4 hours |
| **Total** | | **19-24 hours** |

**Note:** pallet-committee-management is the most complex due to `BoundedBTreeMap` and extensive validator management logic.

---

## References

### Substrate Documentation
- [Substrate Storage Documentation](https://docs.substrate.io/build/runtime-storage/)
- [BoundedVec Documentation](https://docs.rs/frame-support/latest/frame_support/struct.BoundedVec.html)
- [BoundedBTreeMap Documentation](https://docs.rs/frame-support/latest/frame_support/struct.BoundedBTreeMap.html)
- [MaxEncodedLen Trait](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/trait.MaxEncodedLen.html)

### Internal References
- ✅ **Completed example**: `pallets/aleph/` - Reference implementation
- 🔴 **To fix**: `pallets/committee-management/src/lib.rs:101`
- 🔴 **To fix**: `pallets/elections/src/lib.rs:72`
- 🔴 **To fix**: `pallets/operations/src/lib.rs:53`
- **Primitives**: `primitives/src/lib.rs` - Shared types

### Similar Work in Ecosystem
- Polkadot/Kusama pallets: Examples of `BoundedVec` usage in production
- FRAME best practices: Avoiding `without_storage_info` attribute

---

## Notes

### Vendor Code (Frontier)
The following files also have `#[pallet::without_storage_info]`:
- `vendors/frontier/frame/evm/src/lib.rs`
- `vendors/frontier/frame/ethereum/src/lib.rs`

**These are NOT in scope** - they are external dependencies. We only fix custom Selendra pallets.

### Future Improvements
After completing this work, consider:
1. Adding runtime benchmarks for accurate weight calculations
2. Monitoring actual validator counts in production
3. Adjusting bounds based on real-world usage
4. Adding telemetry for bound utilization (e.g., warn at 80% capacity)

---

**Last Updated:** 2025-10-13
**Status:** pallet-aleph ✅ complete, 3 pallets remaining 🔴
