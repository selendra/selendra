# Requirements Document

## Introduction

This specification outlines the development requirements for the Selendra Ecosystem Applications suite - a comprehensive collection of applications that will transform Selendra into the dominant blockchain for Southeast Asian sports entertainment and payments.

**Strategic Context:**
Selendra is building a complete digital economy for Southeast Asia with four interconnected pillars:
1. **Selendra Blockchain** - EVM-compatible L1 with 1s blocks, <2s finality
2. **Baray Payment System** - Banking integration (ABA, ACLEDA, WING) + KHQR + Bakong
3. **StadiumX & CPL Play** - Sports ticketing and fan engagement with 30K+ existing users
4. **Riverbase.app** - Headless e-commerce SaaS for SMEs with 100+ stores

**Key Differentiators:**
- Direct API access to top 3 Cambodian banks (unique in blockchain space)
- 30,000 users with custodial wallets already holding digital assets
- Partnership with Cambodian Premier League and 5 professional clubs
- Kun Khmer (national sport) launching November 2025 with 10 events/week
- $10K worth of CPL STAR tokens already on-chain
- 100+ SME stores on Riverbase (instant merchant distribution advantage)

**Three-Stablecoin Strategy:**
- **CPL STAR** ($0.01 USD) - Sports loyalty token for predictions and merchandise
- **sUSD** ($1.00 USD) - 1:1 wrapped USDT/USDC bridge token for DeFi
- **KHRt** (1 KHR ~$0.00025) - Local currency stablecoin backed by bank reserves via Baray

The ecosystem apps will leverage these unique assets and Selendra v4.0's advanced features to deliver best-in-class user experiences that compete with leading Web3 applications while solving real problems for Southeast Asian users.

## Requirements

### Requirement 1: Unified Wallet Application (Enhance Existing)

**User Story:** As a Selendra user, I want a comprehensive wallet application so that I can manage both native and EVM assets seamlessly with advanced account features.

**Current State:** 30,000 custodial wallets on StadiumX holding STAR + player cards with email/password login

#### Acceptance Criteria

1. WHEN users create accounts THEN they SHALL be able to generate both native Substrate and EVM addresses simultaneously
2. WHEN managing assets THEN users SHALL see unified balance views for SEL, USDT/USDC, sUSD, KHRt, STAR, and fan tokens
3. WHEN adding money THEN users SHALL generate KHQR codes, pay from bank, and receive KHRt automatically via Baray integration
4. WHEN cashing out THEN users SHALL burn KHRt and receive KHR in their bank account via Baray
5. WHEN swapping tokens THEN users SHALL access integrated DEX with gasless transactions (custodial advantage)
6. WHEN setting up security THEN users SHALL be able to configure social recovery with trusted guardians
7. WHEN interacting with dApps THEN users SHALL use session keys for seamless transactions without repeated signing
8. IF users lose access THEN they SHALL recover accounts through guardian-based social recovery system
9. WHEN staking SEL THEN users SHALL delegate to validators and claim rewards in SEL or KHRt
10. WHEN using mobile THEN users SHALL have biometric security, push notifications, and Khmer language support

### Requirement 2: DeFi Dashboard and Trading Platform

**User Story:** As a DeFi user, I want a comprehensive trading platform so that I can access all Selendra DeFi features through an intuitive interface.

#### Acceptance Criteria

1. WHEN trading tokens THEN users SHALL access the native DEX with priority pools: STAR/KHRt, KHRt/sUSD, SEL/sUSD, sUSD/USDT, SEL/STAR
2. WHEN providing liquidity THEN users SHALL add/remove liquidity to pools with APY calculations and impermanent loss warnings
3. WHEN staking SEL THEN users SHALL delegate to validators with real-time rewards tracking and auto-compounding options
4. WHEN monitoring portfolios THEN users SHALL see comprehensive analytics including P&L, yield farming rewards, and asset allocation
5. WHEN earning yield THEN users SHALL deposit KHRt or sUSD to earn 5-10% APY (significantly better than banks at 0.5-2%)
6. WHEN using sUSD THEN users SHALL deposit USDT/USDC 1:1 and redeem back 1:1 (simple wrapped model)
7. IF arbitrage opportunities exist THEN advanced users SHALL access MEV-protected trading with sandwich attack prevention
8. WHEN using leverage THEN users SHALL access lending/borrowing protocols with health factor monitoring and liquidation protection

### Requirement 3: Developer Portal and Documentation Hub

**User Story:** As a developer, I want a centralized portal so that I can access all development resources, tools, and community support in one place.

#### Acceptance Criteria

1. WHEN starting development THEN developers SHALL find comprehensive getting-started guides for both Substrate and EVM development
2. WHEN exploring APIs THEN developers SHALL access interactive API documentation with live examples and code generation
3. WHEN deploying contracts THEN developers SHALL use integrated deployment tools with testnet faucets and contract verification
4. WHEN using templates THEN developers SHALL access 10+ production-ready project templates (ERC20, NFT, DeFi, DAO)
5. WHEN coding THEN developers SHALL use browser-based IDE with Monaco Editor supporting Solidity and Rust
6. WHEN needing tokens THEN developers SHALL access instant testnet faucet without email verification
7. WHEN debugging issues THEN developers SHALL access community forums, Discord integration, and AI-powered troubleshooting
8. IF seeking inspiration THEN developers SHALL browse a showcase of successful projects with open-source code examples
9. WHEN building complex dApps THEN developers SHALL access advanced tutorials covering cross-chain integration and unified accounts

### Requirement 4: NFT Marketplace and Creator Tools

**User Story:** As an NFT creator and collector, I want a full-featured marketplace so that I can mint, trade, and showcase NFTs with advanced features.

#### Acceptance Criteria

1. WHEN minting NFTs THEN creators SHALL use no-code tools with batch minting, metadata management, and royalty configuration
2. WHEN trading NFTs THEN users SHALL access marketplace features including auctions, offers, bundles, and fractional ownership
3. WHEN showcasing collections THEN creators SHALL build custom gallery pages with social features and community engagement
4. WHEN verifying authenticity THEN users SHALL see provenance tracking and creator verification badges
5. IF creating utility NFTs THEN creators SHALL integrate with DeFi protocols for staking, governance, and yield generation
6. WHEN cross-chain trading THEN users SHALL bridge NFTs to/from Ethereum and other supported networks

### Requirement 5: Governance and DAO Management Platform

**User Story:** As a community member, I want governance tools so that I can participate in network governance and manage DAOs effectively.

#### Acceptance Criteria

1. WHEN participating in governance THEN users SHALL vote on council elections, referenda, and treasury proposals through intuitive interfaces
2. WHEN creating DAOs THEN users SHALL deploy governance contracts with customizable voting mechanisms and treasury management
3. WHEN proposing changes THEN users SHALL submit proposals with impact analysis, discussion threads, and voting timelines
4. WHEN delegating votes THEN users SHALL choose representatives with transparent voting history and delegation tracking
5. IF managing treasuries THEN DAOs SHALL have multi-signature controls with spending limits and approval workflows
6. WHEN analyzing governance THEN users SHALL access analytics on voting patterns, proposal outcomes, and participation rates

### Requirement 6: Cross-Chain Bridge Interface

**User Story:** As a multi-chain user, I want a user-friendly bridge interface so that I can move assets between Selendra and other blockchains safely and efficiently.

#### Acceptance Criteria

1. WHEN bridging assets THEN users SHALL bridge USDT/USDC from Ethereum, BSC, Polygon, and Arbitrum/Optimism to Selendra
2. WHEN monitoring transfers THEN users SHALL track transaction status across chains with real-time updates and notifications
3. WHEN selecting routes THEN users SHALL see optimal paths with cost comparisons and security ratings for different bridge options
4. WHEN handling large amounts THEN users SHALL be informed of timelock delays and additional security measures (24-hour for $1M+)
5. WHEN using security THEN transfers SHALL be secured by 2-of-3 multi-signature system with emergency pause capability
6. IF bridge issues occur THEN users SHALL access support tools with transaction recovery and dispute resolution
7. WHEN bridging new tokens THEN users SHALL request token additions through community governance processes

### Requirement 7: Analytics and Block Explorer

**User Story:** As a network participant, I want comprehensive analytics tools so that I can monitor network health, transaction activity, and ecosystem growth.

#### Acceptance Criteria

1. WHEN exploring transactions THEN users SHALL search and view detailed transaction information across both native and EVM runtimes
2. WHEN monitoring network THEN users SHALL see real-time metrics including TPS, block times, validator performance, and network security
3. WHEN analyzing DeFi THEN users SHALL access TVL tracking, yield farming analytics, and protocol performance metrics
4. WHEN researching addresses THEN users SHALL see comprehensive address profiles with transaction history and token holdings
5. IF investigating contracts THEN users SHALL access contract verification, source code viewing, and interaction history
6. WHEN tracking governance THEN users SHALL monitor proposal lifecycle, voting participation, and treasury spending

### Requirement 8: Mobile Applications

**User Story:** As a mobile user, I want native mobile apps so that I can access Selendra features on-the-go with optimal performance and security.

#### Acceptance Criteria

1. WHEN using mobile wallet THEN users SHALL have full wallet functionality with biometric security and hardware wallet support
2. WHEN trading on mobile THEN users SHALL access simplified DeFi interfaces optimized for mobile interaction patterns
3. WHEN receiving notifications THEN users SHALL get push notifications for governance votes, staking rewards, and price alerts
4. WHEN scanning QR codes THEN users SHALL easily send transactions and connect to dApps through QR code integration
5. IF using offline features THEN users SHALL access read-only functionality and transaction preparation without internet connection
6. WHEN syncing across devices THEN users SHALL have consistent experience with cloud backup and multi-device account access

### Requirement 9: Enterprise Integration Tools & Merchant Payments

**User Story:** As an enterprise developer, I want integration tools so that I can connect existing systems with Selendra blockchain functionality.

#### Acceptance Criteria

1. WHEN integrating APIs THEN enterprises SHALL access RESTful APIs with comprehensive documentation and SDKs for major programming languages
2. WHEN processing payments THEN merchants SHALL accept KHRt, sUSD, STAR, and SEL with 0.5-1% fees (vs 3-5% for credit cards)
3. WHEN using Riverbase THEN 100+ existing SME stores SHALL enable crypto payments with simple toggle (instant merchant distribution)
4. WHEN settling payments THEN merchants SHALL choose to hold crypto, auto-convert to KHRt, or cash out to bank via Baray
5. WHEN managing compliance THEN enterprises SHALL access KYC/AML tools with regulatory reporting and audit trail features
6. WHEN processing remittances THEN workers abroad SHALL send money home for 0.5-2% fees (vs 7-10% for Western Union)
7. WHEN managing payroll THEN companies SHALL pay employees in KHRt, sUSD, or SEL with same-day settlement
8. WHEN scaling operations THEN enterprises SHALL use dedicated infrastructure with SLA guarantees and priority support
9. IF requiring custom features THEN enterprises SHALL access white-label solutions and custom development services
10. WHEN monitoring usage THEN enterprises SHALL have detailed analytics dashboards with usage metrics and cost optimization insights

### Requirement 10: Sports Ecosystem Applications

**User Story:** As a sports fan, I want comprehensive sports applications so that I can engage with CPL football and Kun Khmer through predictions, NFT tickets, fan tokens, and social features.

**Current State:** StadiumX (30K users), CPL Play (10K users, 2K weekly active), $10K STAR tokens on-chain

#### Acceptance Criteria

1. WHEN making predictions THEN users SHALL predict CPL football and Kun Khmer outcomes with STAR, KHRt, or sUSD stakes
2. WHEN Kun Khmer launches THEN users SHALL predict fight outcomes, methods, rounds, duration, and techniques (10 events/week)
3. WHEN buying tickets THEN users SHALL receive ERC-721 NFT tickets with QR codes for venue entry and collectible value
4. WHEN trading tickets THEN users SHALL buy/sell on secondary marketplace with 2-3% platform fee and fraud prevention
5. WHEN supporting clubs THEN users SHALL buy fan tokens for governance voting, exclusive access, and revenue sharing
6. WHEN earning loyalty THEN users SHALL earn STAR for attendance, purchases, predictions, and referrals with tier-based benefits
7. WHEN redeeming rewards THEN users SHALL exchange STAR for CPL merchandise, experiences, and partner products
8. WHEN socializing THEN users SHALL challenge friends, join leaderboards, earn achievements, and share attended events
9. WHEN using mobile THEN users SHALL access all features through React Native apps with push notifications
10. IF creating content THEN users SHALL access fantasy sports, athlete tokens, and streaming/media platforms

### Requirement 11: Community and Social Features

**User Story:** As a community member, I want social features so that I can connect with other users, share experiences, and build the Selendra ecosystem together.

#### Acceptance Criteria

1. WHEN joining the community THEN users SHALL access integrated social features with profiles, following, and reputation systems
2. WHEN sharing achievements THEN users SHALL showcase NFTs, DeFi positions, and governance participation through social profiles
3. WHEN learning together THEN users SHALL access community-driven tutorials, AMAs, and educational content
4. WHEN getting support THEN users SHALL use integrated help systems with community moderators and expert assistance
5. IF contributing to ecosystem THEN users SHALL participate in bug bounties, hackathons, and developer incentive programs
6. WHEN building reputation THEN users SHALL earn badges and recognition for positive community contributions and platform usage