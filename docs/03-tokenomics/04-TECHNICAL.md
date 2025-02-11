# Selendra Tokenomics 2.0 - Technical Specification

## Token Specifications

### Supply Parameters
- Current Total Supply: 293,245,067.42 SEL
  - Mainnet: 227,887,400 SEL
  - BSC: 65,357,667.42 SEL
- Maximum Supply: π × 1,000,000,000 ≈ 3,141,592,653.59 SEL
- Time to Max Supply: ~135.63 years (Year 2160)
- Remaining Supply: 2,848,347,586.17 SEL

### Emission Configuration
```rust
parameter_types! {
    pub const YEARLY_INFLATION: Balance = 21_000_000 * TOKEN;
    pub const VALIDATOR_REWARD: Perbill = Perbill::from_percent(90);
    pub const TREASURY_SHARE: Perbill = Perbill::from_percent(10);
}
```

### Fee Structure
```rust
parameter_types! {
    // Base fee configuration
    pub const InitialBaseFee: Balance = 2_300_000_000;    // 2.3 Gwei
    pub const MinBaseFee: Balance = 100_000_000;         // 0.1 Gwei
    pub const MaxBaseFee: Balance = 10_000_000_000;      // 10 Gwei
    pub const BaseFeeAdjustment: Permill = Permill::from_percent(10);
    
    // Fee distribution
    pub const BurnRate: Permill = Permill::from_percent(30);          // 0.69 Gwei of 2.3
    pub const ValidatorFeeShare: Permill = Permill::from_percent(60); // 1.38 Gwei of 2.3
    pub const TreasuryFeeShare: Permill = Permill::from_percent(10);  // 0.23 Gwei of 2.3
    
    // Transaction weights
    pub const BaseExtrinsicWeight: Weight = 1;
    pub const ComplexExtrinsicMultiplier: u32 = 2;     // 4.6 Gwei
    pub const DAppExtrinsicMultiplier: u32 = 3;        // 6.9 Gwei
    
    // Block space targeting
    pub const TargetBlockFullness: Permill = Permill::from_percent(50);
    pub const AdjustmentPeriod: BlockNumber = 100;       // ~100 seconds
}
```

## Dynamic Fee Mechanism

### Base Fee Adjustment Algorithm
```rust
Next_Base_Fee = Current_Base_Fee * (1 + Adjustment_Factor)
where Adjustment_Factor = (Current_Usage - Target_Usage) * 0.1

// Boundaries
Minimum_Fee = 0.1 Gwei
Maximum_Fee = 10 Gwei
Maximum_Adjustment = 10% per period
```

### Network Load Scenarios

#### 1. Normal Load (50%)
```rust
Current_Usage = 50%
Target_Usage = 50%
Adjustment_Factor = 0%
Next_Base_Fee = 2.3 Gwei
```

#### 2. High Load (80%)
```rust
Current_Usage = 80%
Target_Usage = 50%
Adjustment_Factor = +3%
Fee_Progression = [2.369, 2.440, 2.513, 2.588, 2.666] // Gwei over 5 periods
```

#### 3. Low Load (20%)
```rust
Current_Usage = 20%
Target_Usage = 50%
Adjustment_Factor = -3%
Fee_Progression = [2.231, 2.164, 2.099, 2.036, 1.975] // Gwei over 5 periods
```

#### 4. Extreme Load (95%)
```rust
Current_Usage = 95%
Target_Usage = 50%
Raw_Adjustment = +4.5%
Fee_Progression = [2.4035, 2.5117, 2.6247, 2.7428, 2.8662] // Gwei over 5 periods
```

## Economic Models

### Validator Economics
```rust
// Base inflation rewards
Annual_Base_Reward = 18_900_000 SEL
Validator_Base_Share = Stake_Ratio * Annual_Base_Reward

// Dynamic fee rewards
Block_Fee_Reward = Block_Fees * 0.6
Validator_Fee_Share = (Stake_Ratio + Performance_Multiplier) * Block_Fee_Reward

// Total rewards
Total_Annual_Reward = Validator_Base_Share + Accumulated_Fee_Rewards
Effective_APY = Total_Annual_Reward / Total_Stake
```

### Treasury Economics
```rust
// Fixed income
Annual_Base_Income = 2_100_000 SEL

// Variable income
Daily_Fee_Income = Daily_Network_Fees * 0.1
Protocol_Revenue_Share = Protocol_Fees * 0.1

// Total income
Total_Annual_Income = Annual_Base_Income + (Daily_Fee_Income * 365) + Annual_Protocol_Revenue
```

## Vesting Schedules

### Ecosystem Development Fund (997,921,655.16 SEL)
- Initial Unlock: 199,584,331.03 SEL (20%)
- Vesting Period: 48 months
- Cliff: 6 months
- Monthly Release: 16,632,027.59 SEL

### Community Treasury (712,086,896.55 SEL)
- Initial Unlock: 71,208,689.66 SEL (10%)
- Vesting Period: 48 months
- Monthly Release: 13,351,629.31 SEL

### Future Reserve (284,834,758.62 SEL)
- Full Lock: 12 months
- Linear Vesting: 36 months
- Governance Controlled

## Network Metrics

### Growth Projections
```rust
struct YearlyTargets {
    tvl: Balance,
    active_validators: u32,
    daily_transactions: u32,
}

const YEAR_1_TARGETS = YearlyTargets {
    tvl: 100_000_000 * DOLLARS,
    active_validators: 100,
    daily_transactions: 100_000,
};

const YEAR_3_TARGETS = YearlyTargets {
    tvl: 500_000_000 * DOLLARS,
    active_validators: 200,
    daily_transactions: 500_000,
};
```

### Market Stability Parameters
```rust
const BUYBACK_THRESHOLD: Balance = 50 * CENTS;  // $0.50
const MAX_DAILY_BUYBACK: Balance = 1_000_000 * DOLLARS;  // $1M
const LIQUIDITY_TARGET: Balance = 10_000_000 * DOLLARS;  // $10M
const MAX_PRICE_IMPACT: Percent = Percent::from_percent(2);
```
