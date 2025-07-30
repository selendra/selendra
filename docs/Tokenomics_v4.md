# Selendra Network v4: SEL Tokenomics with Native Account Abstraction

**Version 4.0**  
**Date: January 2025**

## Executive Summary

This document defines the tokenomics for SEL, the native utility token of Selendra Network v4, with support for Native Account Abstraction. The tokenomics model addresses smart contract accounts, fee delegation, gasless transactions, and multi-token fee payments while maintaining network security and sustainability.

## 1. Introduction

Selendra Network v4 introduces Native Account Abstraction at the protocol level, changing how users interact with the blockchain. This requires a tokenomics model that supports:

- **Smart Contract Account Operations**: Every account is a programmable smart contract
- **Fee Delegation & Sponsorship**: Applications can pay fees for their users
- **Multi-Token Fee Payments**: Users can pay fees in various tokens
- **Gasless User Experience**: Frictionless onboarding and usage
- **Session Key Economics**: Temporary permission systems

## 2. SEL Token Core Utilities

### 2.1 Primary Utility Functions

**1. Network Security (Staking)**
- Validators stake SEL to participate in Aura + AlephBFT consensus
- Minimum validator bond: 31,416 SEL (subject to governance adjustment)
- Delegators can stake with validators to earn rewards
- Slashing penalties for malicious behavior or poor performance

**2. Base Layer Transaction Fees**
- SEL remains the fundamental unit for computational cost measurement
- All fee calculations are denominated in SEL at the protocol level
- Fee burning mechanism creates deflationary pressure

**3. Governance & Protocol Updates**
- SEL holders vote on protocol upgrades and parameter changes
- Staked SEL provides voting weight in governance decisions
- Treasury allocation and ecosystem fund management

**4. Network Service Access**
- Account abstraction feature activation
- Premium naming services
- Cross-chain bridge operations
- Advanced developer tools and APIs

### 2.2 Additional Utilities for Account Abstraction

**5. Paymaster Operations**
- SEL required to operate as a paymaster (fee sponsor)
- Minimum SEL bond for paymaster registration
- Economic structure for fee delegation services

**6. Session Key Management**
- SEL fees for session key creation and management
- Spam prevention for temporary permissions
- Automated key rotation and security services

**7. Social Recovery Services**
- SEL fees for guardian setup and recovery operations
- Incentive structure for recovery service providers
- Dispute resolution and arbitration services

## 3. Native Account Abstraction Fee Architecture

### 3.1 Multi-Layer Fee System

```
User Experience Layer:
┌─────────────────────────────────────────────┐
│  User pays in: SEL, USD Stablecoin, or     │
│  gets sponsored (gasless experience)        │
└─────────────────────────────────────────────┘
                    ↓
Protocol Layer:
┌─────────────────────────────────────────────┐
│  All fees ultimately settled in SEL        │
│  Automatic conversion via DEX integration   │
└─────────────────────────────────────────────┘
```

### 3.2 Fee Payment Mechanisms

**1. Direct SEL Payment (Traditional)**
```rust
// User account has SEL balance
account.pay_fees(transaction_cost_in_sel);
```

**2. Multi-Token Fee Payment**
```rust
// User pays in stablecoin, protocol converts to SEL
account.pay_fees_with_token(
    token: USD_STABLECOIN,
    amount: usd_equivalent,
    max_slippage: 2% // Protection against conversion variation
);
```

**3. Sponsored Transactions (Gasless)**
```rust
// Paymaster sponsors the transaction
paymaster.sponsor_transaction(
    user_account: user,
    operation: user_operation,
    fee_policy: "app_covers_all_fees"
);
```

**4. Session Key Operations**
```rust
// Temporary key with spending limit
session_key.execute_with_limit(
    spending_limit: 100_SEL_equivalent,
    duration: 7_days,
    permissions: ["transfer", "interact_with_dex"]
);
```

### 3.3 Paymaster Economic Model

**Paymaster Types:**

**1. Application Paymasters**
- DApps sponsor fees for their users
- Business model: User acquisition and retention
- Revenue: Application-specific (subscriptions, transaction fees, etc.)

**2. Subscription Paymasters**
- Users pay monthly/yearly fee for gasless transactions
- Revenue model: Predictable subscription income
- SEL reserves: Maintain buffer for fee payment volatility

**3. Commercial Paymasters**
- Third-party services offering fee sponsorship
- Revenue model: Fee markup or service commissions
- Market position: Compete on conversion rates and user experience

### 3.4 Paymaster Economic Requirements

```rust
pub struct PaymasterConfig {
    // Minimum SEL bond to operate as paymaster
    min_bond: Balance, // e.g., 10,000 SEL
    
    // Fee conversion parameters
    max_conversion_slippage: Percent, // e.g., 5%
    fee_markup_limit: Percent, // e.g., 10%
    
    // Risk management
    daily_sponsorship_limit: Balance,
    user_spending_limits: UserLimits,
    
    // Economic incentives
    paymaster_reward_share: Percent, // Share of saved UX friction value
}
```

## 4. Economic Sustainability Model

### 4.1 Fee Flow Architecture

```
Transaction Initiated (Base Fee: 0.001 SEL):
┌─────────────────┐
│ User Operation  │
└─────────┬───────┘
          ↓
Fee Payment Decision:
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   User Pays     │    │  Paymaster      │    │   Session Key   │
│  (0.001 SEL)    │    │   Sponsors      │    │ (0.0005 SEL)    │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          ↓                      ↓                      ↓
Protocol Fee Settlement (0.001 SEL base fee):
┌─────────────────────────────────────────────────────────────────┐
│              SEL Fee Distribution (Per 0.001 SEL)              │
│  0.0004 SEL → Burned (40% - Deflationary pressure)           │
│  0.00035 SEL → Validators (35% - Block rewards)              │
│  0.00015 SEL → Treasury (15% - Development fund)            │
│  0.0001 SEL → Paymaster Incentives (10% - UX rewards)        │
└─────────────────────────────────────────────────────────────────┘

Daily Volume Example (10,000 TPS average):
- Total daily fees: 864,000 SEL
- Burned: 345,600 SEL (deflationary pressure)
- Validators: 302,400 SEL (rewards increase)
- Treasury: 129,600 SEL (ecosystem development)
- Paymaster incentives: 86,400 SEL (user experience fund)
```

### 4.2 Token Conversion Mechanism

**Integrated DEX for Fee Conversion:**
- Protocol-level DEX integration for token conversion
- Automated Market Maker (AMM) pools for major tokens
- Slippage protection and MEV resistance
- Conversion rates displayed to users

**Supported Fee Tokens (Initial):**
- SEL (native)
- USD Stablecoin (USDC equivalent)
- KHR Stablecoin (local market)
- ETH (bridged)
- BTC (bridged)

**Governance-Controlled Token List:**
- SEL holders vote to add/remove supported fee tokens
- Quality standards: liquidity requirements, security audits
- Fee conversion parameters set by governance

### 4.3 Economic Incentive Alignment

**User Benefits:**
- Freedom to pay fees in preferred token
- Gasless experience through paymasters
- Predictable costs through session keys
- Social recovery without complex fee management

**Paymaster Benefits:**
- Revenue opportunities through fee sponsorship
- User acquisition tool for applications
- Competitive differentiation through superior UX

**Validator Benefits:**
- At 10,000 TPS: 302,400 SEL daily from fees (110M SEL annually)
- Combined with 21M SEL inflation = 131M SEL total annual rewards
- 6.2x increase in validator income vs inflation-only model
- SEL demand from all fee conversions

**Network Benefits:**
- Competitive fees support adoption
- Deflationary pressure: 126M SEL burned annually at 10K TPS
- Economic sustainability: Low user costs with validator rewards
- Treasury funding: 47M SEL annually for development

## 5. Gasless Transaction Economics

### 5.1 Gasless Transaction Models

**1. Application-Sponsored Model**
```
Example: Gaming DApp
- Game sponsors all in-game transactions
- Users play without crypto knowledge required
- Revenue: In-app purchases, premium features
- Cost: Network fees as customer acquisition expense
```

**2. Freemium Model**
```
Example: DeFi Platform
- Basic operations: User pays fees
- Premium users: Gasless transactions included
- Revenue: Subscription fees, premium features
- Cost: Predictable monthly gasless transaction budget
```

**3. Cross-Subsidization Model**
```
Example: DEX Platform
- High-value users: Pay normal fees
- Small transactions: Sponsored to encourage volume
- Revenue: Trading fees and increased volume
- Cost: Calculated customer lifetime value investment
```

### 5.2 Sustainable Gasless Economics

**Economic Viability Requirements:**

1. **Customer Lifetime Value > Sponsorship Cost**
   - At 0.001 SEL per transaction, sponsorship costs are low
   - 1,000 sponsored transactions cost 1 SEL (~$0.025)
   - Favorable LTV to CAC ratios achievable

2. **Predictable Fee Budgeting**
   - Applications can budget: 1 SEL = 1,000 transactions
   - Monthly budget example: 100 SEL = 100,000 sponsored transactions
   - Real-time monitoring shows transaction counts

3. **Value-Based Sponsorship**
   - Base fees support sponsoring user transactions
   - Micro-transaction compatible (gaming, social media)
   - Simplified subsidy models

## 6. Session Key Economic Model

### 6.1 Session Key Fee Structure

**Creation Costs:**
```rust
pub struct SessionKeyFees {
    // One-time creation fee
    creation_fee: Balance, // e.g., 0.1 SEL
    
    // Daily maintenance (for active keys)
    daily_fee: Balance, // e.g., 0.01 SEL/day
    
    // Per-transaction fee discount
    transaction_discount: Percent, // e.g., 50% off normal fees
    
    // Maximum spending limit enforcement
    limit_enforcement_fee: Balance, // e.g., 0.01 SEL per check
}
```

**Economic Benefits:**
- Users save on transaction signing UX friction
- Applications gain seamless user experience
- Validators earn fees from increased transaction volume
- Network benefits from higher engagement

### 6.2 Session Key Business Models

**1. Gaming Applications**
- Session keys for in-game actions (0.0005 SEL per transaction with 50% discount)
- Players avoid approving every move
- Game sponsors session key creation (cost: 0.17 SEL setup)
- Break-even: 340+ transactions per session
- Revenue: In-app purchases and subscriptions

**2. DeFi Automation**
- Session keys for automated trading strategies (0.0005 SEL per trade)
- Users set spending limits and permissions
- Profitable for high-frequency traders (500+ trades/month)
- Revenue: Performance fees and management fees

**3. Social Applications**
- Direct fee sponsorship more cost-effective than session keys
- Social platforms can sponsor interactions at scale
- Example: 1 million likes = 1,000 SEL cost (~$25)
- Revenue: Advertising and premium features

## 7. Social Recovery Economic Framework

### 7.1 Guardian Economics

**Guardian Incentive Structure:**
```rust
pub struct GuardianIncentives {
    // Payment for successful recovery assistance
    recovery_reward: Balance, // e.g., 50 SEL
    
    // Bond for guardian commitment
    guardian_bond: Balance, // e.g., 10 SEL
    
    // Reputation system rewards
    reputation_multiplier: f32, // Increases rewards over time
    
    // Penalty for non-participation
    non_response_penalty: Balance, // e.g., 5 SEL
}
```

**Recovery Service Providers:**
- Professional guardian services
- Revenue model: Service fees and reputation rewards
- Insurance options for recovery failures
- SLA commitments for response times

### 7.2 Recovery Cost Structure

**User Costs:**
- Guardian setup: 1 SEL per guardian
- Recovery initiation: 20 SEL (refunded if successful)
- Emergency recovery: 100 SEL (premium for urgent cases)

**Service Provider Revenue:**
- Recovery assistance fees: 30-50 SEL per successful recovery
- Premium services: 24/7 support, guaranteed response times
- Insurance products: Protection against failed recoveries

## 8. Implementation Roadmap

### 8.1 Phase 1: Basic Account Abstraction (Months 1-6)

**Fee Mechanisms:**
- [ ] Direct SEL fee payment for smart contract accounts (0.001 SEL base fee)
- [ ] Basic paymaster infrastructure
- [ ] Simple fee delegation for whitelisted applications

**Economic Parameters:**
- Base transaction fee: 0.001 SEL (~$0.000025 USD when SEL reaches $0.025)
- Paymaster minimum bond: 1,000 SEL
- Fee distribution: 40% burn, 35% validators, 15% treasury, 10% paymaster incentives
- Session key creation fee: 0.1 SEL

### 8.2 Phase 2: Multi-Token & Advanced Features (Months 7-12)

**Enhanced Fee System:**
- [ ] Multi-token fee payment integration
- [ ] Advanced paymaster economics
- [ ] Session key management and automation

**Economic Enhancements:**
- Support for 5+ fee payment tokens
- Paymaster reward system implementation
- Dynamic fee adjustment based on network usage

### 8.3 Phase 3: Advanced Economics & Optimization (Months 13-18)

**Complete Economic Framework:**
- [ ] Social recovery service marketplace
- [ ] Advanced gasless transaction models
- [ ] Comprehensive session key economics

**Market Maturation:**
- Competitive paymaster ecosystem
- Professional guardian services
- Automated fee optimization algorithms

## 9. Risk Management & Security

### 9.1 Economic Attack Vectors

**1. Paymaster Exploitation**
- Risk: Malicious users drain paymaster funds
- Mitigation: Spending limits, user verification, rate limiting

**2. Fee Token Manipulation**
- Risk: Price manipulation of fee payment tokens
- Mitigation: Decentralized price oracles, slippage protection

**3. Session Key Abuse**
- Risk: Compromised session keys used maliciously
- Mitigation: Spending limits, permission scoping, automatic expiration

### 9.2 Economic Sustainability Safeguards

**1. Fee Minimum Enforcement**
- All transactions must pay minimum network fee
- No completely free transactions (prevents spam)
- Economic spam prevention mechanisms

**2. Paymaster Solvency Requirements**
- Minimum SEL bonds for operation
- Regular solvency checks and reporting
- Automatic suspension for underfunded paymasters

**3. Token Conversion Safeguards**
- Maximum slippage protection
- Circuit breakers for extreme volatility
- Emergency pause mechanisms for market disruptions

## 10. Governance & Parameter Management

### 10.1 Adjustable Parameters

**Network-Level Parameters:**
- Fee burn percentage (currently 40%)
- Minimum paymaster bond (currently 1,000 SEL)
- Supported fee payment tokens
- Session key fee structure

**Paymaster Parameters:**
- Maximum daily sponsorship limits
- Conversion slippage tolerances
- User spending limit defaults
- Reward distribution ratios

### 10.2 Governance Process

**Parameter Updates:**
- SEL holder voting on economic parameters
- Technical committee review for implementation
- Gradual rollout with monitoring periods
- Emergency adjustment procedures

**Market Response:**
- Real-time monitoring of economic metrics
- Automated alerts for unusual patterns
- Community feedback integration
- Research-driven optimization

## 11. Success Metrics & KPIs

### 11.1 Adoption Metrics

**User Experience:**
- Percentage of gasless transactions
- Session key adoption rate
- Multi-token fee payment usage
- Social recovery setup rate

**Economic Health:**
- Paymaster ecosystem growth
- Fee conversion volume and efficiency
- SEL token velocity and demand
- Network revenue sustainability

### 11.2 Network Economics

**Token Metrics:**
- SEL burn rate and deflationary pressure
- Staking participation and rewards
- Treasury fund growth and allocation
- Cross-token fee payment volumes

**Market Development:**
- Number of active paymasters
- Guardian service provider growth
- Session key service ecosystem
- Average transaction costs (user-facing)

## 12. Conclusion

Selendra Network v4's tokenomics creates an economic model that supports Native Account Abstraction while maintaining network security and decentralization. The multi-layered fee system, paymaster economics, and session key framework enable improved user experience without compromising blockchain fundamentals.

Key features include:
- **User Experience**: Users can pay fees in preferred tokens or use gasless experiences
- **Economic Sustainability**: Multiple revenue streams and incentive mechanisms
- **Developer Opportunities**: Business models through paymaster and guardian services
- **Network Growth**: Adoption through reduced friction and improved accessibility

This tokenomics model positions Selendra v4 for user-centric blockchain design while maintaining the economic security and sustainability required for long-term operation.

---

## Appendix A: Fee Calculation Examples

### A.1 Multi-Token Fee Payment
```
User wants to make a transaction (0.001 SEL fee):
1. User selects USD stablecoin payment
2. Protocol queries DEX: 1 SEL = $0.025 USD (target price)
3. Required USD: 0.001 SEL × $0.025 = $0.000025 USD
4. Slippage protection: 5% maximum
5. Actual conversion: $0.000026 USD (4% slippage)
6. Fee distribution: 40% burn, 35% validators, etc.
```

### A.2 Paymaster Sponsorship
```
Gaming DApp sponsors user transaction:
1. User performs in-game action
2. Transaction cost: 0.001 SEL
3. Game's paymaster covers fee automatically
4. User sees no fee payment required
5. Game's SEL balance reduces by 0.001 SEL
6. Monthly budget tracking updated (1,000 transactions = 1 SEL)
```

### A.3 Session Key Operation
```
User creates 7-day trading session:
1. Session key creation: 0.1 SEL
2. Daily maintenance: 0.01 SEL × 7 days = 0.07 SEL
3. Transaction fee discount: 50% off normal fees (0.0005 SEL per transaction)
4. Spending limit: 100 SEL equivalent
5. Total setup cost: 0.17 SEL
6. Ongoing savings: 0.0005 SEL per transaction vs 0.001 SEL normal fee
```

## Appendix B: Economic Model Formulas

### B.1 Paymaster Viability Calculation
```
Paymaster Break-Even Analysis:
Revenue per User per Month (R) ≥ Average Fee Cost per User per Month (C)

Where:
R = Monthly subscription fee OR transaction-based revenue
C = (Average transactions per user per month) × (Average fee per transaction)

Example:
Gaming DApp:
R = $5/month in-app purchases per user
C = 30 transactions/month × 0.001 SEL × $0.025/SEL = $0.00075/month
Result: Extremely viable - revenue far exceeds fee costs
```

### B.2 Session Key ROI Formula
```
Session Key Value:
Savings = (Transaction Frequency × Fee Discount × Avg Fee) - Setup Cost

Example:
Active trader:
Setup Cost = 0.17 SEL
Monthly Transactions = 100
Fee per Transaction = 0.001 SEL
Discount = 50%
Monthly Savings = (100 × 0.50 × 0.001) - 0.17 = -0.12 SEL
Note: Setup cost higher than savings for low-frequency users.
Break-even: ~340 transactions per month
```