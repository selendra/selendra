# Selendra Network v4: Fee Implementation Plan

**Version 1.0**  
**Date: July 2025**

## Overview

We're updating Selendra's fee structure to support broader adoption while maintaining network sustainability. The new system will have a base fee of 0.001 SEL per transaction, targeting ~$0.000025 USD when SEL reaches the target price.

## Fee Structure Changes

### Current vs New
- **Current v3**: ~0.0005 SEL (~$0.000012 USD)
- **Target v4**: 0.001 SEL (~$0.000025 USD) - 2x increase
- **Reason**: Competitive while ensuring validator rewards

### Fee Distribution
- 40% burned (deflationary pressure)
- 35% to validators (security incentive) 
- 15% to network treasury (development funding)
- 10% to paymaster pool (sponsorship incentives)

## Implementation Phases

### Phase 1: Fixed SEL Fee (v4 Launch)
**Core Tasks:**
- [ ] Update runtime fee parameters to 0.001 SEL
- [ ] Implement fee distribution system
- [ ] Add account abstraction support
- [ ] Configure paymaster infrastructure
- [ ] Enable session key discounts (50% off)

### Phase 2: USD-Pegged Fees (Post-Listing)
**Advanced Features:**
- [ ] Oracle price feed integration
- [ ] Multi-token fee payment system
- [ ] Dynamic USD-to-SEL conversion
- [ ] Emergency fallback mechanisms

## Account Abstraction Integration

### Key Features
- Paymaster-sponsored transactions
- Multi-token fee payments with slippage protection (5% max)
- Session key fee discounts (50% off base fee)
- Gasless transaction support
- Guardian-based account recovery
- Cross-business loyalty token exchanges

## Economic Impact

### Performance Targets
- **Current v3**: 400-800 TPS with 1-second blocks, 2-3 second finality
- **Target v4**: 1,200-4,000 TPS through parallel processing
- **Peak capacity**: Up to 10,000+ TPS with advanced scaling

### At Target Performance (4,000 TPS Average)
- **Daily fees**: 345,600 SEL
- **Annual validator rewards**: 44,150,400 SEL (6.25x increase over inflation-only)
- **Annual burn**: 50,457,600 SEL (deflationary pressure)
- **Treasury funding**: 18,921,600 SEL annually

## Risk Management

### Safeguards
- Oracle failure protection with emergency fixed-fee mode
- Price deviation monitoring and smoothing algorithms
- Minimum fee enforcement for spam prevention
- Governance controls for parameter adjustments

## Implementation Timeline

### Phase 1: Foundation (Months 1-3)
- Build transaction conflict detection algorithms
- Adapt Substrate's Executive for parallel execution
- Update fee structure to 0.001 SEL base
- Account abstraction integration

### Phase 2: Core Engine (Months 4-6)
- Implement configurable thread pool architecture
- Enable parallel access to different account ranges
- Multi-token payment system
- Paymaster infrastructure

### Phase 3: Optimization (Months 7-9)
- Real-time transaction grouping based on access patterns
- Optimize state access for parallel execution
- Target 1,200+ TPS sustained performance

### Phase 4: Production (Months 10-12)
- Gradual rollout with fallback to sequential processing
- Target 4,000+ TPS peak capacity
- USD-pegged fees (post-listing)
- Full v4 feature deployment

## Files to Update

**Core Components:**
- `runtime/src/lib.rs` - Fee configuration
- `pallets/dynamic-evm-base-fee/src/lib.rs` - Dynamic fees
- `pallets/transaction-payment/` - Payment handling
- Create `pallets/fee-distribution/` - Distribution logic
- Account abstraction pallet - Smart contract accounts
- Paymaster registration system

**Technical Constraints:**
- Block weight limit: 400ms computation per 1-second block
- EVM gas limit: 36 million gas per block
- Maximum block size: 5MB
- Validator set: Up to 100,000 validators supported

## Success Metrics

### Performance
- [ ] 100% uptime for fee system
- [ ] Support 1,200-4,000 TPS (v4 target)
- [ ] Support up to 10,000+ TPS (advanced scaling)
- [ ] Maintain 1-second blocks and 2-3 second finality
- [ ] <100ms fee calculation time

### Economics
- [ ] 6x+ validator reward increase over inflation-only model
- [ ] 50M+ SEL burned annually at target performance
- [ ] Functional paymaster ecosystem with 10+ active services

### Adoption
- [ ] 50%+ transactions use advanced features (gasless, session keys)
- [ ] 50+ applications actively using v4 features
- [ ] 100,000+ daily active users across applications
- [ ] 90%+ successful migration of existing applications

## Governance

**Adjustable Parameters:**
- Base fee amount and target USD fee
- Fee distribution percentages
- Oracle update intervals and deviation limits

**Update Process:**
1. Technical review (1 week)
2. Community discussion (2 weeks) 
3. Token holder voting (1 week)
4. Implementation (1 week notice)

---

This fee system supports broader adoption through ultra-low fees (~$0.000025), rewards validators with 6x income increase, creates deflationary pressure through burning, and enables advanced features like gasless transactions and session keys. The system targets 1,200-4,000 TPS through parallel processing while maintaining 1-second blocks and 2-3 second mathematical finality.

**Implementation Priority**: High - Required for v4 launch success