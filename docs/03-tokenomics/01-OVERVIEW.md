# Selendra Tokenomics 2.0 - Simple Guide

> For technical details, please see [TOKENOMICS_TECHNICAL.md](TOKENOMICS_TECHNICAL.md)

## Understanding Selendra's Token Economy

### The Basics

#### What is SEL?
- SEL is Selendra's native token
- Used for transaction fees, staking, and governance
- Available on Selendra Mainnet and Binance Smart Chain (BSC)
- For cross-chains future, it will be on other networks

#### Current Status
- Total Supply: 293.24 million SEL
  - 227.89 million on Selendra Mainnet
  - 65.36 million on BSC
- Maximum Supply: 3.14 billion SEL (π × 1 billion)
- Will reach maximum supply around year 2160

### How New Tokens are Created

#### Annual Creation
- 21 million new SEL created each year
- Distribution:
  - 90% (18.9 million) goes to validators for securing the network
  - 10% (2.1 million) goes to treasury for development

### Transaction Fees Explained

#### Basic Transaction Cost
- Standard transaction: 2.3 Gwei
- Complex transaction: 4.6 Gwei
- DApp interaction: 6.9 Gwei

#### Where Do Fees Go?
 For every 2.3 Gwei fee:
- 0.69 Gwei (30%) is burned (removed from supply)
- 1.38 Gwei (60%) goes to validators
- 0.23 Gwei (10%) goes to treasury

#### Dynamic Fees
Fees automatically adjust based on network usage:
- Network busy (lots of transactions) → Fees go up
- Network quiet (few transactions) → Fees go down
- Adjusts every ~100 seconds to maintain optimal network usage

### Token Distribution

#### 1. Ecosystem Development (35% of Remaining)
- DApp Grants: Supporting new projects
- Developer Incentives: Attracting developers
- Ecosystem Partnerships: Growing the network

#### 2. Community Treasury (25% of Remaining)
- Community Projects: Supporting community initiatives
- Protocol Upgrades: Improving the network

#### 3. Staking Rewards (30% of Remaining)
- Validator Incentives: Rewarding network security
- Long-term Staking: Encouraging token holding

#### 4. Future Reserve (10% of Remaining)
- Emergency Fund: For unexpected events
- Strategic Partnerships: For future opportunities

### Staking Benefits

#### Validator Rewards
- Base rewards from annual emission
- Extra rewards from transaction fees
- Performance bonuses for good behavior

#### Long-term Staking
- Lock tokens for 6 months: +10% rewards
- Lock tokens for 12 months: +25% rewards
- Lock tokens for 24 months: +50% rewards

### Future Growth

#### Year 1 Goals
- $100M Total Value Locked
- 100 active validators
- 100,000 daily transactions

#### Year 3 Goals
- $500M Total Value Locked
- 200 active validators
- 500,000 daily transactions

### Market Protection
- Network buys back SEL if price falls below $0.50
- Maximum daily buyback: $1 million
- Aims to maintain $10 million in liquidity

## Want to Learn More?
For technical details about formulas, exact numbers, and implementation specifics, please check [TOKENOMICS_TECHNICAL.md](TOKENOMICS_TECHNICAL.md)

## Current Token State

### Mainnet (Native Chain)
- **Token Name**: Selendra (SEL)
- **Decimals**: 18
- **Current Supply**: 227.8874 MSEL (227,887,400 SEL)
- **Holders**: ~3,000
- **Distribution**: ~30M SEL in circulation

### BSC Chain
- **Token Supply**: 65,357,667.42 SEL
- **Holders**: 32,568
- **Total Transfers**: 37,588
- **Chain**: Binance Smart Chain

## Token Migration Strategy

### Phase 1: Supply Consolidation
1. **Current Total Supply**: 293,245,067.42 SEL
   - Mainnet: 227,887,400 SEL
   - BSC: 65,357,667.42 SEL

2. **Supply Model Implementation**
   - Maximum supply: π × 1,000,000,000 ≈ 3,141,592,653.59 SEL
   - Current supply: 293,245,067.42 SEL (9.33% of max)
   - Remaining potential: 2,848,347,586.17 SEL

### Mathematical Significance
- Using π as a supply multiplier provides several advantages:
  1. Natural mathematical constant that's transcendental and irrational
  2. Creates a unique and memorable total supply
  3. Symbolizes continuous and infinite growth potential while maintaining a cap
  4. Represents the ratio of a circle's circumference to diameter, reflecting cyclical nature of economics

### Token Denominations
```rust
1 SEL = 1,000 MILLI_SEL
1 MILLI_SEL = 1,000 MICRO_SEL
1 MICRO_SEL = 1,000 NANO_SEL
1 NANO_SEL = 1,000 PICO_SEL
```

## Token Distribution

## Token Allocation

### Current Distribution (293,245,067.42 SEL)
1. **Circulating Supply** - 95,357,667.42 SEL (32.52% of current supply)
   - BSC Chain: 65,357,667.42 SEL
   - Mainnet Circulation: ~30M SEL

2. **Reserved Supply** - 197,887,400 SEL (67.48% of current supply)
   - Currently on Mainnet: 197,887,400 SEL
   - Reserved for ecosystem development and staking

### Future Allocation Strategy (Remaining Supply: 2,848,347,586.17 SEL)

1. **Ecosystem Development (35% of Remaining)**
   - DApp Grants: 427,252,137.93 SEL (15%)
   - Developer Incentives: 341,801,710.34 SEL (12%)
   - Ecosystem Partnerships: 227,867,806.89 SEL (8%)

2. **Community Treasury (25% of Remaining)**
   - Community Projects: 427,252,137.93 SEL (15%)
   - Protocol Upgrades: 284,834,758.62 SEL (10%)

3. **Staking Rewards (30% of Remaining)**
   - Validator Incentives: 569,669,517.23 SEL (20%)
   - Long-term Staking Pool: 284,834,758.62 SEL (10%)

4. **Future Reserve (10% of Remaining)**
   - Emergency Fund: 142,417,379.31 SEL (5%)
   - Strategic Partnerships: 142,417,379.31 SEL (5%)

Note: Percentages are of the remaining supply (π × 1B - current supply)

## Emission Schedule

### Current State
- **Total Supply**: 293,245,067.42 SEL
- **Circulating Supply**: 95,357,667.42 SEL (32.52%)
- **Reserved**: 197,887,400 SEL (67.48%)

### Future Emission (New Model)

#### Staking Rewards Parameters
```rust
parameter_types! {
    // Initial annual emission rate: 5%
    pub const InitialEmissionRate: Permill = Permill::from_percent(5);
    // Minimum annual emission rate: 1%
    pub const MinimumEmissionRate: Permill = Permill::from_percent(1);
    // Emission reduction rate: 20% per year
    pub const EmissionReductionRate: Permill = Permill::from_percent(20);
    // Maximum supply cap (π × 1B)
    pub const MaximumSupply: Balance = 3_141_592_653_590_000_000_000_000_000; // π × 1B SEL
}
```

### Hybrid Economic Model (2025 Onwards)

#### 1. Fixed Annual Emission
- Total yearly emission: 21,000,000 SEL
- Base validator rewards: 18,900,000 SEL (90%)
- Treasury allocation: 2,100,000 SEL (10%)

#### 2. Dynamic Fee Structure
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

    // Example transaction costs
    // Base transaction:     2.3 Gwei = InitialBaseFee * BaseExtrinsicWeight
    // Complex transaction: 4.6 Gwei = InitialBaseFee * ComplexExtrinsicMultiplier
    // DApp interaction:    6.9 Gwei = InitialBaseFee * DAppExtrinsicMultiplier
}
```

#### 3. Validator Revenue Streams
1. **Fixed Inflation** (Base Layer)
   - Predictable 18.9M SEL annually
   - Distributed proportionally to stake
   - Ensures minimum validator revenue

2. **Dynamic Fees** (Performance Layer)
   - Transaction fee share (60%)
   - Priority fees for faster inclusion
   - MEV rewards from block production
   - Performance-based multipliers

#### 4. Treasury Funding
1. **Fixed Allocation**
   - 2.1M SEL annually from inflation
   - Predictable base for operations

2. **Variable Income**
   - 10% of all transaction fees
   - Protocol revenue share
   - Special transaction fees

#### Supply Timeline (from 2025)
- Starting Supply: 293,245,067.42 SEL
- Maximum Supply: 3,141,592,653.59 SEL (π × 1B)
- Remaining Supply: 2,848,347,586.17 SEL
- Years to Max Supply: ~135.63 years (Year 2160)

#### Inflation Rate Decrease
- 2025 (293.24M supply): ~7.16% annual inflation
- 2035 (503.24M supply): ~4.17% annual inflation
- 2045 (713.24M supply): ~2.94% annual inflation
- 2055 (923.24M supply): ~2.27% annual inflation
- At 1B supply: ~2.1% annual inflation
- At max supply (π × 1B): ~0.67% annual inflation

#### Long-term Sustainability
- Fixed emission creates predictable validator rewards
- Natural reduction in inflation rate as supply grows
- Governance can adjust parameters if needed
- Maximum supply reached around year 2160

### Post-Maximum Supply Strategy (After 2160)

#### 1. Transaction Fee Economics
```rust
parameter_types! {
    // After max supply, 100% of fees go to validators and treasury
    pub const ValidatorFeeShare: Permill = Permill::from_percent(90);
    pub const TreasuryFeeShare: Permill = Permill::from_percent(10);
    
    // Dynamic fee adjustment based on network usage
    pub const BaseFeeAdjustment: Permill = Permill::from_percent(10);
    pub const MaxFeeIncrease: Permill = Permill::from_percent(100);
}
```

#### 2. Validator Rewards
- Transition from inflation-based to fee-based rewards
- Priority fees for validator compensation
- Performance-based reward multipliers
- MEV (Maximal Extractable Value) sharing

#### 3. Deflationary Mechanisms
- Continue 30% transaction fee burn
- Implement periodic buy-back and burn
- Slashing penalties permanently reduce supply
- Lost/unused tokens reduce circulating supply

#### 4. Economic Security
- Minimum validator stake requirements
- Dynamic fee markets ensure network security
- Treasury-funded security incentives
- Cross-chain security bonds

#### 5. Governance Authority
- DAO control over fee parameters
- Ability to adjust reward mechanisms
- Emergency response capabilities
- Protocol upgrade management

## Distribution Strategy and Vesting

### 1. Ecosystem Development Fund (997,921,655.16 SEL)

#### Initial Unlock (20%): 199,584,331.03 SEL
- Available at TGE for immediate ecosystem needs
- Focused on critical partnerships and initial development

#### Vesting Schedule (80%): 798,337,324.13 SEL
- Duration: 48 months
- Cliff: 6 months
- Monthly Release: 16,632,027.59 SEL
- Special Provisions:
  - Emergency unlock requires 75% validator approval
  - Maximum 10% acceleration in any quarter

#### Allocation Timeline
1. **DApp Grants** (427,252,137.93 SEL)
   - Year 1: 30% (128,175,641.38 SEL)
   - Year 2: 30% (128,175,641.38 SEL)
   - Year 3: 20% (85,450,427.59 SEL)
   - Year 4: 20% (85,450,427.59 SEL)

2. **Developer Incentives** (341,801,710.34 SEL)
   - Quarterly releases
   - Performance-based distribution
   - Hackathon rewards pool

3. **Ecosystem Partnerships** (227,867,806.89 SEL)
   - Strategic partner allocation
   - Integration incentives
   - Market making reserves

### 2. Community Treasury (712,086,896.55 SEL)

#### Initial Unlock (10%): 71,208,689.66 SEL
- Governance bootstrapping
- Initial community initiatives

#### Vesting Schedule (90%): 640,878,206.89 SEL
- Linear vesting: 48 months
- Monthly Release: 13,351,629.31 SEL
- Governance controlled

#### Usage Timeline
1. **Community Projects** (427,252,137.93 SEL)
   - Year 1: 25% (106,813,034.48 SEL)
   - Year 2: 25% (106,813,034.48 SEL)
   - Year 3: 25% (106,813,034.48 SEL)
   - Year 4: 25% (106,813,034.48 SEL)

2. **Protocol Upgrades** (284,834,758.62 SEL)
   - Quarterly budget allocation
   - Emergency reserve: 20%
   - Development grants: 80%

### 3. Staking Rewards (854,504,275.85 SEL)

#### Emission Schedule
- Initial Annual Rate: 21,000,000 SEL
- Validator Share: 90% (18,900,000 SEL)
- Treasury Share: 10% (2,100,000 SEL)

#### Distribution Model
1. **Validator Incentives** (569,669,517.23 SEL)
   ```rust
   parameter_types! {
       pub const BaseReward: Balance = 100 * TOKEN;
       pub const PerformanceMultiplier: Permill = Permill::from_percent(150);
       pub const MinimumStake: Balance = 50_000 * TOKEN;
   }
   ```

2. **Long-term Staking** (284,834,758.62 SEL)
   - Lockup periods: 6, 12, 24 months
   - Bonus rewards: 10%, 25%, 50%
   - Early unstaking penalty: 25%

### 4. Future Reserve (284,834,758.62 SEL)

#### Vesting Schedule
- Full lock: 12 months
- Linear vesting: 36 months
- Governance controlled release

#### Strategic Usage
1. **Emergency Fund** (142,417,379.31 SEL)
   - Black swan events
   - Security incidents
   - Market stabilization

2. **Strategic Partnerships** (142,417,379.31 SEL)
   - CEX listings
   - Protocol integrations
   - Infrastructure expansion

## Economic Models

### 1. Hybrid Validator Economics
```rust
// Base inflation rewards (fixed)
Annual_Base_Reward = 18_900_000 SEL
Validator_Base_Share = Stake_Ratio * Annual_Base_Reward

// Dynamic fee rewards
Block_Fee_Reward = Block_Fees * 0.6  // 60% to validators
Validator_Fee_Share = (Stake_Ratio + Performance_Multiplier) * Block_Fee_Reward

// Total validator rewards
Total_Annual_Reward = Validator_Base_Share + Accumulated_Fee_Rewards
Effective_APY = Total_Annual_Reward / Total_Stake
```

### 2. Dynamic Fee Mechanism

#### Base Fee Adjustment Formula
```rust
// Base fee adjustment every 100 blocks (~100 seconds)
Next_Base_Fee = Current_Base_Fee * (1 + Adjustment_Factor)
where Adjustment_Factor = (Current_Usage - Target_Usage) * 0.1

// Fee boundaries
Base_Fee = 2.3 Gwei      // Initial base fee
Minimum_Fee = 0.1 Gwei   // Lower bound
Maximum_Fee = 10 Gwei    // Upper bound
Maximum_Adjustment = 10% per period
```

#### Dynamic Fee Scenarios

1. **Normal Network Load (50% Usage)**
```rust
// Target usage met exactly
Current_Usage = 50%
Target_Usage = 50%
Adjustment_Factor = (50% - 50%) * 0.1 = 0%
Next_Base_Fee = 2.3 Gwei * (1 + 0) = 2.3 Gwei
```

2. **High Network Load (80% Usage)**
```rust
// Network congestion triggers fee increase
Current_Usage = 80%
Target_Usage = 50%
Adjustment_Factor = (80% - 50%) * 0.1 = +3%
Next_Base_Fee = 2.3 Gwei * (1 + 0.03) = 2.369 Gwei

// If congestion continues for 5 periods
Period 1: 2.369 Gwei
Period 2: 2.440 Gwei
Period 3: 2.513 Gwei
Period 4: 2.588 Gwei
Period 5: 2.666 Gwei
```

3. **Low Network Load (20% Usage)**
```rust
// Low usage triggers fee decrease
Current_Usage = 20%
Target_Usage = 50%
Adjustment_Factor = (20% - 50%) * 0.1 = -3%
Next_Base_Fee = 2.3 Gwei * (1 - 0.03) = 2.231 Gwei

// If low usage continues for 5 periods
Period 1: 2.231 Gwei
Period 2: 2.164 Gwei
Period 3: 2.099 Gwei
Period 4: 2.036 Gwei
Period 5: 1.975 Gwei
```

4. **Extreme Network Load (95% Usage)**
```rust
// Heavy congestion triggers maximum adjustment
Current_Usage = 95%
Target_Usage = 50%
Raw_Adjustment = (95% - 50%) * 0.1 = +4.5%
Capped_Adjustment = min(4.5%, 10%) = 4.5%
Next_Base_Fee = 2.3 Gwei * (1 + 0.045) = 2.4035 Gwei

// If extreme usage continues for 5 periods
Period 1: 2.4035 Gwei
Period 2: 2.5117 Gwei
Period 3: 2.6247 Gwei
Period 4: 2.7428 Gwei
Period 5: 2.8662 Gwei
```

#### Fee Distribution (at 2.3 Gwei base fee)
```rust
Burned_Amount = Fee * 0.3     // 0.69 Gwei burned
Validator_Share = Fee * 0.6    // 1.38 Gwei to validators
Treasury_Share = Fee * 0.1     // 0.23 Gwei to treasury
```

### 3. Treasury Economics
```rust
// Fixed income
Annual_Base_Income = 2_100_000 SEL

// Variable income
Daily_Fee_Income = Daily_Network_Fees * 0.1
Protocol_Revenue_Share = Protocol_Fees * 0.1

// Total treasury income
Total_Annual_Income = Annual_Base_Income + (Daily_Fee_Income * 365) + Annual_Protocol_Revenue
```

### 4. Growth Projections

#### Year 1
- Target TVL: $100M
- Active Validators: 100
- Daily Transactions: 100,000

#### Year 2
- Target TVL: $250M
- Active Validators: 150
- Daily Transactions: 250,000

#### Year 3
- Target TVL: $500M
- Active Validators: 200
- Daily Transactions: 500,000

### 5. Market Stability
- Buy-back threshold: $0.50
- Maximum daily buy-back: $1M
- Liquidity depth target: $10M
- Price impact limit: 2%

## Enhanced Token Utility

### 1. Core Network Functions
- **Transaction Processing**
  - Standard transfers
  - Smart contract execution
  - Cross-chain operations
  - Priority lanes

### 2. Advanced Governance
- **Proposal Tiers**
  - Standard: 1,000 SEL
  - Important: 10,000 SEL
  - Critical: 100,000 SEL
  - Constitutional: 1,000,000 SEL
- **Voting Mechanisms**
  - Quadratic voting
  - Time-weighted voting power
  - Delegation systems
  - Conviction voting

### 3. Enterprise Features
- **Resource Allocation**
  - Dedicated processing lanes
  - Guaranteed block space
  - Priority transaction processing
  - Custom smart contract environments

### 4. Economic Participation
- **Liquidity Provision**
  - AMM liquidity mining
  - Stable pool incentives
  - Cross-chain bridge rewards
- **Network Services**
  - Oracle operation
  - Archive node running
  - RPC service provision
  - Infrastructure hosting

### 3. Staking Mechanics
- Minimum Stake: 5,000 SEL
- Validator Requirements: 50,000 SEL
- Maximum Validators: 100 initially, scaling to 200
- Delegation Minimum: 100 SEL

### 4. Economic Adjustments
```rust
parameter_types! {
    // Inflation adjustment based on staking ratio
    pub const IdealStakingRatio: Permill = Permill::from_percent(60);
    pub const MaxInflationRate: Permill = Permill::from_percent(15);
    pub const MinInflationRate: Permill = Permill::from_percent(2);
}
```

## Advanced Token Mechanics

### Dynamic Fee Structure
1. **Base Transaction Fee**
   - 30% burned automatically
   - 60% to validators/stakers
   - 10% to treasury

2. **Premium Services**
   - Smart contract deployment fees
   - Cross-chain bridge fees
   - Priority transaction fees
   - 50% burned, 50% to ecosystem development

### Deflationary Mechanisms
1. **Automated Market Operations**
   - Dynamic buy-back based on market conditions
   - Treasury-backed price floor mechanism
   - Algorithmic supply adjustment

2. **Burn Catalysts**
   - Gas fee multiplier during high congestion
   - Name registration burns
   - Feature activation burns
   - Governance proposal deposits (partial burn)

### Supply Optimization
1. **Adaptive Issuance**
   ```rust
   parameter_types! {
       // Target staking ratio for network security
       pub const TargetStakingRatio: Permill = Permill::from_percent(60);
       // Emission adjustment factor
       pub const EmissionAdjustment: Permill = Permill::from_percent(10);
       // Maximum emission rate change per epoch
       pub const MaxEmissionChange: Permill = Permill::from_percent(25);
   }
   ```

2. **Validator Economics**
   - Progressive rewards based on stake duration
   - Performance-based multipliers
   - Community-voted commission adjustments

## Economic Security and Incentives

### Enhanced Slashing Mechanism
1. **Uptime Requirements**
   - Warning threshold: 98% uptime
   - Minor slash: 95-98% uptime (0.1%)
   - Major slash: <95% uptime (0.5%)
   - Critical slash: <90% uptime (2%)

2. **Security Violations**
   - Double-signing: Progressive 5-20% based on stake size
   - Invalid blocks: 2-10% based on impact
   - Consensus violations: 10-30% based on severity

3. **Recovery Mechanism**
   - Partial slash recovery through good behavior
   - Community governance slash reversals
   - Automatic slash insurance for honest mistakes

### Positive Incentives
1. **Validator Rewards**
   - Base rewards: 18.9M SEL annually
   - Performance multipliers: Up to 50% bonus
   - Long-term staking bonuses

2. **Nominator Benefits**
   - Compound interest options
   - Early supporter multipliers
   - Governance voting weight

3. **Developer Incentives**
   - Smart contract deployment rebates
   - Gas fee sharing program
   - Builder mining program

### Validator Economics
- Commission Range: 5-20%
- Maximum Commission Change: 1% per epoch
- Unbonding Period: 28 days

## Treasury Management

### Funding Sources
1. Transaction Fees: 30% burned
2. Slashing Penalties: 50% burned
3. Network Revenue: 20% burned

### Allocation Guidelines
1. Development: 40%
2. Marketing: 20%
3. Community: 20%
4. Reserves: 20%

## Governance Parameters

### Proposal Thresholds
- Standard Proposal: 1,000 SEL
- Emergency Proposal: 10,000 SEL
- Constitutional Change: 100,000 SEL

### Voting
- Minimum Voting Period: 7 days
- Maximum Voting Period: 14 days
- Quorum: 30% of staked tokens
- Super-majority: 66% for critical changes

## Economic Sustainability

### Revenue Streams
1. Transaction Fees
2. Smart Contract Deployment Fees
3. Premium Service Fees
4. Cross-chain Bridge Fees

### Cost Control
1. Dynamic Fee Adjustment
2. Resource Usage Optimization
3. Validator Reward Scaling

## Success Metrics

### Network Health
- Staking Ratio: Target 60%
- Active Validators: >80% of maximum
- Transaction Volume Growth

### Economic Stability
- Price Stability
- Treasury Growth
- Development Funding Sustainability

### Community Engagement
- Governance Participation
- Developer Activity
- DApp Deployment Rate

## Implementation Timeline

### Phase 1: Foundation (Months 1-6)
- Token Generation Event
- Initial Exchange Listings
- Staking System Launch

### Phase 2: Growth (Months 7-12)
- Governance System Activation
- Treasury Management
- Community Programs

### Phase 3: Maturity (Year 2+)
- Full DAO Transition
- Advanced Economic Features
- Cross-chain Integration

---

This tokenomics design aims to create a sustainable, growth-oriented economy for Selendra while maintaining security, decentralization, and community engagement. Regular reviews and adjustments through governance will ensure the system remains effective and adaptable.
