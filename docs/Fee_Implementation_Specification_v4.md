# Selendra Network v4: Fee Implementation Plan

**Version 1.0**  
**Date: January 2025**

## What We're Implementing

We're updating Selendra's fee structure to support broader adoption while maintaining network sustainability. The new system will have a base fee of 0.001 SEL per transaction, targeting ~$0.000025 USD when SEL reaches the target price.

## Current vs New Fee Structure

### Current State
- **Base fee**: 0.000005376 SEL per transaction
- **USD equivalent**: ~$0.000000134 USD
- **System**: EIP-1559-like dynamic adjustment

### What We're Building
- **New base fee**: 0.001 SEL per transaction (186x increase)
- **Target USD**: $0.000025 USD
- **Reason**: Competitive while ensuring validator rewards

## Implementation Plan

### Phase 1: Fixed SEL Fee (Launch with v4)
**When**: Immediately with v4 launch
**Why**: SEL not yet listed, no price oracles available

#### Tasks to Complete

**Runtime Updates:**
- [ ] Update `runtime/src/lib.rs` with new fee parameters
- [ ] Set `TransactionBaseFee` to 1_000_000_000_000_000 (0.001 SEL)
- [ ] Configure fee distribution: 40% burn, 35% validators, 15% treasury, 10% paymaster incentives
- [ ] Add minimum fee enforcement

**Fee Distribution System:**
- [ ] Create custom fee handler for multi-way distribution
- [ ] Implement automatic fee burning mechanism
- [ ] Set up validator reward pool distribution
- [ ] Configure treasury and paymaster incentive pools

**Dynamic Fee Pallet Updates:**
- [ ] Update `pallets/dynamic-evm-base-fee/src/lib.rs`
- [ ] Add account abstraction compatibility
- [ ] Configure base fee parameters
- [ ] Add transaction type multipliers

### Phase 2: USD-Pegged Fees (After Listing)
**When**: 6-12 months after launch
**Trigger**: SEL gets DEX/CEX listings with reliable oracles

#### Tasks to Complete

**Oracle Integration:**
- [ ] Add price oracle interface
- [ ] Set target USD fee ($0.000025)
- [ ] Configure price update intervals
- [ ] Add price deviation limits

**Dynamic Fee Calculation:**
- [ ] Implement USD-to-SEL conversion logic
- [ ] Add oracle failure fallback system
- [ ] Create price smoothing algorithms
- [ ] Build emergency mode triggers

## Account Abstraction Integration

### Smart Account Fee Features
- [ ] Support for paymaster-sponsored transactions
- [ ] Session key fee discounts (50% off)
- [ ] Multi-token fee payment system
- [ ] Gas estimation for user operations

### Paymaster System
- [ ] Paymaster registration and bonding
- [ ] Balance management for sponsors
- [ ] Fee delegation mechanisms
- [ ] Incentive distribution system

### Multi-Token Payments
- [ ] Token-to-SEL conversion interface
- [ ] Slippage protection (5% max)
- [ ] Supported token management
- [ ] Real-time conversion rates

## Economic Impact

### Revenue Analysis (10,000 TPS average)
- **Daily fees**: 864,000 SEL
- **Annual fees**: 315,360,000 SEL

### Fee Distribution (Annual)
- **Burned (40%)**: 126,144,000 SEL - Creates deflationary pressure
- **Validators (35%)**: 110,376,000 SEL - Reward increase
- **Treasury (15%)**: 47,304,000 SEL - Development funding
- **Paymaster (10%)**: 31,536,000 SEL - User experience fund

### Validator Reward Analysis
- **Current (inflation only)**: 21,000,000 SEL annually
- **New (inflation + fees)**: 131,376,000 SEL annually
- **Increase**: 6.25x reward increase for validators

## Risk Management

### Oracle Failure Protection
- [ ] Emergency fixed-fee mode
- [ ] Price deviation monitoring
- [ ] Automatic fallback systems
- [ ] Manual override capabilities

### Fee Gaming Prevention
- [ ] Price smoothing algorithms
- [ ] Update frequency limits
- [ ] Maximum change per update
- [ ] Multi-source price validation

### Network Security
- [ ] Minimum fee enforcement
- [ ] Spam prevention mechanisms
- [ ] Emergency halt capabilities
- [ ] Validator slashing for manipulation

## Governance Controls

### Adjustable Parameters
We can update these through governance:
- [ ] Base fee amount
- [ ] Target USD fee
- [ ] Fee distribution percentages
- [ ] Oracle update intervals
- [ ] Price deviation limits

### Update Process
1. Propose parameter changes
2. Technical review (1 week)
3. Community discussion (2 weeks)
4. Token holder voting (1 week)
5. Implementation (1 week notice)

## Implementation Checklist

### Month 1-2: Core System
- [ ] Update runtime fee configuration
- [ ] Implement fee distribution logic
- [ ] Enhance dynamic fee pallet
- [ ] Add account abstraction support
- [ ] Complete comprehensive testing

### Month 3-6: Advanced Features
- [ ] Build oracle integration (inactive)
- [ ] Add multi-token payment system
- [ ] Create paymaster infrastructure
- [ ] Implement governance controls
- [ ] Add monitoring systems

### Post-Listing: USD Pegging
- [ ] Activate oracle price feeds
- [ ] Enable dynamic USD-pegged fees
- [ ] Monitor system performance
- [ ] Optimize based on usage
- [ ] Scale infrastructure

## Testing Requirements

### Technical Tests
- [ ] Fee calculation accuracy (99.9%+)
- [ ] Distribution mechanism reliability
- [ ] Oracle integration stability
- [ ] Emergency mode functionality
- [ ] Performance under load

### Economic Tests
- [ ] Validator reward distribution
- [ ] Burn mechanism verification
- [ ] Treasury funding accuracy
- [ ] Paymaster incentive system
- [ ] Multi-token conversion rates

### User Experience Tests
- [ ] Account abstraction fee handling
- [ ] Gasless transaction flow
- [ ] Session key discounts
- [ ] Multi-token payment UX
- [ ] Error handling and recovery

## Success Metrics

### Performance Targets
- [ ] 100% uptime for fee system
- [ ] <100ms fee calculation time
- [ ] Zero fee-related failures
- [ ] Support target 10,000+ TPS

### Economic Goals
- [ ] 6x+ validator reward increase
- [ ] 100M+ SEL burned annually
- [ ] Functional paymaster ecosystem
- [ ] Competitive with other networks

### Adoption Metrics
- [ ] 50%+ transactions use advanced features
- [ ] 10+ active paymaster services
- [ ] High user satisfaction scores
- [ ] Maintained cost competitiveness

## Code Implementation Areas

### Files to Update
- [ ] `runtime/src/lib.rs` - Fee configuration
- [ ] `pallets/dynamic-evm-base-fee/src/lib.rs` - Dynamic fees
- [ ] `pallets/transaction-payment/` - Payment handling
- [ ] Create `pallets/fee-distribution/` - Distribution logic
- [ ] Update client APIs for new fee structure

### New Components
- [ ] Oracle price feed integration
- [ ] Multi-token conversion system
- [ ] Paymaster registration pallet
- [ ] Emergency fee mode handler
- [ ] Governance parameter management

## Timeline

### Immediate (With v4 Launch)
**Weeks 1-8**: Fixed SEL fee system
- Core fee structure implementation
- Account abstraction integration
- Basic paymaster support
- Comprehensive testing

### Near-term (3-6 months)
**Months 3-6**: Advanced features
- Multi-token payment system
- Oracle integration (inactive)
- Enhanced paymaster features
- Governance controls

### Long-term (6+ months)
**Post-listing**: USD-pegged system
- Activate oracle price feeds
- Enable dynamic USD fees
- Monitor and optimize
- Scale based on usage

## Final Notes

This fee system is designed to:
1. **Support broader adoption** - Low fees reduce barriers
2. **Reward validators** - 6x increase in validator income
3. **Create deflationary pressure** - Token burning mechanism
4. **Enable advanced features** - Gasless transactions, session keys
5. **Maintain competitiveness** - Competitive fees in the market

We're building this system to make blockchain more accessible while ensuring the network remains sustainable and secure for validators and users.

---

**Implementation Priority**: High - Required for v4 launch success