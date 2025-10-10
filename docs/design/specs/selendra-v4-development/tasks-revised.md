# Selendra V4 Development Plan - Optimized for 2-3 Person Team

**Target Timeline:** 48 weeks (12 months)
**Team Size:** 2-3 core developers + Claude Code
**Focus:** Critical path to production-ready L1 blockchain

---

## Priority Legend
- **[P0]** - Critical, blocks other work
- **[P1]** - High value, core features
- **[P2]** - Nice to have, can be deferred
- **[CUT]** - Removed from scope

## Role Assignments
- **[@runtime]** - Runtime/Pallets developer (Rust)
- **[@frontend]** - Frontend/SDK developer (TypeScript)
- **[@fullstack]** - Can work on both (flexible assignment)

---

## Phase 1: Security Critical Fixes (Weeks 1-4)

### **TASK-001: Remove Insecure Randomness** [P0] [@runtime]

**Why Critical:** Current randomness is validator-manipulable, breaking gambling/gaming contracts.

**Prerequisites:** None - start immediately

**Deliverables:**
- [ ] Remove `pallet_insecure_randomness_collective_flip` from runtime
- [ ] Add Moonbeam's `pallet-randomness` dependency to `Cargo.toml`
- [ ] Configure VRF+VDF randomness with proper parameters
- [ ] Update `pallet_contracts::Config` to use new randomness source
- [ ] Write 10+ unit tests for randomness quality
- [ ] Deploy to testnet and collect 1000+ random samples for analysis

**Definition of Done:**
- [ ] Zero references to `pallet_insecure_randomness_collective_flip` in codebase
- [ ] All tests passing (unit + integration)
- [ ] Contracts can successfully call `seal_random()` function
- [ ] Randomness passes chi-square distribution test
- [ ] Testnet validated for 48 hours with no issues

**API Contract:**
```rust
// Runtime config
impl pallet_contracts::Config for Runtime {
    type Randomness = Randomness; // Now points to pallet_randomness
    // ... other config
}

// Usage in contracts
let random_value = self.env().random(&subject).0;
```

**Files to Modify:**
- `bin/runtime/src/lib.rs` (lines 413, 784, 1020)
- `bin/runtime/Cargo.toml`

**Estimated Effort:** 2 weeks
**Actual Effort:** ___ (track after completion)

**Risks:**
- Moonbeam pallet version compatibility with our Polkadot SDK fork
- Migration strategy for existing contracts using old randomness

---

### **TASK-002: Expand Contract Runtime Call Filter** [P0] [@runtime]

**Why Critical:** Contracts currently CANNOT transfer tokens or batch operations. This kills DeFi composability.

**Prerequisites:** None

**Deliverables:**
- [ ] Modify `ContractsCallRuntimeFilter` in `bin/runtime/src/lib.rs:771-779`
- [ ] Allow `Balances::transfer`, `transfer_keep_alive`, `transfer_all`
- [ ] Allow `Utility::batch`, `batch_all`, `force_batch`
- [ ] Keep blocking `Sudo`, `Treasury`, `Operations`, `Democracy`
- [ ] Write 20+ unit tests covering all allowed/denied calls
- [ ] Deploy example contract demonstrating token transfers

**Definition of Done:**
- [ ] Contracts can successfully call `Balances::transfer`
- [ ] Contracts can batch multiple operations
- [ ] Contracts CANNOT call sudo or treasury (security maintained)
- [ ] All tests passing
- [ ] Example contract deployed to testnet with documentation

**API Contract:**
```rust
impl Contains<RuntimeCall> for ContractsCallRuntimeFilter {
    fn contains(call: &RuntimeCall) -> bool {
        matches!(
            call,
            RuntimeCall::Balances(
                pallet_balances::Call::transfer { .. } |
                pallet_balances::Call::transfer_keep_alive { .. } |
                pallet_balances::Call::transfer_all { .. }
            ) |
            RuntimeCall::Staking(_) |
            RuntimeCall::NominationPools(_) |
            RuntimeCall::Utility(
                pallet_utility::Call::batch { .. } |
                pallet_utility::Call::batch_all { .. } |
                pallet_utility::Call::force_batch { .. }
            )
            // Explicitly blocked: Sudo, Treasury, Operations
        )
    }
}
```

**Files to Modify:**
- `bin/runtime/src/lib.rs:771-779`

**Estimated Effort:** 1 week
**Actual Effort:** ___

**Risks:**
- Careful security review needed to ensure no privileged calls leak through

---

### **TASK-003: Fix Unbounded Storage in Core Pallets** [P0] [@runtime]

**Why Critical:** Storage exhaustion vulnerability in 4 critical pallets.

**Prerequisites:** None

**Deliverables:**
- [ ] Remove `#[pallet::without_storage_info]` from `pallet-operations`
- [ ] Add bounded storage with `MaxAccounts: u32 = 10000` parameter
- [ ] Apply same fix to `pallet-elections`
- [ ] Apply same fix to `pallet-committee-management`
- [ ] Apply same fix to `pallet-aleph`
- [ ] Implement storage migrations for existing data
- [ ] Test migrations on testnet fork
- [ ] Verify all pallets compile without `without_storage_info`

**Definition of Done:**
- [ ] Zero `#[pallet::without_storage_info]` attributes in production pallets
- [ ] All storage items use `BoundedVec` or equivalent
- [ ] Storage migrations tested on testnet with real data
- [ ] Migrations complete in < 2 blocks
- [ ] All tests passing
- [ ] Runtime compiles without warnings

**Implementation Example:**
```rust
// Before
#[pallet::pallet]
#[pallet::without_storage_info]  // ❌ REMOVE THIS
pub struct Pallet<T>(_);

// After
#[pallet::pallet]
pub struct Pallet<T>(_);

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

**Files to Modify:**
- `pallets/operations/src/lib.rs:53`
- `pallets/elections/src/lib.rs:72`
- `pallets/committee-management/src/lib.rs:101`
- `pallets/aleph/src/lib.rs:78`

**Estimated Effort:** 1 week
**Actual Effort:** ___

**Risks:**
- Migration testing critical - could brick chain if done wrong
- Need to verify MAX bounds don't break existing validator sets

---

## Phase 2: Governance & Decentralization (Weeks 5-16)

### **TASK-004: Implement Council Governance System** [P1] [@runtime]

**Why Important:** Removing sudo requires alternative governance mechanism.

**Prerequisites:** Phase 1 complete (need secure base)

**Deliverables:**
- [ ] Add `pallet-collective` dependency to runtime
- [ ] Configure 7-member council with proper parameters
- [ ] Set motion duration to 7 days
- [ ] Implement 4/7 voting threshold for proposals
- [ ] Create council election mechanism via `pallet-elections`
- [ ] Grant council powers for treasury approval
- [ ] Grant council powers for runtime upgrades
- [ ] Write tests for proposal lifecycle
- [ ] Deploy to testnet and run mock governance

**Definition of Done:**
- [ ] Council can create, vote on, and execute proposals
- [ ] Council can approve treasury spends
- [ ] Council can authorize runtime upgrades
- [ ] Election mechanism selects 7 council members
- [ ] All governance tests passing
- [ ] Testnet council operational for 2 weeks

**API Contract:**
```rust
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
```

**Files to Modify:**
- `bin/runtime/src/lib.rs` (add council config)
- `bin/runtime/Cargo.toml` (add pallet-collective)

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

---

### **TASK-005: Integrate Democracy Pallet** [P1] [@runtime]

**Why Important:** Enables community-driven proposals and voting.

**Prerequisites:** TASK-004 (council provides checks/balances)

**Deliverables:**
- [ ] Add `pallet-democracy` and `pallet-conviction-voting` dependencies
- [ ] Configure referendum parameters (launch: 7d, voting: 14d, enactment: 2d)
- [ ] Set minimum deposit to 100 SEL for proposals
- [ ] Implement conviction-based voting weights
- [ ] Configure council as fast-track origin
- [ ] Write tests for referendum lifecycle
- [ ] Deploy to testnet and run test referendum

**Definition of Done:**
- [ ] Anyone can submit proposal with 100 SEL deposit
- [ ] Community can vote with conviction multipliers
- [ ] Successful referenda execute after enactment period
- [ ] Council can fast-track critical proposals
- [ ] All tests passing
- [ ] Test referendum passed on testnet

**API Contract:**
```rust
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
    // ... more config
}
```

**Files to Modify:**
- `bin/runtime/src/lib.rs`
- `bin/runtime/Cargo.toml`

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

---

### **TASK-006: Remove Sudo & Complete Decentralization** [P1] [@runtime]

**Why Important:** Decentralization = credibility for serious blockchain.

**Prerequisites:** TASK-004 + TASK-005 (governance must work first)

**Deliverables:**
- [ ] Update all `EnsureRoot` origins to `EnsureRootOrHalfCouncil`
- [ ] Migrate treasury governance to council control
- [ ] Update operations pallet to use council approval
- [ ] Remove `pallet-sudo` from `construct_runtime!`
- [ ] Remove `pallet-sudo` from `Cargo.toml`
- [ ] Implement emergency safe mode without sudo
- [ ] Test all admin functions work via council
- [ ] **Mainnet:** Execute sudo key burning ceremony

**Definition of Done:**
- [ ] Zero references to `pallet_sudo` in runtime
- [ ] All administrative functions require council/democracy approval
- [ ] Treasury spends require council vote
- [ ] Runtime upgrades require referendum
- [ ] Emergency safe mode functional without centralized control
- [ ] Testnet operating sudo-free for 4 weeks
- [ ] **Mainnet:** Sudo key = 0x000...000 (burned)

**Migration Checklist:**
```rust
// Before
impl pallet_treasury::Config for Runtime {
    type ApproveOrigin = EnsureSignedBy<TreasuryGovernance, AccountId>;
    type RejectOrigin = EnsureSignedBy<TreasuryGovernance, AccountId>;
    // ...
}

// After
type EnsureRootOrHalfCouncil = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
>;

impl pallet_treasury::Config for Runtime {
    type ApproveOrigin = EnsureRootOrHalfCouncil;
    type RejectOrigin = EnsureRootOrHalfCouncil;
    // ...
}
```

**Files to Modify:**
- `bin/runtime/src/lib.rs` (remove sudo, update origins)
- `bin/runtime/Cargo.toml` (remove sudo dependency)

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

**Critical Risks:**
- No way to recover if governance breaks after sudo removal
- Must have 100% confidence in council + democracy before removing sudo
- Should have escape hatch (emergency multisig?) documented

---

## Phase 3: DeFi Infrastructure (Weeks 17-28)

### **TASK-007: Build TypeScript SDK** [P0] [@frontend]

**Why Critical:** EVERYTHING depends on this. No SDK = no apps possible.

**Prerequisites:** Phase 1 complete (need stable runtime)

**Deliverables:**
- [ ] Create new repository `selendra-sdk-ts` with TypeScript config
- [ ] Implement `ApiManager` for WebSocket connections
- [ ] Build `AccountManager` for native + EVM accounts
- [ ] Create `TransactionBuilder` for common operations
- [ ] Add `BalanceManager` for token queries
- [ ] Implement proper TypeScript types from chain metadata
- [ ] Write 50+ unit tests
- [ ] Create 10+ example scripts
- [ ] Generate API documentation with TypeDoc
- [ ] Publish to npm as `@selendra/sdk`

**Definition of Done:**
- [ ] Package published to npm
- [ ] Can connect to testnet via WebSocket
- [ ] Can query balances, send transfers, stake, etc.
- [ ] Full TypeScript types (no `any` types)
- [ ] 80%+ test coverage
- [ ] Documentation site live
- [ ] 10 working examples in `/examples` folder

**API Contract:**
```typescript
// Core API
import { SelendraApi } from '@selendra/sdk';

const api = new SelendraApi();
await api.connect('wss://rpc.testnet.selendra.org');

// Account management
const account = api.accounts.fromMnemonic('your twelve words here');
const balance = await api.balances.get(account.address);

// Transactions
const tx = await api.transactions.transfer(
    from: account,
    to: 'recipient-address',
    amount: '100 SEL'
);
await tx.signAndSend(account);

// Staking
await api.staking.bond(validator, amount);
await api.staking.nominate([validator1, validator2]);
```

**Package Structure:**
```
@selendra/sdk/
├── src/
│   ├── api/           # WebSocket connection manager
│   ├── accounts/      # Account management (native + EVM)
│   ├── balances/      # Balance queries
│   ├── transactions/  # Transaction builders
│   ├── staking/       # Staking operations
│   ├── contracts/     # Contract interaction (later)
│   ├── types/         # Generated types from metadata
│   └── utils/         # Helper functions
├── examples/
│   ├── 01-connect.ts
│   ├── 02-transfer.ts
│   ├── 03-stake.ts
│   └── ...
├── tests/
└── docs/
```

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

**Risks:**
- Polkadot.js types can be complex to wrap elegantly
- Keeping types in sync with runtime upgrades

---

### **TASK-008: Build Native DEX Pallet** [P1] [@runtime]

**Why Important:** Native DEX = 10x cheaper gas = killer feature vs EVM chains.

**Prerequisites:** Phase 1 + Phase 2 complete

**Deliverables:**
- [ ] Create `pallets/dex/` with Uniswap V2 constant product AMM formula
- [ ] Implement `create_pool(token_a, token_b, amount_a, amount_b)`
- [ ] Implement `add_liquidity(pool_id, amount_a, amount_b)`
- [ ] Implement `remove_liquidity(pool_id, liquidity_amount)`
- [ ] Implement `swap_exact_tokens_for_tokens(token_in, token_out, amount_in, min_out)`
- [ ] Implement LP token minting/burning
- [ ] Add 0.3% trading fee collection
- [ ] Implement TWAP price oracle
- [ ] Write 100+ tests covering edge cases
- [ ] Benchmark all operations (target: <50,000 weight for swaps)
- [ ] Deploy to testnet with 3+ trading pairs

**Definition of Done:**
- [ ] Can create pools for any token pair
- [ ] Can add/remove liquidity with LP token accounting
- [ ] Swaps work with slippage protection
- [ ] Trading fees accumulate correctly
- [ ] TWAP oracle updates on each trade
- [ ] All tests passing including fuzzing
- [ ] Benchmarks show 10x better than EVM DEX
- [ ] Testnet has 3+ active pools with real trades

**API Contract:**
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Create a new liquidity pool
    #[pallet::weight(10_000)]
    pub fn create_pool(
        origin: OriginFor<T>,
        token_a: T::AssetId,
        token_b: T::AssetId,
        amount_a: T::Balance,
        amount_b: T::Balance,
    ) -> DispatchResult;

    /// Swap exact tokens for minimum output
    #[pallet::weight(10_000)]
    pub fn swap_exact_tokens_for_tokens(
        origin: OriginFor<T>,
        token_in: T::AssetId,
        token_out: T::AssetId,
        amount_in: T::Balance,
        amount_out_min: T::Balance,
    ) -> DispatchResult;
}
```

**Files to Create:**
- `pallets/dex/src/lib.rs`
- `pallets/dex/src/tests.rs`
- `pallets/dex/src/benchmarking.rs`
- `pallets/dex/Cargo.toml`

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

**Risks:**
- Complex math for liquidity calculations
- Front-running protection needed
- Must handle decimal precision carefully

---

### **TASK-009: Implement sUSD Bridge Token** [P1] [@runtime]

**Why Important:** Simple stablecoin to bootstrap DeFi ecosystem.

**Prerequisites:** TASK-008 (DEX needed for trading)

**Deliverables:**
- [ ] Create `pallets/bridge-token/` for 1:1 USDT/USDC backing
- [ ] Implement `mint(amount)` with bridge operator signature
- [ ] Implement `burn(amount)` with withdrawal address
- [ ] Add reserve tracking (must maintain 100%+ backing)
- [ ] Create bridge monitoring with alerts
- [ ] Set up multi-sig bridge operators (2-of-3)
- [ ] Write 50+ tests for mint/burn operations
- [ ] Deploy to testnet with 3 bridge operators
- [ ] Create operator dashboard for monitoring

**Definition of Done:**
- [ ] Can mint sUSD when USDT deposited to Ethereum bridge contract
- [ ] Can burn sUSD and receive USDT on Ethereum
- [ ] Reserve ratio never drops below 100%
- [ ] Multi-sig requires 2/3 operators for operations
- [ ] All tests passing
- [ ] Testnet operational with real bridging for 2 weeks
- [ ] Monitoring dashboard shows reserve ratio real-time

**API Contract:**
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Mint sUSD (bridge operators only)
    #[pallet::weight(10_000)]
    pub fn mint(
        origin: OriginFor<T>,
        to: T::AccountId,
        amount: T::Balance,
        eth_tx_hash: H256,
    ) -> DispatchResult;

    /// Burn sUSD and request withdrawal
    #[pallet::weight(10_000)]
    pub fn burn(
        origin: OriginFor<T>,
        amount: T::Balance,
        eth_address: H160,
    ) -> DispatchResult;

    /// Get current reserve ratio
    fn get_reserve_ratio() -> Perbill;
}
```

**Files to Create:**
- `pallets/bridge-token/src/lib.rs`
- `pallets/bridge-token/src/tests.rs`

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

### **TASK-010: Build KHRt Stablecoin Pallet** [P1] [@runtime]

**Why Important:** Key differentiator - local currency with bank integration.

**Prerequisites:** TASK-009 (learn from sUSD implementation)

**⚠️ CRITICAL BLOCKER:** Requires banking partnerships with ABA, ACLEDA, WING. Build pallet now, but CANNOT LAUNCH until partnerships confirmed.

**Deliverables:**
- [ ] Create `pallets/khrt/` with Cambodian Riel backing
- [ ] Implement `mint(amount, bank_tx_id)` with bank deposit verification
- [ ] Implement `burn(amount, bank_account)` with auto withdrawal
- [ ] Add KYC/AML compliance checks
- [ ] Implement daily/monthly limits by KYC tier
- [ ] Add 110% reserve requirement
- [ ] Create emergency pause mechanism
- [ ] Integrate with Baray API for bank transactions
- [ ] Write 100+ tests including compliance scenarios
- [ ] **Build banking API mock for testing**

**Definition of Done:**
- [ ] Pallet compiles and all tests pass
- [ ] Mock banking API works in testnet
- [ ] Can mint KHRt with simulated bank deposit
- [ ] Can burn KHRt with simulated bank withdrawal
- [ ] KYC checks prevent over-limit transactions
- [ ] Reserve monitoring works correctly
- [ ] Emergency pause can be triggered
- [ ] **Pallet ready, awaiting bank partnerships to launch**

**API Contract:**
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Mint KHRt after bank deposit confirmed
    #[pallet::weight(10_000)]
    pub fn mint(
        origin: OriginFor<T>,
        amount: T::Balance,
        bank_tx_id: Vec<u8>,
        kyc_level: KycLevel,
    ) -> DispatchResult;

    /// Burn KHRt and request bank withdrawal
    #[pallet::weight(10_000)]
    pub fn burn(
        origin: OriginFor<T>,
        amount: T::Balance,
        bank_account: BankAccount,
    ) -> DispatchResult;
}
```

**Banking Partnership Checklist (PARALLEL WORKSTREAM):**
- [ ] Initial discussions with ABA Bank
- [ ] Initial discussions with ACLEDA Bank
- [ ] Initial discussions with WING Bank
- [ ] API access agreements signed
- [ ] Regulatory compliance path defined
- [ ] KYC provider selected and integrated
- [ ] Reserve account opened
- [ ] Audit requirements documented
- [ ] Launch approval from regulators

**Files to Create:**
- `pallets/khrt/src/lib.rs`
- `pallets/khrt/src/banking.rs` (API integration)
- `pallets/khrt/src/compliance.rs` (KYC/AML)
- `pallets/khrt/src/tests.rs`

**Estimated Effort:** 4 weeks (pallet only)
**Banking Partnership Timeline:** 3-12 months (external dependency)

**Actual Effort:** ___

---

## Phase 4: Cross-Chain & Testing (Weeks 29-40)

### **TASK-011: Implement LayerZero Bridge** [P1] [@fullstack]

**Why Important:** Cross-chain liquidity unlocks growth.

**Prerequisites:** Phase 3 complete (need tokens to bridge)

**⚠️ SECURITY CRITICAL:** Must get external audit before mainnet. Budget $50K-100K.

**Deliverables:**
- [ ] Deploy LayerZero endpoint contract on Selendra
- [ ] Deploy Ethereum endpoint for SEL <-> ETH bridging
- [ ] Add support for USDC, USDT, WBTC bridging
- [ ] Implement 2-of-3 multi-sig security
- [ ] Add 24-hour timelock for transfers > $1M
- [ ] Create bridge monitoring pallet
- [ ] Implement emergency pause mechanism
- [ ] Build bridge UI for users
- [ ] Write 100+ tests for bridge scenarios
- [ ] **Get external security audit**

**Definition of Done:**
- [ ] Can bridge SEL from Selendra to Ethereum
- [ ] Can bridge USDC from Ethereum to Selendra
- [ ] Multi-sig works correctly (requires 2/3)
- [ ] Timelock delays large transfers
- [ ] Emergency pause functional
- [ ] All tests passing
- [ ] **Security audit passed with all critical issues fixed**
- [ ] Testnet bridge operational for 4 weeks
- [ ] Bridge UI deployed and working

**Security Requirements:**
```rust
// Multi-sig for bridge operations
pub struct BridgeOperators {
    operators: [AccountId; 3],
    threshold: u8, // = 2
}

// Timelock for large transfers
pub struct BridgeTimelock {
    min_delay: BlockNumber, // 24 hours for > $1M
    pending_transfers: BTreeMap<H256, PendingTransfer>,
}
```

**Estimated Effort:** 6 weeks (excluding audit wait time)
**Security Audit:** 2-4 weeks + $50K-100K budget
**Actual Effort:** ___

---

### **TASK-012: Comprehensive Testing & QA** [P0] [@fullstack]

**Why Critical:** Can't launch without thorough testing.

**Prerequisites:** All Phase 1-4 features complete

**Deliverables:**
- [ ] Write 100+ integration tests covering critical paths
- [ ] Implement cross-runtime tests (native ↔ EVM)
- [ ] Create runtime upgrade test scenarios
- [ ] Build automated CI/CD testing pipeline
- [ ] Perform load testing (1000+ TPS target)
- [ ] Conduct chaos engineering tests
- [ ] Test all upgrade migration paths
- [ ] Verify backwards compatibility
- [ ] Create test coverage report (target: 85%+)

**Definition of Done:**
- [ ] 100+ integration tests passing
- [ ] Can handle 1000 TPS on testnet
- [ ] Runtime upgrades work smoothly
- [ ] Test coverage ≥ 85%
- [ ] CI/CD runs all tests on every PR
- [ ] Chaos tests don't crash node
- [ ] All critical user journeys tested end-to-end

**Test Categories:**
```
1. Runtime Tests (50+ tests)
   - Balance transfers
   - Staking operations
   - Governance voting
   - Treasury proposals

2. DEX Tests (30+ tests)
   - Pool creation
   - Liquidity provision
   - Token swaps
   - Fee collection

3. Bridge Tests (20+ tests)
   - Cross-chain transfers
   - Multi-sig operations
   - Timelock mechanisms
   - Emergency pause

4. Integration Tests (50+ tests)
   - End-to-end user journeys
   - Cross-pallet interactions
   - Upgrade scenarios
```

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

---

### **TASK-013: Security Audit & Launch Prep** [P0] [@fullstack]

**Why Critical:** Can't launch without professional audit.

**Prerequisites:** TASK-012 complete (must pass internal testing first)

**Deliverables:**
- [ ] Select audit firm (3-5 proposals)
- [ ] Prepare audit documentation
- [ ] Submit code for audit
- [ ] **Wait for audit results** (2-4 weeks)
- [ ] Fix all critical and high severity findings
- [ ] Re-audit fixes if needed
- [ ] Prepare mainnet deployment runbook
- [ ] Create rollback procedures
- [ ] Document upgrade process
- [ ] Train community validators

**Definition of Done:**
- [ ] Audit report received
- [ ] Zero critical vulnerabilities
- [ ] Zero high severity issues (or all fixed)
- [ ] Audit firm gives approval to launch
- [ ] Deployment runbook tested on testnet
- [ ] Rollback procedure tested
- [ ] 20+ validators ready for mainnet

**Audit Scope:**
- All custom pallets (DEX, bridge-token, khrt)
- Runtime configuration
- Bridge contracts
- Multi-sig implementations
- Governance mechanisms

**Budget:** $50K-150K depending on firm

**Estimated Effort:** 4 weeks (plus 2-4 week audit wait)
**Actual Effort:** ___

---

## Phase 5: Documentation & Developer Experience (Weeks 41-48)

### **TASK-014: Developer Documentation** [P1] [@frontend]

**Why Important:** Good docs = developer adoption.

**Prerequisites:** SDK complete (TASK-007)

**Deliverables:**
- [ ] Write "Quick Start" guide (5-minute deploy goal)
- [ ] Create comprehensive SDK documentation
- [ ] Build 20+ code examples
- [ ] Write migration guide from v3 to v4
- [ ] Document all precompiles with Solidity interfaces
- [ ] Create video tutorials (5 videos x 10 minutes)
- [ ] Set up documentation site (docs.selendra.org)
- [ ] Add troubleshooting guides

**Definition of Done:**
- [ ] Developer can deploy first contract in < 5 minutes following docs
- [ ] All SDK methods documented with examples
- [ ] 20+ working code examples
- [ ] 5 video tutorials published
- [ ] Docs site live and searchable
- [ ] Migration guide tested by 3+ developers

**Documentation Structure:**
```
docs.selendra.org/
├── Quick Start/
│   ├── Install SDK
│   ├── First Transaction
│   ├── Deploy Contract
│   └── Use DEX
├── SDK Reference/
│   ├── API
│   ├── Accounts
│   ├── Transactions
│   ├── Staking
│   └── DEX
├── Examples/
│   ├── Token Transfer
│   ├── Staking
│   ├── DEX Trading
│   ├── Bridge
│   └── Governance
├── Tutorials/
│   ├── Build a DApp
│   ├── Create a Token
│   ├── Build a DEX UI
│   └── NFT Marketplace
└── Migration/
    ├── v3 to v4 Guide
    ├── Breaking Changes
    └── Troubleshooting
```

**Estimated Effort:** 2 weeks
**Actual Effort:** ___

---

### **TASK-015: Testnet Faucet & Developer Tools** [P2] [@frontend]

**Why Important:** Removes friction for developers.

**Prerequisites:** TASK-014 (docs should link to faucet)

**Deliverables:**
- [ ] Build instant testnet faucet (no signup required)
- [ ] Implement simple CAPTCHA rate limiting
- [ ] Create faucet API endpoint
- [ ] Add Discord bot integration (`/faucet <address>`)
- [ ] Build transaction explorer for testnet
- [ ] Create simple block explorer
- [ ] Deploy all tools to production

**Definition of Done:**
- [ ] Faucet dispenses 100 SEL in < 10 seconds
- [ ] Rate limit: 1 request per hour per address
- [ ] Discord bot works in developer channel
- [ ] Faucet API documented
- [ ] Explorer shows blocks, txs, accounts
- [ ] All tools live at testnet.selendra.org

**Faucet API:**
```typescript
POST https://faucet.selendra.org/api/drip
{
  "address": "0x1234...",
  "captcha": "token"
}

Response:
{
  "tx_hash": "0xabc...",
  "amount": "100 SEL",
  "explorer_link": "https://testnet.selendra.org/tx/0xabc..."
}
```

**Estimated Effort:** 2 weeks
**Actual Effort:** ___

---

### **TASK-016: Mainnet Launch** [P0] [@fullstack]

**Why Critical:** The goal of everything.

**Prerequisites:** ALL previous tasks complete

**Launch Checklist:**
- [ ] All audits passed
- [ ] Testnet stable for 4+ weeks
- [ ] 20+ validators ready
- [ ] Documentation complete
- [ ] Community notified (2 weeks advance)
- [ ] Exchange listings prepared (if applicable)
- [ ] Monitor mainnet for 72 hours post-launch
- [ ] Incident response team on standby

**Definition of Done:**
- [ ] Mainnet producing blocks
- [ ] Validators syncing correctly
- [ ] Governance functional (council + democracy)
- [ ] DEX operational with first pools
- [ ] Bridge operational (if audit passed)
- [ ] No critical issues in first 72 hours
- [ ] Community successfully using network

**Go/No-Go Criteria:**
- ✅ All audits passed
- ✅ Testnet uptime > 99.9% for 30 days
- ✅ All critical bugs fixed
- ✅ 20+ validators committed
- ✅ Community informed and supportive
- ✅ Rollback plan ready

**Estimated Effort:** 4 weeks (prep + monitoring)
**Actual Effort:** ___

---

## ❌ TASKS REMOVED FROM SCOPE

### **[CUT] TASK: Staking Precompile**
**Reason:** Low ROI. Staking already works. Few Solidity devs will stake from contracts.
**Time Saved:** 2 weeks

### **[CUT] TASK: DEX Precompile**
**Reason:** Redundant. If we have native DEX pallet, Solidity devs can use SDK.
**Time Saved:** 2 weeks

### **[CUT] TASK: Governance Precompile**
**Reason:** Governance is complex, not needed for EVM contracts initially.
**Time Saved:** 2 weeks

**Total Time Saved:** 6 weeks

---

## Timeline Summary

| Phase | Weeks | Key Deliverables |
|-------|-------|-----------------|
| Phase 1: Security | 1-4 | Fix critical vulnerabilities |
| Phase 2: Governance | 5-16 | Council + Democracy + Remove Sudo |
| Phase 3: DeFi | 17-28 | SDK + DEX + Stablecoins |
| Phase 4: Bridge & Testing | 29-40 | LayerZero + QA + Audit |
| Phase 5: Docs & Launch | 41-48 | Documentation + Mainnet |

**Total: 48 weeks (12 months)**

---

## Success Metrics

### Technical Metrics:
- ✅ Zero critical security vulnerabilities
- ✅ 85%+ test coverage
- ✅ 1000+ TPS capacity
- ✅ <2s block time
- ✅ 99.9% uptime

### DeFi Metrics:
- ✅ 10x cheaper gas than EVM alternatives
- ✅ $100K+ monthly DEX volume
- ✅ 5+ active trading pairs
- ✅ sUSD maintains 1:1 peg

### Developer Metrics:
- ✅ SDK downloaded 1000+ times
- ✅ 50+ developers building
- ✅ 10+ dApps deployed
- ✅ 95%+ developer satisfaction

### Decentralization Metrics:
- ✅ Sudo removed
- ✅ 20+ validators
- ✅ Council operational
- ✅ 5+ successful referenda

---

## Risk Management

### Technical Risks:
| Risk | Mitigation |
|------|-----------|
| Bridge exploit | Multi-sig + timelock + audit |
| DEX math errors | Extensive fuzzing + formal verification |
| Storage migration fails | Test on fork first, have rollback plan |
| Randomness still manipulable | Thorough testing, multiple implementations |

### Business Risks:
| Risk | Mitigation |
|------|-----------|
| Banking partnerships delayed | Build pallet now, launch when ready |
| Audit takes too long | Book audit firm early, have 2 backup firms |
| Validator dropout | Incentivize with good tokenomics |
| Low adoption | Focus on docs, examples, developer experience |

---

## Team Allocation (2-3 person team)

**Person 1 - Runtime Dev [@runtime]:**
- Phase 1: All security fixes (4 weeks)
- Phase 2: All governance (12 weeks)
- Phase 3: DEX + Stablecoins (12 weeks)
- Phase 4: Bridge pallet (2 weeks)
- Phase 5: Launch support (2 weeks)

**Person 2 - Frontend Dev [@frontend]:**
- Phase 2: Build SDK (4 weeks)
- Phase 3: Wait for DEX, then build SDK extensions (8 weeks)
- Phase 4: Build bridge UI (6 weeks)
- Phase 5: Documentation + tools (4 weeks)

**Person 3 - Full Stack [@fullstack] (optional, can be split):**
- Phase 1: Testing support (4 weeks)
- Phase 2: Governance testing (12 weeks)
- Phase 3: Integration testing (12 weeks)
- Phase 4: Bridge + comprehensive QA (12 weeks)
- Phase 5: Launch preparation (8 weeks)

**With Claude Code:**
- Use for boilerplate generation
- Use for test writing (can generate 70% of tests)
- Use for documentation generation
- Use for code review and refactoring
- Can potentially reduce effort by 30-40% on some tasks

---

## Next Steps

1. **Week 0:** Review this plan with team
2. **Week 0:** Set up development environment
3. **Week 0:** Create GitHub project board with all tasks
4. **Week 1:** Start TASK-001 (Remove insecure randomness)
5. **Weekly:** Team sync to track progress and adjust

**Let's build! 🚀**
