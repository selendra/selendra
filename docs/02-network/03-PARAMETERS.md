# Network Parameters

## System Parameters

### 1. Block Parameters
```rust
pub struct BlockParams {
    /// Maximum block size in bytes
    pub max_block_size: u32,
    /// Maximum block weight
    pub max_block_weight: Weight,
    /// Target block time in seconds
    pub target_block_time: u32,
    /// Maximum block transactions
    pub max_block_transactions: u32,
}

impl Default for BlockParams {
    fn default() -> Self {
        Self {
            max_block_size: 5_242_880,    // 5MB
            max_block_weight: 1_000_000,
            target_block_time: 6,          // 6 seconds
            max_block_transactions: 10_000,
        }
    }
}
```

### 2. Network Parameters
```rust
pub struct NetworkParams {
    /// Maximum peers
    pub max_peers: u32,
    /// Minimum peers
    pub min_peers: u32,
    /// Peer timeout in seconds
    pub peer_timeout: u32,
    /// Maximum pending transactions
    pub max_pending_transactions: u32,
}

impl Default for NetworkParams {
    fn default() -> Self {
        Self {
            max_peers: 50,
            min_peers: 10,
            peer_timeout: 60,
            max_pending_transactions: 8192,
        }
    }
}
```

## Consensus Parameters

### 1. BABE Parameters
```rust
pub struct BabeParams {
    /// Epoch duration in slots
    pub epoch_length: u64,
    /// Slot duration in milliseconds
    pub slot_duration: u64,
    /// Primary slot probability
    pub c: (u64, u64),
    /// Secondary slots enabled
    pub secondary_slots: bool,
}

impl Default for BabeParams {
    fn default() -> Self {
        Self {
            epoch_length: 2400,
            slot_duration: 6000,
            c: (1, 4),
            secondary_slots: true,
        }
    }
}
```

### 2. GRANDPA Parameters
```rust
pub struct GrandpaParams {
    /// Voting round duration
    pub round_duration: Duration,
    /// Maximum round number
    pub max_round_number: u64,
    /// Vote threshold
    pub threshold: (u64, u64),
}

impl Default for GrandpaParams {
    fn default() -> Self {
        Self {
            round_duration: Duration::from_secs(60),
            max_round_number: 1000,
            threshold: (2, 3),  // 2/3 supermajority
        }
    }
}
```

## Staking Parameters

### 1. Validator Parameters
```rust
pub struct ValidatorParams {
    /// Minimum stake required
    pub minimum_stake: Balance,
    /// Maximum validators
    pub max_validators: u32,
    /// Commission range
    pub commission_range: (Perbill, Perbill),
    /// Slash percentages
    pub slash_percentages: SlashPercentages,
}

impl Default for ValidatorParams {
    fn default() -> Self {
        Self {
            minimum_stake: 10_000 * DOLLARS,
            max_validators: 100,
            commission_range: (
                Perbill::from_percent(0),
                Perbill::from_percent(100)
            ),
            slash_percentages: SlashPercentages::default(),
        }
    }
}
```

### 2. Era Parameters
```rust
pub struct EraParams {
    /// Era duration in sessions
    pub era_duration: SessionIndex,
    /// Session duration in blocks
    pub session_duration: BlockNumber,
    /// Reward curve
    pub reward_curve: PiecewiseLinear<'static>,
}

impl Default for EraParams {
    fn default() -> Self {
        Self {
            era_duration: 24,          // 24 sessions per era
            session_duration: 600,      // 600 blocks per session
            reward_curve: DEFAULT_CURVE,
        }
    }
}
```

## Transaction Parameters

### 1. Fee Parameters
```rust
pub struct FeeParams {
    /// Base fee per transaction
    pub base_fee: Balance,
    /// Fee multiplier
    pub fee_multiplier: Multiplier,
    /// Length fee per byte
    pub length_fee: Balance,
    /// Weight fee per unit
    pub weight_fee: Balance,
}

impl Default for FeeParams {
    fn default() -> Self {
        Self {
            base_fee: DOLLARS / 100,    // 0.01 SEL
            fee_multiplier: Multiplier::saturating_from_rational(1, 1),
            length_fee: DOLLARS / 1000,  // 0.001 SEL per byte
            weight_fee: DOLLARS / 1000,  // 0.001 SEL per weight
        }
    }
}
```

### 2. Transaction Limits
```rust
pub struct TransactionLimits {
    /// Maximum transaction size
    pub max_tx_size: u32,
    /// Maximum transaction weight
    pub max_tx_weight: Weight,
    /// Maximum transaction lifetime
    pub max_tx_lifetime: BlockNumber,
}

impl Default for TransactionLimits {
    fn default() -> Self {
        Self {
            max_tx_size: 262_144,    // 256KB
            max_tx_weight: 100_000,
            max_tx_lifetime: 100,     // 100 blocks
        }
    }
}
```

## Runtime Parameters

### 1. Storage Parameters
```rust
pub struct StorageParams {
    /// Maximum storage size
    pub max_storage_size: u32,
    /// Storage deposit
    pub storage_deposit: Balance,
    /// Deposit per byte
    pub deposit_per_byte: Balance,
}

impl Default for StorageParams {
    fn default() -> Self {
        Self {
            max_storage_size: 1_073_741_824,  // 1GB
            storage_deposit: DOLLARS,
            deposit_per_byte: DOLLARS / 1_000_000,
        }
    }
}
```

### 2. Execution Parameters
```rust
pub struct ExecutionParams {
    /// Maximum code size
    pub max_code_size: u32,
    /// Maximum stack size
    pub max_stack_size: u32,
    /// Maximum memory pages
    pub max_memory_pages: u32,
}

impl Default for ExecutionParams {
    fn default() -> Self {
        Self {
            max_code_size: 1_048_576,  // 1MB
            max_stack_size: 1_048_576,  // 1MB
            max_memory_pages: 16,       // 1MB (64KB per page)
        }
    }
}
```

## Governance Parameters

### 1. Democracy Parameters
```rust
pub struct DemocracyParams {
    /// Launch period in blocks
    pub launch_period: BlockNumber,
    /// Voting period in blocks
    pub voting_period: BlockNumber,
    /// Enactment period in blocks
    pub enactment_period: BlockNumber,
    /// Minimum deposit
    pub minimum_deposit: Balance,
}

impl Default for DemocracyParams {
    fn default() -> Self {
        Self {
            launch_period: 28_800,     // 2 days
            voting_period: 28_800,     // 2 days
            enactment_period: 28_800,  // 2 days
            minimum_deposit: 100 * DOLLARS,
        }
    }
}
```

### 2. Council Parameters
```rust
pub struct CouncilParams {
    /// Maximum members
    pub max_members: u32,
    /// Term duration in blocks
    pub term_duration: BlockNumber,
    /// Candidacy bond
    pub candidacy_bond: Balance,
    /// Voting bond
    pub voting_bond: Balance,
}

impl Default for CouncilParams {
    fn default() -> Self {
        Self {
            max_members: 13,
            term_duration: 201_600,    // 14 days
            candidacy_bond: 1000 * DOLLARS,
            voting_bond: 100 * DOLLARS,
        }
    }
}
```
