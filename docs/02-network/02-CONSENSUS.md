# Selendra Consensus Mechanism

## Overview

Selendra uses a hybrid consensus mechanism combining:
- Nominated Proof of Stake (NPoS)
- GRANDPA Finality
- BABE Block Production
- Validator Selection

## Nominated Proof of Stake (NPoS)

### 1. Validator Selection
```rust
pub struct ValidatorSet {
    /// Current active validators
    active_validators: Vec<AccountId>,
    /// Queued validators for next era
    queued_validators: Vec<AccountId>,
    /// Minimum stake required
    minimum_stake: Balance,
    /// Maximum number of validators
    max_validators: u32,
}

impl ValidatorSet {
    /// Select validators for next era
    pub fn select_validators(
        &mut self,
        nominations: Vec<Nomination>,
    ) -> Result<Vec<AccountId>, Error> {
        // Sort nominations by stake
        let mut sorted_nominations = nominations.clone();
        sorted_nominations.sort_by(|a, b| b.total_stake().cmp(&a.total_stake()));
        
        // Select top validators
        let selected = sorted_nominations
            .iter()
            .take(self.max_validators as usize)
            .map(|nom| nom.validator_id.clone())
            .collect();
            
        Ok(selected)
    }
}
```

### 2. Staking Mechanism
```rust
pub struct StakingSystem {
    /// Validator stakes
    validator_stakes: BTreeMap<AccountId, Balance>,
    /// Nominator stakes
    nominator_stakes: BTreeMap<AccountId, Vec<(AccountId, Balance)>>,
    /// Era information
    current_era: EraIndex,
    /// Reward configuration
    reward_config: RewardConfig,
}

impl StakingSystem {
    /// Stake tokens for a validator
    pub fn stake(
        &mut self,
        staker: AccountId,
        validator: AccountId,
        amount: Balance,
    ) -> Result<(), Error> {
        // Validate stake amount
        ensure!(amount >= self.minimum_stake, Error::InsufficientStake);
        
        // Update stakes
        match self.nominator_stakes.get_mut(&staker) {
            Some(nominations) => {
                nominations.push((validator.clone(), amount));
            }
            None => {
                self.nominator_stakes.insert(
                    staker.clone(),
                    vec![(validator.clone(), amount)]
                );
            }
        }
        
        // Update validator total stake
        *self.validator_stakes.entry(validator).or_default() += amount;
        
        Ok(())
    }
}
```

## GRANDPA Finality

### 1. Block Finalization
```rust
pub struct GrandpaFinality {
    /// Current round
    round: RoundNumber,
    /// Pending votes
    pending_votes: BTreeMap<BlockHash, Vec<Vote>>,
    /// Finalized blocks
    finalized_blocks: Vec<BlockHash>,
    /// Validator set
    validator_set: ValidatorSet,
}

impl GrandpaFinality {
    /// Process a new vote
    pub fn process_vote(
        &mut self,
        vote: Vote,
    ) -> Result<(), Error> {
        // Validate vote
        self.validate_vote(&vote)?;
        
        // Add vote to pending
        self.pending_votes
            .entry(vote.block_hash)
            .or_default()
            .push(vote);
            
        // Check for finality
        self.try_finalize(vote.block_hash);
        
        Ok(())
    }
    
    /// Try to finalize a block
    fn try_finalize(&mut self, block_hash: BlockHash) {
        let votes = self.pending_votes.get(&block_hash).unwrap_or(&vec![]);
        
        // Check for supermajority
        if self.has_supermajority(votes) {
            self.finalize_block(block_hash);
        }
    }
}
```

### 2. Vote Aggregation
```rust
pub struct VoteAggregator {
    /// Votes by round
    votes: BTreeMap<RoundNumber, RoundVotes>,
    /// Current round state
    round_state: RoundState,
    /// Threshold calculator
    threshold: ThresholdCalculator,
}

impl VoteAggregator {
    /// Add a new vote
    pub fn add_vote(
        &mut self,
        vote: SignedVote,
    ) -> Result<(), Error> {
        // Validate signature
        self.validate_signature(&vote)?;
        
        // Add to round votes
        self.votes
            .entry(vote.round)
            .or_default()
            .add_vote(vote);
            
        // Check completion
        self.check_round_completion(vote.round);
        
        Ok(())
    }
    
    /// Check if round is complete
    fn check_round_completion(&mut self, round: RoundNumber) {
        let votes = self.votes.get(&round).unwrap();
        
        if votes.has_threshold(self.threshold.get_threshold()) {
            self.complete_round(round);
        }
    }
}
```

## BABE Block Production

### 1. Slot Assignment
```rust
pub struct BabeSlotAssignment {
    /// Current epoch
    epoch: EpochIndex,
    /// Slot number
    slot: SlotNumber,
    /// Random seed
    random_seed: Randomness,
    /// Validator set
    validator_set: ValidatorSet,
}

impl BabeSlotAssignment {
    /// Check if validator is primary for slot
    pub fn is_primary(
        &self,
        validator: &AccountId,
    ) -> bool {
        let threshold = self.calculate_threshold(validator);
        let value = self.slot_value(validator);
        
        value < threshold
    }
    
    /// Calculate VRF output for slot
    fn slot_value(&self, validator: &AccountId) -> u64 {
        let input = (
            self.random_seed,
            self.slot,
            validator,
        );
        
        vrf::make_bytes(input)
    }
}
```

### 2. Block Production
```rust
pub struct BabeBlockProducer {
    /// Current state
    state: BlockProductionState,
    /// Transaction pool
    tx_pool: TransactionPool,
    /// Block builder
    block_builder: BlockBuilder,
}

impl BabeBlockProducer {
    /// Produce a new block
    pub fn produce_block(
        &mut self,
        slot: SlotNumber,
    ) -> Result<Block, Error> {
        // Check if we should produce
        if !self.should_produce_block(slot) {
            return Err(Error::NotMySlot);
        }
        
        // Build block
        let mut block = self.block_builder.build_block()?;
        
        // Add transactions
        self.add_transactions(&mut block)?;
        
        // Finalize and sign
        self.finalize_block(&mut block)?;
        
        Ok(block)
    }
    
    /// Add transactions to block
    fn add_transactions(&self, block: &mut Block) -> Result<(), Error> {
        let transactions = self.tx_pool.get_ready_transactions();
        
        for tx in transactions {
            if block.can_add_transaction(&tx) {
                block.add_transaction(tx);
            }
        }
        
        Ok(())
    }
}
```

## Validator Selection

### 1. Validator Scoring
```rust
pub struct ValidatorScoring {
    /// Performance metrics
    metrics: BTreeMap<AccountId, ValidatorMetrics>,
    /// Scoring parameters
    params: ScoringParams,
}

impl ValidatorScoring {
    /// Calculate validator score
    pub fn calculate_score(
        &self,
        validator: &AccountId,
    ) -> Score {
        let metrics = self.metrics.get(validator).unwrap_or_default();
        
        // Calculate components
        let uptime_score = self.calculate_uptime_score(metrics);
        let performance_score = self.calculate_performance_score(metrics);
        let stake_score = self.calculate_stake_score(metrics);
        
        // Combine scores
        Score::combine(vec![
            uptime_score,
            performance_score,
            stake_score,
        ])
    }
}
```

### 2. Validator Rotation
```rust
pub struct ValidatorRotation {
    /// Current validator set
    current_set: ValidatorSet,
    /// Queued validators
    queued_validators: Vec<AccountId>,
    /// Rotation config
    config: RotationConfig,
}

impl ValidatorRotation {
    /// Rotate validator set
    pub fn rotate_validators(
        &mut self,
        scores: BTreeMap<AccountId, Score>,
    ) -> Result<ValidatorSet, Error> {
        // Sort by score
        let mut sorted_validators: Vec<_> = scores.iter().collect();
        sorted_validators.sort_by(|a, b| b.1.cmp(a.1));
        
        // Select top validators
        let new_set: Vec<_> = sorted_validators
            .iter()
            .take(self.config.max_validators)
            .map(|(v, _)| v.clone())
            .collect();
            
        Ok(ValidatorSet::new(new_set))
    }
}
```

## Performance Optimization

### 1. Block Time Optimization
```rust
pub struct BlockTimeOptimizer {
    /// Block production metrics
    metrics: BlockMetrics,
    /// Target block time
    target_time: Duration,
    /// Adjustment parameters
    params: AdjustmentParams,
}

impl BlockTimeOptimizer {
    /// Adjust block production parameters
    pub fn adjust_parameters(
        &mut self,
    ) -> Result<AdjustmentResult, Error> {
        let current_time = self.metrics.average_block_time();
        
        if current_time > self.target_time {
            self.decrease_difficulty()
        } else if current_time < self.target_time {
            self.increase_difficulty()
        } else {
            Ok(AdjustmentResult::NoChange)
        }
    }
}
```

### 2. Network Optimization
```rust
pub struct NetworkOptimizer {
    /// Network metrics
    metrics: NetworkMetrics,
    /// Optimization config
    config: OptimizerConfig,
}

impl NetworkOptimizer {
    /// Optimize network parameters
    pub fn optimize(
        &mut self,
    ) -> Result<OptimizationResult, Error> {
        // Analyze metrics
        let analysis = self.analyze_metrics()?;
        
        // Apply optimizations
        if analysis.needs_optimization() {
            self.apply_optimizations(&analysis)?;
        }
        
        Ok(OptimizationResult::new(analysis))
    }
}
```
