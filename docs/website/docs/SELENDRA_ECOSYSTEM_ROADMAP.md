# SELENDRA ECOSYSTEM: COMPLETE APP ROADMAP

**Document Version:** 1.0
**Last Updated:** January 2025
**Timeline:** 2025-2026
**Team Size:** 2-3 developers + Claude Code

---

## EXECUTIVE SUMMARY

Selendra is building a complete digital economy for Southeast Asia with four interconnected pillars:

1. **Selendra Blockchain** - EVM-compatible L1 with 1s blocks, <2s finality
2. **Baray Payment System** - Banking integration (ABA, ACLEDA, WING) + KHQR + Bakong
3. **StadiumX & CPL Play** - Sports ticketing and fan engagement with 30K+ existing users
4. **Riverbase.app** - Headless e-commerce SaaS for SMEs with Baray payments + Telegram mini shops

**Key Differentiators:**
- Direct API access to top 3 Cambodian banks (unique in blockchain space)
- 30,000 users with custodial wallets already holding digital assets
- Partnership with Cambodian Premier League and 5 professional clubs
- Kun Khmer (national sport) launching November 2025 with 10 events/week
- $10K worth of CPL STAR tokens already on-chain
- 100+ SME stores already using Riverbase e-commerce platform (instant merchant distribution)

**Strategic Vision:**
Build the dominant blockchain for Southeast Asian sports entertainment and payments, starting with Cambodia and expanding to Thailand, Vietnam, and Laos.

**Success Metrics by End of 2025:**
- 60,000+ StadiumX users
- 40,000+ Selendra wallet users
- $1M+ Total Value Locked
- 100+ merchants accepting crypto payments (Riverbase advantage)
- 200K+ monthly transactions

---

## I. CURRENT ASSETS & INFRASTRUCTURE

### Blockchain Infrastructure
- ✅ **Selendra Blockchain**: EVM-compatible, Substrate-based, 1s blocks, <2s finality
- ✅ **selendra.org**: Marketing website (React + TypeScript + Vite + Tailwind)
- ✅ **explorer.selendra.org**: Block explorer (existing, needs enhancement)

### Payment Infrastructure  
- ✅ **Baray Payment System**: LIVE
  - API aggregator for ABA, ACLEDA, WING banks
  - KHQR integration (7M+ Cambodians use KHQR)
  - Bakong blockchain integration (National Bank of Cambodia)
  - Currently handles ticket and player card purchases

### Live Consumer Products
- ✅ **StadiumX**: Sports ticketing platform
  - 30,000 users
  - 5 out of 11 Cambodian Premier League clubs
  - Cambodia national team partnership
  - CPL technology partner
  - Kun Khmer launching November 2025 (10 events/week, targeting 20K new users)
  
- ✅ **CPL Play**: Prediction game application
  - 10,000 users
  - 2,000 weekly active users (20% engagement rate)
  - $10K worth of CPL STAR tokens on Selendra
  - Weekly football outcome predictions
  - STAR token redemption for CPL merchandise

### Existing Wallet Infrastructure
- ✅ **Custodial Wallets**: 30,000 users with wallets holding STAR + player cards
  - Email/password login (Web2 UX)
  - No seed phrases (user-friendly)
  - Integrated into StadiumX platform

### Token Economics (Current)
- ✅ **CPL STAR**: Stable value token at $0.01 USD (1 cent)
  - Backed by CPL organization
  - 1,000,000 STAR currently on-chain ($10K value)
  - Earn-only (predictions, rewards)
  - Redeemable for CPL merchandise
  - Fixed value, not tradeable yet

### E-commerce Infrastructure
- ✅ **Riverbase.app**: Headless e-commerce SaaS platform LIVE
  - 100+ SME stores using the platform
  - Telegram mini shop integration
  - One-click store creation
  - Built-in Baray payments (bank + KHQR)
  - Ideal distribution channel for crypto payment adoption
  - Existing merchant relationships and trust
  - Multi-vendor marketplace capability

---

## II. THREE-STABLECOIN STRATEGY

Selendra will operate three distinct stable value tokens, each serving different purposes:

### 1. CPL STAR (Fixed at $0.01 USD)

**Type:** Sports loyalty token / stable voucher
**Backed by:** CPL organization promise
**Current Status:** ✅ Live with $10K on-chain

**Use Cases:**
- Sports predictions and rewards
- CPL merchandise redemption
- Fan engagement and loyalty
- Sports-specific transactions
- Stadium payments

**Advantages:**
- Already live with 30K users
- Easy mental math (100 STAR = $1)
- Integrated with sports ecosystem
- Cultural/emotional connection to CPL
- Simple for non-crypto users

**Limitations:**
- Only redeemable for CPL-related items
- Centralized (CPL controls supply)
- Not general-purpose money
- Sports-specific utility

**Strategy:** Keep STAR within sports ecosystem. Do not compete with general stablecoins.

---

### 2. sUSD (Fixed at $1.00 USD)

**Type:** 1:1 wrapped bridge token (simple deposit/redeem)
**Backed by:** 100% USDT/USDC reserves
**Model:** Wrapped stablecoin (not over-collateralized)

**Use Cases:**
- Bridge USDT/USDC to Selendra
- Base trading pair on DEX
- Collateral for KHRt minting
- DeFi composability
- Simpler alternative to direct USDT/USDC use

**Advantages:**
- **Capital efficient** (100% vs 150% for over-collateralized)
- **Simple to understand** (deposit USDT → get sUSD, burn sUSD → get USDT back)
- **Fast to build** (4-6 weeks vs 10-12 weeks for CDP model)
- **Easy to audit** (much less complex code)
- **No liquidations** (no complex mechanisms needed)
- Works seamlessly with all DeFi protocols
- No bank dependencies

**Limitations:**
- Not "decentralized money" like DAI (but that's fine - KHRt is your stablecoin innovation)
- Requires USDT/USDC bridged first
- Centralized custody of reserves (mitigated by transparency)

**Strategy:** Bridge token to bring external liquidity to Selendra. Keep it simple. Focus complexity on KHRt (your differentiator).

---

### 3. KHRt (Fixed at 1 KHR, ~$0.00025 USD)

**Type:** Local currency stablecoin (RWA-backed)
**Backed by:** KHR in bank accounts (via Baray) + sUSD/USDT collateral
**Model:** Dual-collateralized (real-world asset + crypto)

**Use Cases:**
- Payments to merchants
- Remittances (Cambodia workers abroad)
- Payroll and salaries
- Cash in/out via KHQR (Baray)
- Daily transactions
- Savings (better yield than banks)

**Advantages:**
- Matches local currency (familiar to Cambodians)
- Bank integration via Baray (easy cash out)
- KHQR compatible (7M+ users already use KHQR)
- Payment-focused utility
- Solves real problems (remittances, payments)

**Limitations:**
- Requires banking relationships (have ✅)
- Regulatory complexity
- Cambodia-specific (until regional expansion)
- Requires significant reserves

**Strategy:** YOUR KEY DIFFERENTIATOR. This is what no other blockchain has.

---

### How They Work Together

```
User Flow Example 1: Sports Fan
KHQR payment → KHRt in wallet → Swap to STAR → Predict matches →
Win STAR → Redeem merchandise

User Flow Example 2: DeFi User
Bridge USDT → Deposit for sUSD (1:1) → Earn yield → Use as collateral for KHRt →
Cash out to bank

User Flow Example 3: Merchant
Customer pays in KHRt → Merchant holds or swaps to sUSD →
Earns yield → Cashes out to bank via Baray

User Flow Example 4: Remittance
Worker abroad deposits USDT for sUSD (1:1) → Sends to family in Cambodia →
Family converts to KHRt → Cashes out to bank account
```

### Key DEX Trading Pairs
- **STAR ↔ KHRt** (sports fans to payment currency)
- **KHRt ↔ sUSD** (local to global stablecoin)
- **sUSD ↔ USDT/USDC** (crypto on/off ramp)
- **SEL ↔ all stable assets** (native token base pairs)

---

## III. CORE INFRASTRUCTURE APPS

*These are foundational blockchain applications that power the entire ecosystem.*

### 1. Unified Selendra Wallet (ENHANCE EXISTING)

**Status:** 🏗️ Custodial wallets exist, need major enhancements
**Platform:** Mobile-first (iOS + Android via React Native) + Web
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - This is your super app

**Current State:**
- 30,000 custodial wallets on StadiumX
- Hold STAR + player cards
- Email/password login

**Required Enhancements:**

**A. Multi-Token Support:**
- SEL (native token)
- USDT/USDC (bridged)
- sUSD (Selendra stablecoin)
- KHRt (Cambodian Riel token)
- STAR (CPL token)
- Fan tokens
- All ERC-20 tokens on Selendra

**B. Integrated DEX:**
- In-app token swaps (STAR ↔ SEL ↔ KHRt ↔ sUSD)
- Simple UI (Coinbase-style, not complex DeFi)
- Gasless for users (custodial = you handle gas)
- Real-time price quotes
- Slippage protection

**C. Baray Integration (Critical):**
- "Add Money" button → KHQR code → pay from bank → KHRt appears
- "Cash Out" button → enter bank account → KHRt burns → KHR to bank
- Saved bank accounts
- Transaction limits and KYC
- Real-time status updates

**D. Core Features:**
- Send/receive to contacts (phone, username, or address)
- Transaction history with filters
- Portfolio dashboard (total value in KHR/USD)
- QR code payments (generate/scan)
- Request payment feature
- Recurring payments
- Contact list integration

**E. Staking Interface:**
- Stake SEL tokens
- View rewards in real-time
- Claim rewards (in SEL or KHRt)
- Unstake with cooldown timer

**F. UX/Security:**
- Khmer + English language
- Biometric security (Face ID, fingerprint)
- Social recovery (trusted contacts)
- 2FA for sensitive operations
- Push notifications
- Dark/light mode

**Dependencies:** DEX contracts, KHRt contracts, Baray API integration, staking contracts
**Timeline:** Enhance incrementally over 3-4 months alongside infrastructure development
**Team:** 1-2 developers + Claude Code

---

### 2. Decentralized Exchange (DEX) (BUILD NEW)

**Status:** 🏗️ Does not exist, must build
**Platform:** Smart contracts + Web interface + Mobile integration
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Needed before STAR becomes tradeable

**Model:** AMM-based (fork Uniswap V2 or V3)

**Features:**

**A. Core Functionality:**
- Token swaps with optimal routing
- Liquidity pools with LP tokens
- Add/remove liquidity
- Yield farming / liquidity mining incentives
- Fee structure (0.3% standard, 0.05% stablecoin pairs)

**B. Priority Pools (Launch Order):**
1. STAR/KHRt (enables STAR utility)
2. KHRt/sUSD (stablecoin liquidity)
3. SEL/sUSD (base liquidity pair)
4. sUSD/USDT (bridge to external stablecoins)
5. SEL/STAR (sports fan onboarding)

**C. User Interface:**
- Simple swap interface (Coinbase-style)
- Charts and price history
- Liquidity provider dashboard
- Impermanent loss calculator
- APY/APR display for all pools
- 24h volume and TVL

**D. Integration:**
- Built into custodial wallet (gasless swaps)
- Web interface for advanced users
- API for external integrations

**Dependencies:** None (can start immediately)
**Timeline:** 
- Smart contracts: 3-4 weeks
- UI: 2 weeks
- Testing: 1 week
- Total: 6-7 weeks
**Team:** 1 developer (smart contracts) + 1 developer (frontend) + Claude Code

---

### 3. Staking Portal (BUILD NEW)

**Status:** 🏗️ Does not exist, must build
**Platform:** Web app + Mobile integration
**Priority:** ⭐⭐⭐⭐ HIGH - Critical for L1 utility

**Features:**

**A. Basic Staking:**
- Stake SEL tokens to validators
- Select validators (performance, commission, uptime)
- View staking rewards in real-time
- Claim rewards (in SEL or KHRt option)
- Unstake with cooldown period (7-28 days)
- Validator information and statistics

**B. Advanced Features:**
- APY calculator based on amount and duration
- Validator set information
- Delegation management
- Auto-compounding option
- Batch operations (stake to multiple validators)

**C. Phase 2 (Future):**
- Liquid staking (stSEL tradeable token)
- Governance participation
- Validator nomination

**D. Analytics:**
- Total staked SEL
- Network staking ratio
- Historical rewards
- Performance charts

**Dependencies:** Staking should exist at protocol level (Substrate pallet)
**Timeline:** 4-6 weeks (mostly frontend + integration)
**Team:** 1 developer + Claude Code

---

### 4. Bridge (Cross-Chain) (BUILD NEW - Integration)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Web interface
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Needed for external liquidity

**IMPORTANT:** Do NOT build bridge from scratch. Integrate existing solution.

**Recommended:** LayerZero (most flexible) or Axelar

**Supported Chains (Priority Order):**
1. Ethereum (for USDT/USDC - most critical)
2. BSC (popular in Asia, cheap)
3. Polygon (low cost alternative)
4. Arbitrum/Optimism (L2 liquidity)

**Assets to Bridge:**
- USDT (critical for sUSD reserves)
- USDC (critical for sUSD reserves)
- WETH (DeFi utility)
- WBTC (DeFi utility)
- Major tokens (UNI, AAVE, etc.)

**Features:**
- Simple bridge interface
- Estimated time and fees
- Transaction tracking
- History of bridges
- Support for both directions

**Security:**
- Multi-sig controls
- Rate limiting
- Emergency pause
- Regular audits

**Dependencies:** Partnership/integration with LayerZero or Axelar
**Timeline:** 6-8 weeks (mostly integration and testing)
**Team:** 1 developer + bridge protocol support + Claude Code

---

### 5. sUSD Stablecoin System (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Web interface + Mobile integration
**Priority:** ⭐⭐⭐⭐ HIGH - Foundation for KHRt

**Model:** Simple 1:1 wrapped stablecoin (deposit/redeem)

**Why This Model:**
- **Capital efficient**: 100% backing (not 150%+)
- **Fast to build**: 4-6 weeks vs 10-12 weeks for over-collateralized
- **Easy to audit**: Much simpler code, fewer attack vectors
- **Small team friendly**: Less complexity = faster iteration
- **Job is simple**: Bridge USDT/USDC to Selendra for DEX and KHRt
- **Focus on KHRt**: Your real stablecoin innovation, not this

**Smart Contracts:**

**A. Core Contracts:**
- sUSD ERC-20 token contract
- Reserve manager contract (holds USDT/USDC)
- Deposit contract (USDT/USDC → sUSD 1:1)
- Redeem contract (sUSD → USDT/USDC 1:1)
- Multi-sig admin controls
- Emergency pause functionality

**B. How It Works:**

```
Deposit:
User sends 1000 USDT → Contract locks USDT → Mints 1000 sUSD to user

Redeem:
User burns 1000 sUSD → Contract unlocks 1000 USDT → Sends to user
```

**C. No Liquidations Needed:**
- No collateralization ratios
- No health factors
- No Dutch auctions
- No oracles for minting (still need for DEX pairs)
- No stability fees
- No debt ceilings

**Features:**

**A. User Interface (Simple):**
- **Deposit Tab**:
  - Enter USDT or USDC amount
  - See you'll get equal sUSD
  - Confirm and deposit
- **Redeem Tab**:
  - Enter sUSD amount
  - Choose USDT or USDC back
  - Confirm and redeem
- Transaction history
- Current balance display
- Exchange rate (always 1:1)

**B. Proof of Reserves Dashboard (Public):**
- Real-time USDT reserves in contract
- Real-time USDC reserves in contract
- Total sUSD minted
- Reserve ratio (should always be 100%+)
- Public API for verification
- Contract addresses displayed

**C. Parameters:**
- Deposit minimum: 10 USDT/USDC (prevent spam)
- Redeem minimum: 10 sUSD
- Fee: 0% (or tiny 0.1% for operations)
- No maximum limits (scale naturally)

**D. Safety Mechanisms:**
- Multi-sig for admin functions
- Emergency pause (if issue detected)
- Timelock for parameter changes
- Regular audits
- Transparent reserves (on-chain)

**Use Cases:**
- Bridge USDT/USDC to Selendra quickly
- Base pair for DEX trading
- Collateral for KHRt minting
- Simple stablecoin for DeFi composability

**Optional Phase 2 (If Needed Later):**
If you want to add volatile collateral later (e.g., let users deposit SEL to mint sUSD), you can add that as a separate module with proper collateralization ratios. But start simple.

**Dependencies:** Bridge (for USDT/USDC to arrive on Selendra)
**Timeline:**
- Smart contracts: 2-3 weeks
- Audits: 1-2 weeks
- UI: 1-2 weeks
- Total: 4-6 weeks (much faster!)
**Team:** 1 developer (smart contracts + frontend) + auditor + Claude Code

---

### 6. KHRt Stablecoin System (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Backend service + Web/Mobile interface
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Your key differentiator

**Model:** Dual-collateralized RWA-backed stablecoin
- On-chain: sUSD or USDT/USDC
- Off-chain: KHR in bank accounts (via Baray)
- Start with 100% fiat backing, evolve to hybrid

**Architecture Components:**

**A. Smart Contracts:**
- KHRt ERC-20 token
- Minting/burning mechanisms
- Reserve tracking (on-chain portion)
- Multi-sig controls
- Emergency pause functionality
- Access control (only authorized minters)

**B. Reserve Management Backend:**
- Integration with Baray API
- Real-time bank account monitoring (ABA, ACLEDA, WING)
- Automated reconciliation (hourly)
- Reserve calculations
- Audit trail (immutable logs)
- Alerting system (low reserves, discrepancies)

**C. Minting Service (Backend):**
- KHQR payment flow: User pays → Baray confirms → Auto-mint KHRt
- Withdrawal flow: User requests → Burns KHRt → Baray API sends KHR to bank
- KYC/AML integration
- Rate limiting (prevent abuse)
- Fraud detection
- Transaction limits (start conservative)

**D. Proof of Reserves Dashboard (Public):**
- Real-time on-chain reserves (sUSD/USDT/USDC)
- Real-time off-chain reserves (KHR in banks)
- Total KHRt minted
- Collateralization ratio (should be >100% always)
- Historical data and charts
- Third-party attestations (monthly audits)
- Public API for verification

**E. User Interfaces:**
- "Add Money": Generate KHQR → Pay from bank → KHRt appears
- "Cash Out": Enter bank details → Burn KHRt → Receive KHR
- Transaction history
- Reserve transparency view
- Exchange rate display (KHR/USD)

**Launch Strategy (Conservative):**

**Month 1-2: Development + Audits**
- Smart contracts
- Backend integration with Baray
- Security audits
- Internal testing

**Month 3: Closed Beta**
- 100 users (invite-only)
- $10,000 KHRt cap
- Monitor closely
- Fix issues

**Month 4: Expanded Beta**
- 1,000 users
- $100,000 KHRt cap
- Add more features
- Gather feedback

**Month 5: Open Beta**
- 10,000 users
- $500,000 KHRt cap
- Marketing push
- Merchant partnerships

**Month 6: Public Launch**
- All users
- No cap (or high cap like $5M)
- Full marketing
- Regional expansion planning

**Security & Compliance:**
- Multiple bank accounts for redundancy
- Daily reserve reconciliation
- Real-time monitoring
- Regular third-party audits
- Full KYC/AML compliance
- Work with National Bank of Cambodia
- Emergency pause mechanisms
- Insurance fund (reserve buffer)

**Dependencies:** 
- Baray API (✅ exists)
- Banking relationships (✅ have)
- sUSD or USDT/USDC (for crypto collateral)
- Legal/compliance clearance

**Timeline:** 
- Smart contracts + backend: 8-10 weeks
- Baray integration: 2-3 weeks
- Audits: 3-4 weeks
- Beta phases: 3 months
- Total: ~6 months to public launch

**Team:** 2 developers (smart contracts + backend) + 1 frontend + legal counsel + Claude Code

---

### 7. Block Explorer Enhancement (ENHANCE EXISTING)

**Status:** ✅ explorer.selendra.org exists, needs improvements
**Platform:** Web
**Priority:** ⭐⭐⭐ MEDIUM - Nice to have but not critical

**Current:** Basic block explorer

**Enhancements Needed:**

**A. Performance:**
- Faster page loads
- Better indexing
- Real-time updates (WebSocket)
- Search improvements

**B. Features:**
- Token tracker (all ERC-20s)
- Wallet portfolio view
- Transaction decoder (human-readable)
- Contract verification tool
- Analytics dashboard (TVL, gas, transactions)
- Top tokens, accounts, contracts

**C. Developer Tools:**
- API documentation
- API endpoints for developers
- Webhooks
- Export data (CSV, JSON)

**D. User Experience:**
- Clean, modern UI
- Mobile responsive
- Dark mode
- Khmer language support

**Timeline:** Ongoing improvements, major update 6-8 weeks
**Team:** 1 developer + Claude Code

---

## IV. SPORTS ECOSYSTEM APPS

*These apps integrate with StadiumX/CPL Play and drive user acquisition through sports engagement.*

### 8. Enhanced CPL Play Prediction Platform (ENHANCE EXISTING)

**Status:** ✅ Basic prediction game exists, needs major enhancements
**Platform:** Web + Mobile (integrated into StadiumX)
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Key engagement driver for Kun Khmer launch

**Current State:**
- Basic prediction game: Stake STAR, predict match outcome, win from pool
- 10K users, 2K weekly active

**Required Enhancements:**

**A. Expanded Football Prediction Types:**
- Exact score predictions
- First goal scorer
- Half-time / full-time results
- Player performance (goals, assists, cards)
- Multi-game parlays
- Live in-game predictions (score next goal, next card, etc.)
- Team vs team head-to-head

**B. Kun Khmer Predictions (NEW - Critical):**
- Fight outcome (winner by decision, KO, submission)
- Method of victory
- Round prediction
- Fighter vs fighter matchups
- Special techniques used
- Fight duration
- 10 events/week = massive engagement opportunity

**C. Improved UX/Features:**
- Social features (see friends' predictions, challenges)
- Leaderboards (weekly, monthly, all-time, per club)
- Prediction history and detailed stats
- Win rate tracking by sport, team, fighter
- Badges and achievements
- Push notifications (matches starting, results, winnings)
- Prediction insights (crowd sentiment, odds)

**D. Advanced Gameplay:**
- Paid entry pools (bigger prizes, higher stakes)
- Private pools (create with friends)
- Tournament-style competitions
- Prediction marketplace (trade prediction positions)
- Boosted predictions (use tokens for multipliers)
- Streak bonuses (consecutive correct predictions)

**E. Multi-Token Support:**
- Pay entry fees in STAR, KHRt, or sUSD
- Win in your preferred token
- Cross-token prize pools
- Auto-convert winnings

**F. Gamification:**
- Experience points and levels
- Unlock new prediction types as you level up
- Special events and tournaments
- Seasonal championships
- Hall of fame

**Dependencies:** Enhanced wallet, KHRt (for multi-token), real-time data feeds
**Timeline:** 6-8 weeks to prepare for Kun Khmer launch (by September 2025)
**Team:** 1-2 developers + data feed integration + Claude Code

---

### 9. NFT Ticketing System (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + StadiumX integration
**Priority:** ⭐⭐⭐⭐ HIGH - Ready for Kun Khmer launch

**Features:**

**A. Core NFT Ticketing:**
- Every ticket = ERC-721 NFT on Selendra
- Minted automatically when purchased on StadiumX
- Transferred to user's custodial wallet
- QR code for stadium entry (verifies NFT ownership)
- Ticket metadata (seat, date, event, venue)

**B. Collection & History:**
- Ticket history (all events attended)
- Digital memorabilia collection
- Special badges for historic events (CPL final, national team win)
- Proof of attendance protocol (POAP-style)
- Share attended events on social

**C. Secondary Marketplace:**
- Peer-to-peer ticket sales
- Set your own price
- Platform takes small fee (2-3%)
- Dynamic pricing (tickets appreciate/depreciate)
- Fraud prevention (smart contract enforced)
- Royalties to clubs/league

**D. Season Ticket NFTs:**
- Season passes as NFTs
- Transferable or non-transferable options
- Special perks for holders
- Upgrade/downgrade options
- Loyalty rewards

**E. VIP & Experience NFTs:**
- Meet & greet packages
- Backstage access
- VIP lounge access
- Signed merchandise bundles
- Training session attendance

**Use Cases:**
- CPL football matches
- Kun Khmer fights
- Cambodia national team games
- Special events and tournaments

**Benefits:**
- Prevents counterfeiting (blockchain verified)
- Enables secondary market (increases liquidity)
- Creates collectible value
- Fan engagement tool
- Revenue stream for clubs

**Dependencies:** ERC-721 standard, wallet integration, StadiumX integration
**Timeline:** 6-8 weeks, launch with Kun Khmer (November 2025)
**Team:** 1 developer (smart contracts + integration) + Claude Code

---

### 10. Fan Token Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Web/Mobile interface
**Priority:** ⭐⭐⭐⭐ HIGH - Strong engagement and revenue tool

**Model:** Each club/team gets its own fungible ERC-20 token

**Initial Launch (Pilot - September 2025):**
- 1 major CPL club (biggest fanbase - test with scale)
- 1 smaller CPL club (easier to iterate)
- Cambodia national team token (national pride factor)

**Token Utilities:**

**A. Governance:**
- Vote on club decisions (jersey design, friendly matches, player signings)
- Propose initiatives
- 1 token = 1 vote
- Voting periods and quorums

**B. Exclusive Access:**
- Exclusive merchandise (token holders only)
- Limited edition drops
- Early ticket access (presale)
- VIP experiences (meet players, locker room tours)
- Exclusive content

**C. Revenue Sharing:**
- % of ticket sales distributed to token holders
- % of merchandise sales
- Sponsorship revenue share
- Transparent on-chain distribution

**D. Rewards & Benefits:**
- Discounted tickets (show token balance)
- Priority seating
- Free merchandise at thresholds
- Access to private fan communities
- Special events

**E. Staking:**
- Stake fan tokens to earn rewards
- Boosted voting power when staked
- Loyalty rewards

**Platform Features:**

**A. Token Launch:**
- Fair distribution mechanism (prevent whales)
- Initial token offering
- Vesting schedules for team/players
- Liquidity pool seeding

**B. Club Dashboard:**
- Clubs manage token utilities
- Create votes/proposals
- Engage with token holders
- Analytics (holder distribution, voting participation)
- Revenue tracking

**C. Fan Interface:**
- Buy/sell fan tokens on DEX
- View token utilities
- Participate in governance
- Track benefits and rewards
- Holder benefits tracking

**D. Multi-Token Support:**
- Hold multiple club tokens
- Portfolio view
- Comparative stats

**Expansion Plan:**
- September 2025: Launch 2-3 pilot tokens
- November 2025: Add 2-3 more clubs (with Kun Khmer)
- Q1 2026: Expand to all 11 CPL clubs
- Q2 2026: Individual player tokens
- Q3 2026: Kun Khmer fighter tokens

**Dependencies:** DEX (for trading), wallet, club partnerships
**Timeline:** 6-8 weeks for platform, pilot launch September 2025
**Team:** 1 developer (smart contracts) + 1 developer (frontend) + Claude Code

---

### 11. Athlete/Fighter Token Platform (BUILD NEW)

**Status:** 🏗️ Does not exist (build after fan tokens proven)
**Platform:** Smart contracts + Web/Mobile interface
**Priority:** ⭐⭐⭐ MEDIUM - Launch after fan token platform works

**Model:** Individual athletes get personal ERC-20 tokens

**Target Athletes:**
- Star football players (Keo Sokpheng, etc.)
- Cambodia national team players
- Top Kun Khmer fighters
- Rising stars

**Token Utilities:**

**A. Performance-Linked:**
- Token value tied to real performance
- Win → price increase pressure
- Goals/KOs → bonus distributions
- Automated performance tracking

**B. Exclusive Content:**
- Training videos from athlete
- Behind-the-scenes access
- Diet and fitness tips
- Personal updates
- Live Q&A sessions

**C. Fan Engagement:**
- Direct messaging (token-gated)
- Virtual meet & greets
- Exclusive merchandise
- Birthday/milestone celebrations
- Fan challenges

**D. Revenue Sharing:**
- Share in athlete's endorsement revenue
- Merchandise sales
- Appearance fees
- Prize winnings

**E. Collectible/Investment:**
- Early supporter benefits
- Token appreciation potential
- Trade on secondary market
- Fantasy sports integration

**Benefits for Athletes:**
- New revenue stream
- Deeper fan connection
- Direct monetization
- Global reach

**Launch Strategy:**
- Start with 2-3 top athletes (1 footballer, 1 fighter)
- Prove model works
- Expand to more athletes

**Dependencies:** Fan token platform, wallet, athlete partnerships
**Timeline:** 4-6 weeks after fan token platform proven (Q1 2026)
**Team:** 1 developer + Claude Code

---

### 12. Sports Merchandise & Rewards Marketplace (ENHANCE EXISTING)

**Status:** ✅ Basic STAR redemption exists, needs major expansion
**Platform:** Web + Mobile (integrated into wallet/StadiumX)
**Priority:** ⭐⭐⭐⭐ HIGH - Drives token utility and engagement

**Current State:** STAR redeemable for CPL merchandise

**Enhancements Needed:**

**A. Expanded Product Catalog:**
- All 11 CPL clubs merchandise
- Kun Khmer fight gear (shorts, wraps, gloves)
- Cambodia national team jerseys, scarves, hats, flags
- Signed memorabilia (jerseys, balls, photos)
- Training equipment
- Collectible items
- Digital collectibles (video highlights, moments)
- Limited edition items

**B. Multi-Token Payment:**
- Accept STAR, KHRt, sUSD, SEL
- Dynamic pricing (real-time conversion)
- Token-specific discounts (pay in STAR = 5% off)

**C. Partner Products (Beyond Sports):**
- Restaurant vouchers (near stadiums)
- Hotel discounts (for away game travel)
- Entertainment (movie tickets, concerts)
- Travel packages (stadium tours, away games)
- Tech products and gadgets
- Subscription services

**D. Experiences (Not Just Physical Goods):**
- Stadium tours
- Meet & greet with players/fighters
- Training sessions (learn from pros)
- VIP game access (sideline seats)
- Behind-the-scenes access
- Commentary box experience
- Pre-match meals with players

**E. Loyalty Integration:**
- Earn STAR for purchases (cashback)
- Tiered discounts (bronze/silver/gold members)
- Referral rewards (invite friends, earn STAR)
- Birthday bonuses
- Anniversary rewards

**F. Marketplace Features:**
- User reviews and ratings
- Wishlist
- Gift to other users
- Inventory tracking (low stock alerts)
- Pre-orders for new items
- Auction for rare items

**G. Physical + Digital:**
- Buy physical item → receive NFT certificate
- Verified authenticity
- Digital twin for collectibles

**Target:** 50+ redemption options by Q3 2025, 100+ by Q4 2025

**Dependencies:** Payment gateway, inventory management, partner integrations
**Timeline:** Continuous expansion, major update by August 2025
**Team:** 1 developer + partnerships manager + Claude Code

---

### 13. Fantasy Sports Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web + Mobile
**Priority:** ⭐⭐⭐ MEDIUM - High engagement but complex

**Features:**

**A. CPL Fantasy Football:**
- Draft player cards (as NFTs)
- Weekly lineup management (formation, subs)
- Scoring based on real CPL match performance
- Transfers and trades
- Multiple league formats (season-long, weekly)
- Prize pools in STAR/KHRt

**B. Kun Khmer Fantasy Leagues:**
- Pick fighters for event cards
- Score based on fight outcomes
- Weekly or event-based scoring
- Knockout bonuses
- Special technique bonuses

**C. Player Card System:**
- Player/fighter cards as NFTs
- Rarity levels (common, rare, legendary)
- Card packs (buy with STAR/KHRt)
- Player stats and history on card
- Upgrade cards based on performance

**D. League Management:**
- Public leagues (join anyone)
- Private leagues (friends only)
- League chat and trash talk
- Custom rules and scoring
- Multiple leagues simultaneously

**E. Trading & Marketplace:**
- Trade player cards with others
- Card marketplace (buy/sell)
- Card value tied to player performance
- Limited edition cards

**F. Live Scoring:**
- Real-time updates during matches/fights
- Live leaderboards
- Push notifications for big plays
- Watch party features

**G. Prizes:**
- Weekly/monthly/season prizes
- STAR, KHRt, or merchandise
- Exclusive NFTs for winners
- Hall of fame for champions

**Monetization:**
- League entry fees (platform takes 5-10%)
- Card pack sales
- Trading fees (2%)
- Premium features (advanced stats, insights)

**Data Requirements:**
- Real-time CPL match data
- Kun Khmer fight data
- Player/fighter statistics
- Performance tracking

**Dependencies:** NFT platform, real-time data feeds, payment integration
**Timeline:** 10-12 weeks, launch Q1 2026 after Kun Khmer established
**Team:** 2 developers (complex gameplay logic) + data integration + Claude Code

---

### 14. Loyalty & Rewards Program (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Backend system + Wallet integration
**Priority:** ⭐⭐⭐⭐ HIGH - Drives retention and engagement

**How Users Earn STAR:**

**A. Event Attendance:**
- Attend CPL match → earn 10-50 STAR (based on seat)
- Attend Kun Khmer fight → earn 10-50 STAR
- Attend national team game → earn 100 STAR (premium)
- Check-in at venue (geo-verified)

**B. Purchases:**
- Buy merchandise → earn 5-10% back in STAR
- Buy tickets → earn STAR
- Use payment gateway → earn STAR

**C. Predictions:**
- Participate in predictions → bonus STAR
- Win streaks → multiplier bonuses
- Correct predictions → base rewards

**D. Social Engagement:**
- Refer friends → earn 50-100 STAR per signup
- Share on social media → earn STAR
- Write reviews → earn STAR
- Fan content creation → earn STAR

**E. Challenges:**
- Daily challenges (predict today's match)
- Weekly challenges (attend 2 games)
- Monthly challenges (complete all of the above)
- Special event challenges

**Loyalty Tiers:**

**Bronze Tier (0-1,000 lifetime STAR earned):**
- Standard benefits
- Basic predictions

**Silver Tier (1,000-10,000 lifetime STAR):**
- 5% discount on merchandise
- Priority ticket access
- Exclusive predictions

**Gold Tier (10,000-50,000 lifetime STAR):**
- 10% discount on merchandise
- VIP seating options
- 1.5x prediction multipliers
- Meet & greet access
- Exclusive content

**Platinum Tier (50,000+ lifetime STAR):**
- 15% discount on merchandise
- Best seat selection
- 2x prediction multipliers
- Locker room tours
- Player dinner events
- Custom experiences

**Features:**

**A. Tracking:**
- All loyalty points as on-chain tokens
- Transparent and verifiable
- Real-time balance updates
- Historical earnings

**B. Tier Management:**
- Automatic tier upgrades
- Tier benefits clearly displayed
- Notifications for tier changes
- Annual tier review

**C. Gamification:**
- Progress bars
- Achievement unlocks
- Milestone celebrations
- Leaderboard rankings

**D. Portability:**
- STAR earned is portable across all clubs
- Use at any CPL venue
- Works for both football and Kun Khmer

**Dependencies:** Wallet, STAR token, event tracking system
**Timeline:** 6-8 weeks, launch with Kun Khmer (November 2025)
**Team:** 1 developer + Claude Code

---

### 15. Social Fan Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web + Mobile (could integrate into wallet app)
**Priority:** ⭐⭐⭐ MEDIUM - Drives community engagement

**Features:**

**A. User Profiles:**
- Display name and avatar
- Show attended games (NFT tickets)
- Show owned NFTs and tokens
- STAR holdings (optional visibility)
- Predictions record and win rate
- Badges and achievements
- Favorite clubs and fighters

**B. Social Connections:**
- Follow other fans
- Find friends (phone contacts)
- Fan groups (club-specific, fighter-specific)
- Private messaging
- Group chats

**C. Content & Engagement:**
- Match discussion threads
- Post predictions publicly
- Share attended events
- Photo/video uploads from games
- Live match commentary
- Reaction and emoji system
- Trending topics

**D. Social Tipping:**
- Send STAR to other fans
- Tip for great content
- Reward good predictions
- Support fellow fans

**E. Fan Rankings:**
- Leaderboards (predictions, attendance, engagement)
- Club-specific rankings
- Weekly/monthly/all-time
- Badges for top fans
- Competitive elements

**F. Community Features:**
- Club-specific chat rooms
- Pre-match hype threads
- Post-match analysis
- Trash talk (moderated)
- Fan polls and votes

**G. User-Generated Content:**
- Photos from stadium
- Video highlights
- Memes and gifs
- Fan art
- Match reports

**Privacy Controls:**
- Optional wallet visibility
- Choose what to share
- Block/report users
- Pseudonymous or real name
- Privacy settings granular control

**Moderation:**
- Community guidelines
- Automated content filtering
- User reporting system
- Moderator team
- Appeal process

**Dependencies:** Wallet (for identity and payments), content moderation tools
**Timeline:** 8-10 weeks, launch Q4 2025 with Kun Khmer
**Team:** 1-2 developers (social features are complex) + moderator + Claude Code

---

### 16. Content Platform - Streaming & Media (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web + Mobile
**Priority:** ⭐⭐ LOW-MEDIUM - Nice revenue stream but not critical

**Features:**

**A. Video Content:**
- Match highlights (2-5 min recaps)
- Full match replays
- Player/fighter interviews
- Behind-the-scenes footage
- Training videos and tutorials
- Documentary series
- Historical matches

**B. Live Streaming:**
- Live matches/fights (pay-per-view for international)
- Pre-match shows
- Post-match analysis
- Training sessions live
- Special events

**C. Educational Content:**
- Football tactics explained
- Kun Khmer technique tutorials
- Training programs
- Nutrition and fitness
- Youth development content

**D. Exclusive Content:**
- Token-holder only content
- Premium subscriber content
- Early access for fans
- Ad-free options

**Access Models:**

**A. Pay-Per-View:**
- Single match/fight access
- Pay with STAR, KHRt, or sUSD
- 24-48 hour access window

**B. Subscriptions:**
- Monthly access (e.g., 1000 STAR/month)
- Season pass
- All-access pass

**C. Free Tier:**
- Ad-supported
- Limited content
- Highlights only

**D. Token-Gated:**
- Fan token holders get free access
- Tiered access based on holdings

**Revenue Split:**
- CPL/Kun Khmer organizations
- Individual clubs/fighters
- Selendra (platform fee)
- Content creators

**Content Licensing:**
- Negotiate rights with CPL
- Negotiate rights with Kun Khmer organizations
- Individual fighter agreements
- Archive footage

**Platform Features:**
- Video player (adaptive bitrate)
- Download for offline
- Chromecast/AirPlay
- Watchlist and favorites
- Continue watching
- Recommendations

**Dependencies:** Content licensing deals, video infrastructure (CDN), DRM
**Timeline:** Q2 2026 (after core features established)
**Team:** 1-2 developers + content partnerships + Claude Code

---

## V. PAYMENT & FINTECH APPS

*These apps leverage Baray to connect crypto to the real economy, making Selendra useful for daily transactions.*

### 17. Merchant Payment Gateway (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Backend API + POS apps + Web plugins
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Drives real-world adoption

**Components:**

**A. Point-of-Sale App (In-Person Payments):**
- Generate payment QR code (KHQR-compatible or wallet address)
- Customer scans and pays with KHRt, sUSD, STAR, or SEL
- Instant on-chain confirmation
- Receipt generation (digital + printable)
- Multi-employee support (different accounts/permissions)
- Daily/weekly sales reports
- Inventory tracking (optional)
- Tax calculation
- Cash out to bank via Baray (same-day or instant)

**B. E-commerce Integration:**
- REST API for custom integrations
- WooCommerce plugin (WordPress)
- Shopify app
- Payment links (no-code solution - just share link)
- Hosted checkout page (embeddable)
- Webhook notifications
- Recurring billing support

**C. Features:**
- Accept multiple tokens (KHRt, sUSD, STAR, SEL)
- Auto-conversion to merchant's preferred currency
- Settlement options:
  - Hold in crypto (wallet)
  - Auto-convert to KHRt and hold
  - Auto-cash out to bank via Baray
- Real-time exchange rates
- Invoice generation
- Refund management
- Dispute resolution system
- Analytics dashboard (sales, top products, customer insights)

**D. Pricing (Competitive):**
- 0.5-1% per transaction (vs 3-5% for credit cards)
- No monthly fees
- No setup fees
- Same-day or instant settlement
- Volume discounts for high-volume merchants

**E. Target Merchants (Priority Order):**

**Phase 0 - Riverbase Integration (HIGHEST PRIORITY):**
- **100+ existing SME stores on Riverbase.app**
- One-click crypto payment plugin for Riverbase merchants
- Seamless upgrade: "Accept crypto payments - just enable in settings"
- Merchants already trust Baray/Riverbase (warm leads)
- Can reach 50+ merchants in weeks instead of months
- Telegram mini shop crypto payments
- Leverage existing support infrastructure

**Phase 1 - Stadium Ecosystem:**
- Stadium concession vendors (food, drinks)
- Merchandise shops at venues
- Parking near stadiums

**Phase 2 - Tourism & Hospitality:**
- Restaurants (especially near tourist areas)
- Hotels and guesthouses
- Tour operators
- Souvenir shops

**Phase 3 - Retail:**
- General retail shops
- Supermarkets
- Convenience stores
- Gas stations

**Phase 4 - Services:**
- Salons and spas
- Repair shops (phone, computer)
- Professional services

**Phase 5 - E-commerce Expansion:**
- New Riverbase merchants
- Other e-commerce platforms
- Food delivery platforms
- Ride-sharing integration

**Go-to-Market Strategy:**

**Riverbase-First Approach:**
- Build crypto payment plugin for Riverbase platform (4-6 weeks)
- Beta with 10 Riverbase merchants (June 2025)
- Roll out to 30 Riverbase merchants (July 2025)
- Scale to 50+ Riverbase merchants (August 2025)
- Achieve 50+ live merchants 2-3 months faster than cold outreach
- Expand to non-Riverbase merchants in parallel (stadiums, tourism)
- Target 100+ merchants by Kun Khmer launch (November)
- Target 200+ merchants by mid-2026

**Merchant Value Proposition:**

*For Riverbase Merchants:*
"You're already using Baray. Now accept crypto too - just flip a switch. Same 0.5% fee. Same fast settlement."

*For All Merchants:*
"Accept crypto payments. Pay only 0.5%. Get money in your bank same day."

**Dependencies:** KHRt, Baray integration, wallet, payment processing infrastructure
**Timeline:** 8-10 weeks, pilot launch June 2025
**Team:** 2 developers (backend + apps) + merchant partnerships + Claude Code

---

### 18. Remittance Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web + Mobile
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Huge market ($1.2B+ annual remittances to Cambodia)

**Use Case:** Cambodian workers abroad sending money home

**Sender Experience (Abroad):**

**A. Sending Money:**
- Enter recipient (phone number, wallet address, or bank account)
- Choose amount (in sender's currency: THB, KRW, JPY, USD, etc.)
- See real-time exchange rate to KHRt
- Fee calculator (compare with Western Union/MoneyGram)
- Payment options:
  - Buy sUSD/KHRt with credit card (via on-ramp partner)
  - Send from existing crypto wallet
  - P2P exchange (match with local exchangers)
- Confirm and send

**B. Features for Senders:**
- Recurring payments (monthly to family)
- Multiple recipients (send to different people)
- Transaction history with receipts
- Referral bonuses (invite friends, earn rewards)
- Loyalty program (frequent sender benefits)

**Recipient Experience (Cambodia):**

**A. Receiving Money:**
- Receive KHRt to Selendra wallet
- Notification via SMS/push notification
- Options:
  - Hold in wallet (maybe earn yield)
  - Cash out to bank via Baray (instant)
  - Spend at merchants that accept KHRt
  - Convert to STAR for sports activities

**B. No-Wallet Option:**
- Recipient receives SMS with claim link
- Can claim to bank account directly (via Baray)
- No need for wallet or crypto knowledge

**Pricing (Highly Competitive):**
- 0.5-2% fee (vs 7-10% for Western Union/MoneyGram)
- No hidden fees
- Real exchange rates (not inflated)
- Instant settlement (vs 1-3 days for traditional)

**Market Opportunity:**
- Cambodia receives $1.2B+ in remittances annually
- Average remittance: $200-300
- 1% market share = $12M annual volume
- 5% market share = $60M annual volume

**Corridor Focus (Priority Order):**
1. Thailand → Cambodia (largest corridor)
2. South Korea → Cambodia
3. Japan → Cambodia
4. Malaysia → Cambodia
5. Singapore → Cambodia
6. USA → Cambodia

**Partnerships:**

**A. Sending Side:**
- Partner with money exchangers in Thailand, Korea, Japan
- Integration with local payment methods
- Cash pickup locations (users deposit cash, we send crypto)

**B. Receiving Side:**
- Baray (bank integration - ✅ have)
- Cash pickup option (partner with Wing, ABA branches)

**Marketing:**
- Target worker communities abroad
- Social media in Khmer language
- Word-of-mouth (referral program)
- Compare savings vs Western Union prominently
- "Send money home for 1% instead of 10%"

**Regulatory Compliance:**
- KYC/AML for both sender and recipient
- Transaction limits (start conservative)
- Monitoring for suspicious activity
- Work with National Bank of Cambodia
- Money transmitter licenses (if required)

**Dependencies:** KHRt, Baray, wallet, international payment partners
**Timeline:** 8-10 weeks, launch Q3 2025 (before Kun Khmer)
**Team:** 2 developers + partnerships (international corridors) + compliance + Claude Code

---

### 19. Payroll Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web app (for employers) + Mobile (for employees)
**Priority:** ⭐⭐⭐⭐ HIGH - B2B revenue stream

**Use Case:** Companies pay employees in crypto/stablecoins

**For Employers:**

**A. Payroll Management:**
- Bulk payroll upload (CSV, Excel, or manual entry)
- Employee management (add/remove, role assignment)
- Pay in KHRt, sUSD, or SEL
- Schedule payments (one-time, recurring, biweekly, monthly)
- Automatic calculation of amounts
- Compliance and tax reporting
- Proof of payment (on-chain records)
- Analytics (total payroll cost, savings vs traditional)

**B. Payment Options:**
- Standard payment (monthly salary)
- Salary advances (employees request early payment)
- Bonus payments
- Contractor payments (one-off)

**C. Features:**
- Multi-currency support
- Employee categorization
- Department/team organization
- Payment history and archives
- Notifications to employees
- Integration with accounting software (QuickBooks, Xero)

**For Employees:**

**A. Receiving Salary:**
- Receive salary to Selendra wallet
- Notification when payment arrives
- View payment details
- Historical payment records

**B. Options:**
- Hold in wallet (potentially earn yield on savings)
- Auto-cash out to bank via Baray
- Partial cash out (keep some in crypto)
- Spend at merchants or for predictions

**C. Advanced Features:**
- Salary streaming (get paid by the day or hour instead of monthly)
- Instant salary advance (borrow against future salary)
- Savings goals (auto-save % of each payment)
- Tax documents generation

**Pricing:**
- Small fee per payment (0.1-0.5%)
- Or flat monthly fee for companies ($20-100 depending on size)
- Much cheaper than bank wire transfers or cash distribution

**Target Market:**

**Phase 1 - Tech & Startups:**
- Tech companies in Cambodia
- Startups (crypto-savvy)
- Remote-first companies

**Phase 2 - NGOs & International Orgs:**
- International NGOs in Cambodia
- UN agencies
- Development organizations

**Phase 3 - BPO & Call Centers:**
- Business process outsourcing companies
- Call centers
- Remote work companies

**Phase 4 - General Business:**
- SMEs
- Retail chains
- Service companies

**Value Proposition:**
- Lower fees than bank transfers
- Instant payments (vs 1-3 days for banks)
- Better employee experience
- Transparent on-chain records
- International payments easy (for remote workers)

**Dependencies:** KHRt, Baray, wallet, payroll logic, tax/accounting integration
**Timeline:** 8-10 weeks, launch Q4 2025
**Team:** 2 developers + accounting/tax advisor + Claude Code

---

### 20. Savings & Yield Platform (BUILD NEW)

**Status:** 🏗️ Does not exist (build after DeFi infrastructure ready)
**Platform:** Web + Mobile (integrated into wallet)
**Priority:** ⭐⭐⭐⭐ HIGH - Drives TVL and retention

**Features:**

**A. Simple Savings Accounts:**
- Deposit KHRt or sUSD
- Earn fixed APY (5-10%, significantly better than banks at 0.5-2%)
- Withdraw anytime (no lockup)
- Compounding interest (auto-compound or manual)
- See earnings in real-time

**B. Liquidity Provision:**
- Provide liquidity to DEX pools
- Earn trading fees + farming rewards
- See APY for each pool (STAR/KHRt, KHRt/sUSD, etc.)
- One-click add/remove liquidity
- Impermanent loss warnings and calculator

**C. Token Staking:**
- Stake SEL (native staking)
- Stake STAR (loyalty rewards)
- Stake fan tokens (club rewards)
- View all staking positions in one dashboard
- Compare APYs across all options

**D. Vault Strategies (Advanced):**
- Auto-compounding vaults (automated yield optimization)
- Optimal yield routing (moves funds to best opportunities)
- Risk-adjusted strategies (conservative, balanced, aggressive)
- One-click deposit into strategy
- Managed by smart contracts

**E. Portfolio Management:**
- Total assets earning yield
- Total earned (all-time, daily, weekly, monthly)
- Historical performance charts
- Breakdown by strategy/token
- Tax reporting (export for accountant)

**User Experience:**

**Simple Mode (Default):**
- "Earn 8% on your KHRt" - one button
- Handles all complexity behind the scenes
- See balance growing in real-time

**Advanced Mode:**
- Full control over strategies
- Manual liquidity provision
- Custom allocations
- Detailed analytics

**Marketing:**
"Better than a savings account. Earn 5-10% instead of 0.5%."

**Risk Management:**
- Clear explanation of risks (smart contract, impermanent loss, etc.)
- Risk scores for each strategy
- Insurance options (optional)
- Start with conservative caps

**Dependencies:** DEX (for liquidity), staking contracts, yield aggregation logic
**Timeline:** 6-8 weeks after DEX launched, target Q3 2025
**Team:** 1-2 developers + Claude Code

---

### 21. Lending & Borrowing Platform (BUILD NEW or PARTNER)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Web interface
**Priority:** ⭐⭐⭐ MEDIUM - Nice DeFi feature

**Recommendation:** Fork Compound or Aave rather than building from scratch (much faster and more secure)

**Features:**

**A. Supply Side (Lend):**
- Deposit assets (KHRt, sUSD, SEL, STAR)
- Earn interest from borrowers
- Interest rates algorithmically determined by supply/demand
- Withdraw anytime (if liquidity available)
- See APY for each asset

**B. Borrow Side:**
- Borrow against collateral
- Support over-collateralized loans (150-200% collateral ratio)
- Health factor monitoring (risk of liquidation)
- Liquidation warnings (notifications)
- Flexible repayment (any time, partial or full)
- Borrow multiple assets

**C. Use Cases:**

**For Individuals:**
- Borrow KHRt for business expenses while keeping SEL invested
- Leverage (borrow to buy more SEL)
- Short-term liquidity without selling assets

**For Micro-Lending (Social Impact):**
- Lower collateral requirements than traditional banks
- Smaller loan sizes ($50-$5,000)
- Better interest rates than traditional micro-lenders (15-25% APR vs 30-60%)
- More accessible credit

**D. Parameters:**
- Interest rate models (usage-based)
- Collateral factors (how much you can borrow per asset)
- Liquidation thresholds
- Liquidation penalties (incentive for liquidators)

**Social Impact Angle:**
- Provide fair credit to underserved Cambodians
- Alternative to predatory lenders
- Transparent terms (all on-chain)
- No hidden fees

**Risk Management:**
- Start with conservative parameters
- Cap total borrowed amounts initially
- Use proven oracle systems for prices
- Audited smart contracts
- Insurance fund

**Dependencies:** Price oracles (Chainlink), liquidation mechanisms, DEX (for liquidations)
**Timeline:** 10-12 weeks if forking existing protocol (6+ months if building from scratch)
**Launch:** Q4 2025 or Q1 2026
**Team:** 2 developers (if forking) + auditor + Claude Code

---

### 22. Invoice & Billing Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web app
**Priority:** ⭐⭐ LOW-MEDIUM - Nice to have for freelancers/SMEs

**Use Case:** Freelancers and small businesses need to invoice clients

**Features:**

**A. Invoice Creation:**
- Professional invoice templates (customizable)
- Add line items (description, quantity, rate)
- Calculate totals, taxes, discounts
- Add company logo and details
- Multiple currencies supported

**B. Sending & Payment:**
- Send invoice via email or shareable link
- Client pays with KHRt/sUSD via link (no wallet required for client)
- Payment tracking (paid, unpaid, overdue)
- Automatic reminders for unpaid invoices
- Receipt generation upon payment

**C. Recurring Invoices:**
- Set up recurring billing (monthly, weekly, annually)
- Auto-generate and send invoices
- Subscription-style billing

**D. Management:**
- Client management (contact details, payment history)
- Invoice history and search
- Reports (total invoiced, paid, outstanding)
- Tax calculation (integrated with Cambodia tax rules)
- Export for accounting (CSV, PDF)

**E. Payment Options:**
- Crypto payments (KHRt, sUSD, SEL)
- Optional: Traditional payment methods (credit card via partner)
- Cash out to bank via Baray

**Pricing:**
- Free tier (basic invoicing, limited invoices/month)
- Premium tier ($10-20/month for unlimited invoices, advanced features)
- Per-transaction fee (1-2%) on payments received

**Target Market:**
- Freelancers (designers, developers, consultants, writers)
- Service businesses (lawyers, accountants, clinics, agencies)
- Contractors and gig workers

**Value Proposition:**
"Professional invoicing + instant crypto payments + low fees"

**Dependencies:** Payment gateway, KHRt, email service
**Timeline:** 6-8 weeks, launch Q1 2026
**Team:** 1 developer + Claude Code

---

## VI. DEVELOPER & ECOSYSTEM APPS

*Tools and platforms to help developers build on Selendra and grow the ecosystem.*

### 23. Developer Tools Hub (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web portal
**Priority:** ⭐⭐⭐ MEDIUM - Needed for ecosystem growth

**Components:**

**A. Testnet Faucet:**
- Request testnet SEL for development
- Rate-limited (prevent abuse)
- Require social proof (Twitter/GitHub account)
- Automatic distribution

**B. Contract Verification:**
- Verify and publish smart contract source code
- Automated via API or web form
- Integration with explorer
- Support for all major compilers

**C. Documentation Portal:**
- API documentation (JSON-RPC, REST)
- SDK guides (JavaScript, Python, Go, Rust)
- Tutorials (build your first dApp, integrate wallet, etc.)
- Code examples and boilerplates
- Architecture explanations
- Best practices

**D. SDK Downloads:**
- JavaScript/TypeScript (ethers.js, web3.js, viem, wagmi)
- Python (web3.py)
- Mobile SDKs (iOS, Android)
- Integration guides

**E. Development Tools:**
- Block explorer API
- Webhooks for blockchain events
- Indexer services (SubQuery/The Graph integration)
- Local node setup guides
- Testing frameworks and tools
- Contract templates

**F. Developer Resources:**
- Grants information
- Hackathon announcements
- Community Discord/Telegram
- Office hours with core team
- Technical support

**Dependencies:** Explorer API, faucet backend
**Timeline:** Initial version 4-6 weeks, ongoing updates
**Team:** 1 developer + technical writer + Claude Code

---

### 24. Grants & Ecosystem Fund Portal (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Web app
**Priority:** ⭐⭐⭐ MEDIUM - Important for ecosystem growth

**Features:**

**A. Grant Application:**
- Online application form
- Project description and roadmap
- Team information
- Funding request (in SEL or KHRt)
- Milestone-based funding
- Supporting documents upload

**B. Review Process:**
- Admin review dashboard
- Scoring rubric
- Community voting (optional)
- Decision notifications
- Feedback to applicants

**C. Project Management:**
- Milestone tracking
- Progress updates from grantees
- Payment upon milestone completion
- Report submission
- Communication tools

**D. Ecosystem Directory:**
- Showcase all funded projects
- Project categories (DeFi, NFT, Gaming, Infrastructure, etc.)
- Links to live projects
- Updates and blog posts
- Community feedback

**E. Grant Categories:**
- DeFi protocols
- NFT projects
- Gaming and metaverse
- Infrastructure and tools
- Developer tools
- Community projects
- Education and content

**Grant Amounts:**
- Small grants: $5K-20K
- Medium grants: $20K-100K
- Large grants: $100K+

**Dependencies:** Wallet (for payments), governance (optional)
**Timeline:** 6-8 weeks, launch Q3 2025
**Team:** 1 developer + ecosystem manager + Claude Code

---

### 25. Launchpad / IDO Platform (BUILD NEW)

**Status:** 🏗️ Does not exist
**Platform:** Smart contracts + Web interface
**Priority:** ⭐⭐⭐ MEDIUM - Good for ecosystem and revenue

**Features:**

**A. Token Launch Mechanisms:**
- Fair launch (no presale, everyone equal)
- Dutch auction (price starts high, decreases)
- Fixed price sale
- Lottery/whitelist allocation
- Vesting schedules for team/advisors
- Liquidity locking

**B. Project Management:**
- Project application and vetting
- KYC verification for projects
- Smart contract audits (required)
- Marketing support
- Launch calendar

**C. User Features:**
- Browse upcoming launches
- Whitelist registration
- KYC (if required)
- Participate in sales
- Claim tokens
- Track investments

**D. Security & Compliance:**
- Project vetting process
- Community voting on projects (optional)
- Required audits
- Token lock-up verification
- Anti-bot measures (captcha, limits)
- Refund mechanisms (if launch fails)

**Use Cases:**
- New DeFi protocols launching on Selendra
- Fan tokens (clubs/athletes)
- Gaming tokens
- NFT project tokens

**Monetization:**
- Launch fee (1-3% of raise)
- Or fixed fee ($5K-20K)
- Token listings on DEX (listing fee)

**Benefits to Selendra:**
- Attracts new projects
- Creates network effects
- Drives TVL and volume
- Revenue for treasury

**Dependencies:** DEX (for liquidity), KYC provider, smart contract audit partners
**Timeline:** 8-10 weeks, launch Q4 2025
**Team:** 2 developers (smart contracts + frontend) + legal/compliance + Claude Code

---

### 26. Governance Portal (BUILD IF NEEDED)

**Status:** 🏗️ Build only if Selendra has on-chain governance
**Platform:** Web app
**Priority:** ⭐⭐ LOW - Only if governance exists at protocol level

**Features:**

**A. Proposal Creation:**
- Create proposals (parameter changes, upgrades, treasury spending)
- Proposal description and rationale
- Required deposit (prevent spam)
- Discussion period before voting

**B. Discussion Forum:**
- Proposal-specific discussions
- Community feedback
- Expert opinions
- Q&A with proposers

**C. Voting Interface:**
- Vote yes/no/abstain
- Voting power display (based on staked SEL)
- Delegate voting power to others
- Vote history

**D. Treasury Management:**
- View treasury balance
- Approve treasury spending proposals
- Track fund usage
- Transparency reports

**E. Governance Analytics:**
- Voter turnout
- Proposal success rate
- Top voters/delegates
- Governance participation trends

**Dependencies:** Governance smart contracts or Substrate pallets
**Timeline:** 6-8 weeks (if needed)
**Team:** 1 developer + Claude Code

---

## VII. IMPLEMENTATION ROADMAP BY PHASE

### Phase 1: Foundation (Q1 2025 - Months 1-3)

**Goal:** Build core DeFi infrastructure and enable STAR trading

**Apps to Build:**
1. DEX (AMM) - 6-7 weeks
2. Enhance Custodial Wallets (multi-token) - 4 weeks
3. Bridge Integration (LayerZero/Axelar) - 6-8 weeks
4. Staking Portal - 4-6 weeks
5. sUSD Stablecoin (start development) - ongoing

**Milestones:**
- ✅ STAR becomes tradeable on DEX
- ✅ Users can swap between STAR, SEL, USDT
- ✅ Bridge brings USDT/USDC from Ethereum
- ✅ SEL staking is live
- ✅ Wallet supports multiple tokens

**Success Metrics:**
- $100K+ DEX volume
- 15,000+ wallet users (from 10K CPL Play)
- 1,000+ daily active users
- 500+ SEL stakers

---

### Phase 2: Stablecoin Launch (Q2 2025 - Months 4-6)

**Goal:** Launch KHRt with Baray integration and start merchant adoption

**Apps to Build:**
6. sUSD Launch - 4-6 weeks (simpler 1:1 model = faster!)
7. KHRt Stablecoin System - 10-12 weeks
8. Baray Integration in Wallet - 3-4 weeks
9. Merchant Payment Gateway (pilot) - 8-10 weeks
10. Savings & Yield Platform - 6-8 weeks

**Milestones:**
- ✅ sUSD launched (1:1 wrapped model)
- ✅ KHRt beta with 100 users
- ✅ KHQR on-ramp working (Baray)
- ✅ Bank off-ramp working (cash out to ABA/ACLEDA/WING)
- ✅ Riverbase crypto payment plugin launched
- ✅ First 10 Riverbase merchants accepting crypto payments

**Success Metrics:**
- $50K+ KHRt minted
- $200K+ sUSD supply
- 20,000+ wallet users
- 10 Riverbase pilot merchants
- $500K+ Total Value Locked

---

### Phase 3: Sports Expansion (Q3 2025 - Months 7-9)

**Goal:** Prepare for Kun Khmer explosion with enhanced sports features

**Apps to Build:**
11. Enhanced CPL Play Predictions - 6-8 weeks
12. NFT Ticketing System - 6-8 weeks
13. Fan Token Platform (pilot) - 6-8 weeks
14. Loyalty & Rewards Program - 6-8 weeks
15. Marketplace Expansion - ongoing
16. Remittance Platform - 8-10 weeks

**Milestones:**
- ✅ Kun Khmer predictions ready (10 events/week)
- ✅ NFT tickets for CPL matches
- ✅ 2-3 fan tokens launched
- ✅ Loyalty program active
- ✅ Remittance corridors operational (Thailand, Korea)
- ✅ KHRt open beta (10,000 users)
- ✅ 50+ Riverbase merchants live with crypto payments

**Success Metrics:**
- 25,000+ CPL Play users
- 30,000+ wallet users
- 50+ merchants accepting crypto (Riverbase-led)
- $1M+ Total Value Locked
- 2K+ weekly predictions

---

### Phase 4: Kun Khmer Launch (Q4 2025 - Months 10-11, Nov 2025)

**Goal:** Massive user growth through Kun Khmer, scale merchant network

**Apps to Launch:**
17. Kun Khmer Full Platform - November launch
18. Social Fan Platform - 8-10 weeks
19. Payroll Platform - 8-10 weeks
20. Additional Fan Tokens (2-3 more clubs)
21. Fighter Tokens (pilot with 2 fighters)

**Milestones:**
- ✅ Kun Khmer launches with 10 events/week
- ✅ StadiumX hits 50,000 users (target)
- ✅ 30,000+ Selendra wallet users
- ✅ KHRt public launch (no cap)
- ✅ 100+ merchants accepting crypto (majority Riverbase)
- ✅ Social platform active with community
- ✅ Riverbase crypto payments standard feature

**Success Metrics:**
- 50,000+ StadiumX users
- 35,000+ wallet users
- 100+ merchants (Riverbase provides rapid scale)
- $2M+ Total Value Locked
- 100K+ monthly transactions
- 5+ fan tokens live

---

### Phase 5: Scale & Expand (2026 - Month 12+)

**Goal:** Regional dominance and ecosystem maturation

**Apps to Build:**
22. Fantasy Sports Platform - 10-12 weeks
23. Lending & Borrowing - 10-12 weeks
24. Developer Tools Hub - 4-6 weeks
25. Grants Portal - 6-8 weeks
26. Launchpad - 8-10 weeks
27. Content Platform (streaming) - ongoing
28. Invoice Platform - 6-8 weeks

**Expansion:**
- Thailand pilot (Thai League, Muay Thai)
- Vietnam exploration
- Regional stablecoin (THBt, VNDt)

**Milestones:**
- ✅ 100,000+ wallet users
- ✅ 200+ merchants
- ✅ $10M+ Total Value Locked
- ✅ Regional expansion begins
- ✅ Developer ecosystem thriving

**Success Metrics by Mid-2026:**
- 100,000+ wallet users
- 500K+ monthly transactions
- $10M+ TVL
- 200+ merchants
- 10+ fan/fighter tokens
- Regional presence (Thailand pilot)

---

## VIII. SUCCESS METRICS & KPIs

### By End of Q1 2025:
- **Users:** 15K wallet users, 2K daily active
- **Transactions:** 20K+ monthly
- **TVL:** $100K+
- **DEX:** $100K+ monthly volume
- **Staking:** 500+ stakers

### By End of Q2 2025:
- **Users:** 20K wallet users, 3K daily active
- **Transactions:** 50K+ monthly
- **TVL:** $500K+
- **KHRt:** $50K+ minted
- **Merchants:** 10 Riverbase pilots

### By End of Q3 2025:
- **Users:** 30K wallet users, 5K daily active
- **Transactions:** 100K+ monthly
- **TVL:** $1M+
- **KHRt:** $200K+ minted
- **Merchants:** 50 active (mostly Riverbase)

### By Kun Khmer Launch (Nov 2025):
- **Users:** 35K wallet users, 8K daily active
- **Transactions:** 150K+ monthly
- **TVL:** $2M+
- **KHRt:** $500K+ minted
- **Merchants:** 100 active (Riverbase + stadiums + tourism)

### By End of 2025:
- **Users:** 40K wallet users, 10K daily active
- **Transactions:** 200K+ monthly
- **TVL:** $3M+
- **KHRt:** $1M+ minted
- **Merchants:** 100 active
- **Fan Tokens:** 5-7 live

### By Mid-2026:
- **Users:** 100K wallet users, 20K daily active
- **Transactions:** 500K+ monthly
- **TVL:** $10M+
- **KHRt:** $5M+ minted
- **Merchants:** 200+ active
- **Regional Expansion:** Thailand pilot live

---

## IX. TECHNOLOGY STACK RECOMMENDATIONS

### Frontend
- **React 18** + **TypeScript** (current ✅)
- **Vite 5** (current ✅)
- **Tailwind CSS 3** (current ✅)
- **React Native** (for mobile apps - wallet, CPL Play)
- **wagmi v2** + **viem** (Web3 wallet/contract interactions - best in class)
- **TanStack Query** (data fetching, caching, real-time updates)
- **Zustand** (lightweight global state management)
- **Framer Motion** (animations - current ✅)

### Smart Contracts
- **Solidity 0.8.x** (EVM compatibility)
- **Hardhat** (development framework)
- **OpenZeppelin** (audited contract libraries)
- **Chainlink** (price oracles for KHRt and DEX pairs - not needed for sUSD with 1:1 model)

### Backend
- **Node.js + TypeScript** or **Python** (for custodial wallet backend, Baray integration)
- **PostgreSQL** (primary database - user accounts, transactions, reserves)
- **Redis** (caching, session management)
- **Bull** (job queues for async tasks)
- **Express.js** or **FastAPI** (API framework)

### Infrastructure
- **AWS** or **GCP** (cloud hosting)
- **Vercel** or **Netlify** (frontend hosting)
- **SubQuery** or **The Graph** (blockchain indexing)
- **IPFS** (decentralized storage for NFTs)
- **Sentry** (error tracking and monitoring)
- **Plausible** or **Umami** (privacy-friendly analytics)

### Security
- **AWS KMS** or **GCP KMS** (key management for custodial wallets)
- **HSM** (Hardware Security Module for high-value keys)
- **Multi-sig** wallets (for treasury and critical operations)
- **CertiK** or **OpenZeppelin** (smart contract audits)
- **Bug bounties** (incentivize security researchers)

### DevOps
- **GitHub Actions** (CI/CD)
- **Docker** (containerization)
- **Kubernetes** (optional, if scale requires)
- **Monitoring:** Datadog, Grafana, or similar
- **Logging:** CloudWatch, Papertrail, or similar

---

## X. CRITICAL SUCCESS FACTORS

### Technical Excellence
1. **Security First**
   - Multiple audits for all smart contracts
   - Bug bounties
   - Regular penetration testing
   - Secure key management
   - Incident response plan

2. **UX Perfection**
   - Make crypto invisible (custodial wallets)
   - Khmer language support
   - Mobile-first design
   - One-click everything
   - Clear error messages

3. **Reliability**
   - 99.9%+ uptime
   - Fast transactions (1-2 seconds)
   - Real-time balance updates
   - Baray API never fails

### Ecosystem Health
4. **KHRt Peg Stability**
   - Always maintain 100%+ reserves
   - Real-time proof of reserves
   - Never fractional reserve
   - Monthly third-party audits
   - Transparent communication

5. **Merchant Adoption**
   - Start small, prove it works
   - Lower fees than alternatives
   - Same-day settlement
   - Excellent support
   - Marketing to customers

6. **Community Engagement**
   - Weekly predictions keep users active
   - Loyalty rewards
   - Social features
   - Responsive support
   - Listen to feedback

### Regulatory Compliance
7. **Work with Authorities**
   - Proactive communication with National Bank of Cambodia
   - Full KYC/AML compliance
   - Proper licenses
   - Transparent operations
   - Position as partner, not threat

### Partnerships
8. **Maintain Banking Relationships**
   - Keep ABA, ACLEDA, WING happy
   - Position as bringing customers to banks
   - Revenue sharing if needed
   - Redundancy (use all 3 banks)

9. **Sports Partnerships**
   - Deliver value to CPL and clubs
   - Share revenue appropriately
   - Help grow their fan engagement
   - Support Kun Khmer growth

---

## XI. RISKS & MITIGATION

### High Risk
**1. Banks Cut Off Access**
- **Mitigation:** Diversify across 3 banks, strong relationships, position as partner

**2. KHRt Peg Breaks**
- **Mitigation:** 100%+ reserves always, slow rollout, transparency, insurance buffer

**3. Regulatory Crackdown**
- **Mitigation:** Proactive compliance, work with NBC, adapt quickly, legal counsel

**4. Smart Contract Exploit**
- **Mitigation:** Multiple audits, bug bounties, gradual rollout, insurance fund, pause mechanisms

### Medium Risk
**5. Kun Khmer Launch Disappoints**
- **Mitigation:** Heavy marketing, cultural messaging, lower entry barriers, backup plan

**6. Slow Merchant Adoption**
- **Mitigation:** Start with stadiums (captive audience), show ROI clearly, excellent support

**7. User Wallet Adoption Lag**
- **Mitigation:** Incentives (bonus STAR), seamless UX, integrate tightly with StadiumX

### Low Risk
**8. Competition**
- **Mitigation:** Move fast, bank partnerships are moat, local knowledge advantage

**9. Technical Issues**
- **Mitigation:** Thorough testing, staging environment, gradual rollouts, monitoring

---

## XII. CONCLUSION

Selendra has a unique opportunity to become the dominant blockchain for Southeast Asian sports and payments. The combination of:

✅ Real users (30K+) with existing wallets
✅ Real partnerships (CPL, clubs, national team, top banks)
✅ Real infrastructure (Baray payment rails)
✅ Real catalyst (Kun Khmer launching Nov 2025)
✅ Real differentiation (KHRt stablecoin with bank integration)
✅ Real distribution (100+ SME merchants on Riverbase ready for crypto payments)

...positions Selendra for success where other L1s have failed.

**The Riverbase Advantage:**

Most blockchains struggle with the "cold start problem" - no merchants, no users, no utility. Selendra has the opposite:
- 100+ merchants already using Baray via Riverbase
- Merchants already trust the payment infrastructure
- Simple plugin/toggle to add crypto payments
- Reach 50+ crypto-accepting merchants in weeks, not years
- Instant network effects: users can spend tokens, merchants can earn them
- Proven e-commerce platform with Telegram integration

This is a **massive head start** that cannot be easily replicated.

**The key is execution:**
- Build custodial wallets that normies love
- Launch KHRt carefully and maintain the peg
- Scale merchants methodically
- Keep sports fans engaged weekly
- Maintain bank relationships
- Stay compliant with regulators

**If executed well, by end of 2026:**
- Selendra processes significant % of Cambodia's digital payments
- KHRt is the default digital currency for sports and commerce
- The model expands to Thailand, Vietnam, and Laos
- Selendra is THE regional payment blockchain

**The path is clear. The pieces are in place. Now it's time to build.**

---

## APPENDIX: CONTACT & RESOURCES

**Team Size:** 2-3 developers + Claude Code
**Development Approach:** Vibecoding (iterative, fast, adaptable)
**Timeline:** 12-18 months to full ecosystem
**Launch Target:** Kun Khmer (November 2025) as major catalyst

**Key Repositories:**
- Selendra blockchain
- Website (selendra.org)
- Explorer (explorer.selendra.org)
- StadiumX (existing)
- CPL Play (existing)
- Baray (existing)
- Riverbase.app (existing - 100+ SME stores)

**Documentation:**
- This roadmap (SELENDRA_ECOSYSTEM_ROADMAP.md)
- CLAUDE.md (development guide)
- README.md (project overview)

**Next Steps:**
1. Review and approve this roadmap
2. Prioritize Phase 1 apps
3. Start with DEX (enables STAR trading)
4. Begin KHRt development in parallel
5. Prepare for Kun Khmer launch (Nov 2025)

---

**Document Status:** DRAFT v1.0
**Last Updated:** January 2025
**Next Review:** Quarterly or as needed

