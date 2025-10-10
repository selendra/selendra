# Selendra Ecosystem Apps - Optimized for 2-3 Person Team

**Target Timeline:** 48 weeks (12 months) - ALIGNED with V4 Development
**Team Size:** 2-3 core developers + Claude Code
**Focus:** Payment adoption via Riverbase + Sports ecosystem

---

## Priority Legend
- **[P0]** - Critical, must have for launch
- **[P1]** - High value, core features
- **[P2]** - Nice to have, can defer
- **[CUT]** - Removed from scope

## Role Assignments
- **[@runtime]** - Backend/Runtime developer
- **[@frontend]** - Frontend/Mobile developer
- **[@fullstack]** - Can work on both

## Dependencies on V4 Development
- **Requires SDK** (V4 TASK-007) - Week 17-20
- **Requires DEX** (V4 TASK-008) - Week 21-24
- **Requires sUSD** (V4 TASK-009) - Week 25-27
- **Requires Bridge** (V4 TASK-011) - Week 29-34

---

## Phase 0: Prerequisites (Weeks 1-20)

⚠️ **CRITICAL:** Wait for V4 Phase 1-3 to complete before starting app development.

**During this time, work on:**
- Banking partnership negotiations (KHRt)
- Design system and UI mockups
- User research with StadiumX users
- Merchant requirements gathering (Riverbase)

---

## Phase 1: Core Infrastructure (Weeks 21-28)

### **TASK-001: Monorepo Setup** [P0] [@fullstack]

**Why Critical:** Foundation for all apps.

**Prerequisites:** V4 SDK published to npm (Week 20)

**Deliverables:**
- [ ] Create monorepo structure with Nx
- [ ] Configure TypeScript, ESLint, Prettier
- [ ] Setup Docker development environment
- [ ] Implement CI/CD pipeline with GitHub Actions
- [ ] Create shared component library foundation
- [ ] Add `@selendra/sdk` as dependency
- [ ] Configure hot reloading for all apps

**Definition of Done:**
- [ ] Monorepo compiles successfully
- [ ] Can run all apps in development mode
- [ ] CI/CD runs tests and linting on every PR
- [ ] Docker compose brings up full stack
- [ ] Shared components work across apps
- [ ] SDK integration verified

**Monorepo Structure:**
```
selendra-apps/
├── apps/
│   ├── wallet/           # Enhanced wallet
│   ├── merchant-pos/     # POS application
│   ├── dex-ui/           # DEX trading interface
│   └── admin/            # Admin dashboard
├── packages/
│   ├── ui/               # Shared UI components
│   ├── utils/            # Shared utilities
│   └── types/            # Shared TypeScript types
├── docker-compose.yml
├── nx.json
└── package.json
```

**Estimated Effort:** 1 week
**Actual Effort:** ___

---

### **TASK-002: Enhanced Wallet for Multi-Token Support** [P0] [@frontend]

**Why Critical:** Core user interface for all token operations.

**Prerequisites:**
- TASK-001 (monorepo setup)
- V4 DEX operational (Week 24)

**Deliverables:**
- [ ] Extend StadiumX wallet UI to support SEL, STAR, sUSD tokens
- [ ] Implement token balance display with real-time updates
- [ ] Create transaction history with filtering
- [ ] Add send/receive interface for all tokens
- [ ] Build transaction status tracking
- [ ] Integrate with Selendra SDK for blockchain operations
- [ ] Add token icons and branding
- [ ] Implement QR code generation for receiving

**Definition of Done:**
- [ ] Can view balances for SEL, STAR, sUSD
- [ ] Can send any token to any address
- [ ] Can receive tokens via QR code
- [ ] Transaction history shows last 100 transactions
- [ ] Real-time balance updates via WebSocket
- [ ] Mobile responsive design
- [ ] Works on StadiumX user accounts

**User Journey:**
```
1. User logs into StadiumX wallet
2. Dashboard shows balances: 100 SEL, 50 STAR, 25 sUSD
3. Click "Send" → Select STAR → Enter amount & recipient
4. Confirm transaction → Shows pending → Updates to confirmed
5. Balance updates in real-time
```

**API Integration:**
```typescript
import { SelendraApi } from '@selendra/sdk';

// Get balances
const balances = await api.balances.getMulti(address, ['SEL', 'STAR', 'sUSD']);

// Send token
const tx = await api.transactions.transfer({
    from: account,
    to: recipient,
    token: 'STAR',
    amount: '50'
});

// Subscribe to updates
api.balances.subscribe(address, (balances) => {
    updateUI(balances);
});
```

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

### **TASK-003: Notification System** [P1] [@backend]

**Why Important:** Real-time updates for better UX.

**Prerequisites:** TASK-001

**Deliverables:**
- [ ] Build WebSocket service for real-time updates
- [ ] Implement push notification service (Firebase)
- [ ] Create notification preferences UI
- [ ] Add email notification service (SendGrid)
- [ ] Build price feed distribution system
- [ ] Create notification history/inbox

**Definition of Done:**
- [ ] Users receive real-time transaction notifications
- [ ] Push notifications work on mobile browsers
- [ ] Email notifications sent for large transactions
- [ ] Users can configure notification preferences
- [ ] Price updates pushed every 30 seconds
- [ ] Notification history stored for 30 days

**Notification Types:**
```typescript
type Notification = {
    type: 'transaction' | 'price_alert' | 'governance' | 'event';
    title: string;
    message: string;
    data: Record<string, any>;
    timestamp: number;
}

// Examples:
{
    type: 'transaction',
    title: 'Transaction Confirmed',
    message: 'Sent 50 STAR to 0x1234...',
    data: { tx_hash: '0xabc...', amount: '50', token: 'STAR' }
}

{
    type: 'price_alert',
    title: 'Price Alert',
    message: 'SEL reached $0.50',
    data: { token: 'SEL', price: 0.50 }
}
```

**Estimated Effort:** 2 weeks
**Actual Effort:** ___

---

### **TASK-004: DEX Trading Interface** [P0] [@frontend]

**Why Critical:** Needed for token swaps and liquidity.

**Prerequisites:**
- TASK-002 (wallet)
- V4 DEX pallet operational (Week 24)

**Deliverables:**
- [ ] Create trading interface with token selection
- [ ] Implement real-time price quotes from DEX pallet
- [ ] Add slippage protection configuration
- [ ] Build swap execution with status tracking
- [ ] Create liquidity pool interface (add/remove liquidity)
- [ ] Show trading history and portfolio
- [ ] Add price charts (TradingView integration)

**Definition of Done:**
- [ ] Can swap any token pair with price preview
- [ ] Slippage protection works (max 1% default)
- [ ] Can add liquidity to pools
- [ ] Can remove liquidity and claim LP tokens
- [ ] Trading history shows last 50 trades
- [ ] Charts show 24h price movement
- [ ] Mobile responsive

**Trading UI Flow:**
```
1. Select "Swap" in wallet
2. Choose token pair: STAR → sUSD
3. Enter amount: 100 STAR
4. See quote: ~98 sUSD (2% slippage)
5. Adjust slippage tolerance: 1% → ~99 sUSD
6. Click "Swap" → Confirm → Transaction pending
7. Success! Balance updated
```

**DEX Integration:**
```typescript
import { SelendraApi } from '@selendra/sdk';

// Get price quote
const quote = await api.dex.getQuote({
    tokenIn: 'STAR',
    tokenOut: 'sUSD',
    amountIn: '100'
});
// Returns: { amountOut: '99', priceImpact: 0.01, route: [...] }

// Execute swap
const tx = await api.dex.swap({
    tokenIn: 'STAR',
    tokenOut: 'sUSD',
    amountIn: '100',
    minAmountOut: '99', // 1% slippage
});
```

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

## Phase 2: Payment Infrastructure (Weeks 29-40)

### **TASK-005: Riverbase Merchant Plugin** [P0] [@fullstack]

**Why Critical:** YOUR SECRET WEAPON. 100+ merchants ready to accept crypto.

**Prerequisites:**
- TASK-002 (wallet working)
- sUSD operational (Week 27)

**Deliverables:**
- [ ] Build Riverbase payment plugin architecture
- [ ] Implement QR code generation for payments (SEL, STAR, sUSD)
- [ ] Create payment request/response protocol
- [ ] Add automatic crypto-to-fiat conversion (via DEX)
- [ ] Build merchant dashboard for tracking payments
- [ ] Create settlement system (daily/weekly payouts)
- [ ] Implement payment confirmation callbacks
- [ ] Add refund functionality

**Definition of Done:**
- [ ] Merchant can generate payment QR code
- [ ] Customer scans QR and pays in crypto
- [ ] Payment confirmed in < 5 seconds
- [ ] Auto-convert crypto to KHR/USD for merchant
- [ ] Merchant dashboard shows all payments
- [ ] Settlement to bank account works (manual initially)
- [ ] 3 pilot merchants using in production

**Payment Flow:**
```
1. Customer buys $10 product at merchant
2. Merchant opens Riverbase POS
3. POS generates QR code: selendra://pay?amount=10&merchant=0x...
4. Customer scans with wallet app
5. Wallet shows: Pay 20 sUSD to MerchantName
6. Customer confirms
7. Transaction broadcast to blockchain
8. Merchant sees "Payment Received" in 3 seconds
9. Receipt generated
10. End of day: Merchant gets $290 (29 transactions) to bank
```

**Riverbase Integration:**
```typescript
// Generate payment request
const paymentRequest = await riverbase.createPayment({
    merchant_id: 'merchant123',
    amount_usd: 10.00,
    description: 'Coffee + Pastry',
    accepted_tokens: ['SEL', 'STAR', 'sUSD']
});

// Returns QR code data:
{
    qr_code: 'selendra://pay?req=...',
    payment_id: 'pay_abc123',
    expires_at: '2025-10-10T10:15:00Z'
}

// Listen for payment
riverbase.onPaymentReceived((payment) => {
    // Payment confirmed!
    showSuccessScreen(payment);
});
```

**Estimated Effort:** 4 weeks
**Actual Effort:** ___

**Critical Success Factor:**
- Get 3-5 pilot merchants using this in Week 33
- Gather feedback and iterate quickly
- Goal: 10 active merchants by Week 40

---

### **TASK-006: Merchant POS Application** [P0] [@frontend]

**Why Critical:** Complements Riverbase, enables in-person crypto payments.

**Prerequisites:** TASK-005 (Riverbase plugin)

**Deliverables:**
- [ ] Build mobile POS app (React Native or PWA)
- [ ] Implement QR code scanning for customer payments
- [ ] Add payment confirmation with visual/audio feedback
- [ ] Create receipt generation (email + print)
- [ ] Build transaction history for merchants
- [ ] Add offline payment capability (sync when online)
- [ ] Implement merchant authentication
- [ ] Create simple inventory tracking (optional)

**Definition of Done:**
- [ ] Merchant can log in to POS app
- [ ] Can generate payment QR codes
- [ ] Can scan customer wallet QR codes
- [ ] Payment confirmation in < 5 seconds
- [ ] Receipts sent via email
- [ ] Works offline, syncs when back online
- [ ] 5 merchants using daily

**POS UI Screens:**
```
Screen 1: Login
Screen 2: Dashboard (today's sales: $45, 12 transactions)
Screen 3: New Sale
    - Enter amount: $10
    - Generate QR code
    - Wait for payment...
    - ✅ Payment received!
    - Generate receipt
Screen 4: Transaction History
Screen 5: Settings
```

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

### **TASK-007: KHRt Wallet Interface** [P1] [@frontend]

**Why Important:** Enables Cambodian Riel stablecoin operations.

**Prerequisites:**
- TASK-002 (wallet working)
- V4 KHRt pallet built (Week 28)
- ⚠️ **Banking partnerships confirmed** (external dependency)

**⚠️ BLOCKER:** Can build UI now, but CANNOT LAUNCH until banking APIs are live.

**Deliverables:**
- [ ] Create KHRt minting interface with bank linking
- [ ] Implement KYC verification flow (ID upload)
- [ ] Build KHRt burning interface with bank withdrawal
- [ ] Add transaction limits display by KYC level
- [ ] Create KHRt balance tracking
- [ ] Show reserve ratio (transparency)
- [ ] Add history of mint/burn operations

**Definition of Done:**
- [ ] UI built and functional with mock banking API
- [ ] Can simulate minting KHRt after "bank deposit"
- [ ] Can simulate burning KHRt and "bank withdrawal"
- [ ] KYC flow designed (pending provider integration)
- [ ] Transaction limits enforced in UI
- [ ] **Ready to go live when banking APIs confirmed**

**KHRt Flow (Future):**
```
1. User clicks "Add KHRt"
2. Choose bank: ABA, ACLEDA, or WING
3. Enter amount: 100,000 KHR ($25)
4. Deposit to bank account shown
5. After deposit confirmed, KHRt minted
6. Balance updates: +100,000 KHRt

To withdraw:
1. Click "Cash Out KHRt"
2. Enter amount: 50,000 KHRt
3. Enter bank account number
4. Confirm KYC level (daily limit check)
5. KHRt burned, bank transfer initiated
6. Receive in bank account in 1-2 hours
```

**Estimated Effort:** 3 weeks (UI only)
**Banking Integration:** 3-12 months (external)
**Actual Effort:** ___

---

### **TASK-008: Banking API Integration** [P1] [@backend]

**Why Important:** Required for KHRt to work.

**Prerequisites:**
- V4 KHRt pallet (Week 28)
- Banking partnerships signed

**⚠️ BLOCKER:** Completely dependent on external banking partnerships.

**Parallel Workstream (Start NOW):**
- [ ] Initial discussions with ABA Bank
- [ ] Initial discussions with ACLEDA Bank
- [ ] Initial discussions with WING Bank
- [ ] API documentation requests
- [ ] Sandbox access requests
- [ ] Compliance requirements gathering
- [ ] KYC provider selection
- [ ] Reserve account setup

**Technical Deliverables (when APIs available):**
- [ ] Build ABA Bank API integration
- [ ] Add ACLEDA Bank API integration
- [ ] Implement WING Bank API integration
- [ ] Create unified banking interface
- [ ] Add real-time transaction status tracking
- [ ] Implement webhook handling for deposits
- [ ] Build reconciliation system

**Definition of Done:**
- [ ] Can verify bank deposits via API
- [ ] Can initiate bank withdrawals via API
- [ ] Real-time status updates work
- [ ] Reconciliation matches 100% of transactions
- [ ] Error handling for failed transactions
- [ ] **All banks integrated and tested**

**Banking API Example:**
```typescript
// Unified banking interface
interface BankingProvider {
    verifyDeposit(txId: string): Promise<DepositStatus>;
    initiateWithdrawal(account: string, amount: number): Promise<WithdrawalStatus>;
    getTransactionStatus(txId: string): Promise<TransactionStatus>;
}

// ABA Bank implementation
class ABABankProvider implements BankingProvider {
    async verifyDeposit(txId: string) {
        const response = await fetch('https://api.ababank.com/verify', {
            method: 'POST',
            headers: { 'Authorization': `Bearer ${this.apiKey}` },
            body: JSON.stringify({ transaction_id: txId })
        });
        return response.json();
    }
}
```

**Estimated Effort:** 6 weeks (once APIs available)
**Partnership Timeline:** 3-12 months
**Actual Effort:** ___

---

## Phase 3: Sports Ecosystem (Weeks 41-48)

### **TASK-009: Enhanced CPL Play for Multi-Token** [P1] [@frontend]

**Why Important:** Integrate tokens into existing sports platform.

**Prerequisites:**
- TASK-002 (wallet)
- TASK-004 (DEX)
- V4 infrastructure stable

**Deliverables:**
- [ ] Integrate STAR, sUSD, KHRt for predictions
- [ ] Add token selection interface when placing bets
- [ ] Implement automatic token conversion if needed
- [ ] Create enhanced reward distribution
- [ ] Build leaderboards with token prizes
- [ ] Add achievements with token rewards

**Definition of Done:**
- [ ] Can bet using STAR, sUSD, or KHRt
- [ ] Automatic conversion works (e.g., bet sUSD, pool is STAR)
- [ ] Rewards distributed in chosen token
- [ ] Leaderboard shows top predictors
- [ ] 100+ users using multi-token betting

**CPL Play Integration:**
```typescript
// Place prediction
const prediction = await cplPlay.placeBet({
    match_id: 'KK-2025-10',
    prediction: 'fighter_a_wins',
    amount: '50',
    token: 'STAR' // or sUSD, KHRt
});

// If pool is in different token, auto-convert
if (pool.token !== 'STAR') {
    await api.dex.swap({
        tokenIn: 'STAR',
        tokenOut: pool.token,
        amountIn: '50'
    });
}
```

**Estimated Effort:** 2 weeks
**Actual Effort:** ___

---

### **TASK-010: NFT Ticketing System** [P1] [@fullstack]

**Why Important:** Real utility NFTs for Kun Khmer events.

**Prerequisites:**
- TASK-002 (wallet)
- V4 infrastructure stable

**Deliverables:**
- [ ] Create NFT ticket minting smart contract
- [ ] Implement ticket verification system with QR codes
- [ ] Add ticket transfer functionality
- [ ] Build resale marketplace (optional)
- [ ] Create event management interface
- [ ] Add ticket holder benefits tracking

**Definition of Done:**
- [ ] Event organizers can mint NFT tickets
- [ ] Users can buy tickets with SEL/STAR/sUSD
- [ ] QR code verification at event entrance
- [ ] Can transfer tickets to friends
- [ ] Resale works with royalties to organizer
- [ ] 1 Kun Khmer event using NFT tickets

**Ticket NFT Structure:**
```solidity
contract EventTicket {
    struct Ticket {
        uint256 eventId;
        uint256 seatNumber;
        address owner;
        bool used;
        uint256 price;
    }

    function mintTicket(uint256 eventId, uint256 seatNumber) external;
    function transferTicket(uint256 ticketId, address to) external;
    function verifyTicket(uint256 ticketId) external returns (bool valid);
    function useTicket(uint256 ticketId) external; // Mark as used at entrance
}
```

**User Journey:**
```
1. User sees "Kun Khmer Championship - Nov 15"
2. Click "Buy Ticket" → Select seat → Pay 50 STAR
3. NFT ticket minted to user's wallet
4. Day of event: Open wallet → Show QR code
5. Staff scans QR → Ticket verified → Entry granted
6. Ticket marked as "used" on blockchain
```

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

### **TASK-011: Basic Fan Token Platform** [P2] [@fullstack]

**Why Nice to Have:** Deepens fan engagement, but can defer if needed.

**Prerequisites:**
- TASK-004 (DEX for trading)
- V4 infrastructure stable

**Deliverables:**
- [ ] Create fan token smart contract template
- [ ] Implement token distribution mechanism
- [ ] Add voting functionality for team decisions
- [ ] Build fan engagement features
- [ ] Create fan token trading interface
- [ ] Launch with 1-2 pilot teams

**Definition of Done:**
- [ ] 2 sports teams have fan tokens
- [ ] Fans can buy tokens on DEX
- [ ] Token holders can vote on team decisions
- [ ] Engagement rewards distributed
- [ ] Trading volume > $1K/week

**Fan Token Use Cases:**
```
1. Voting: "Should team sign player X?"
   → Token holders vote
   → Decision based on weighted votes

2. Rewards: "Win prediction contest"
   → Prize: 100 FAN tokens

3. Access: "VIP meet & greet"
   → Must hold 1000 FAN tokens

4. Trading: Buy/sell on DEX
```

**Estimated Effort:** 3 weeks
**Actual Effort:** ___

---

## ❌ TASKS REMOVED FROM SCOPE

### **[CUT] TASK: Heavy Analytics Infrastructure**
**Reason:** Too complex for 2-3 person team. Use hosted services instead.
**Alternative:**
- Use Vercel Analytics for web
- Use Mixpanel for user behavior
- Use Datadog for backend monitoring
**Time Saved:** 3 weeks

### **[CUT] TASK: Mobile Wallet App (React Native)**
**Reason:** Web-first approach, PWA sufficient initially.
**Alternative:**
- Build responsive web app
- Make it a PWA (Progressive Web App)
- Add to homescreen functionality
- Mobile app later if needed
**Time Saved:** 4 weeks

### **[CUT] TASK: Bridge Interface in Ecosystem Apps**
**Reason:** Bridge UI should be part of main wallet, not separate app.
**Alternative:**
- Add bridge tab to main wallet (TASK-002)
- Simple interface: "Bridge USDC from Ethereum"
**Time Saved:** 2 weeks

### **[CUT] TASK: Separate Developer Documentation App**
**Reason:** Documentation is part of V4 development, not ecosystem apps.
**Alternative:**
- V4 TASK-014 covers all documentation
**Time Saved:** 2 weeks

**Total Time Saved:** 11 weeks

---

## Timeline Summary (Aligned with V4)

| Phase | Weeks | Key Deliverables | Depends On |
|-------|-------|------------------|------------|
| Phase 0: Wait | 1-20 | Design, planning, partnerships | V4 Phase 1-3 |
| Phase 1: Core | 21-28 | Wallet + DEX UI | V4 SDK + DEX |
| Phase 2: Payments | 29-40 | Riverbase + POS + KHRt UI | V4 sUSD |
| Phase 3: Sports | 41-48 | CPL + Tickets + Fans | All above |

**Total: 48 weeks (12 months) - Same as V4**

---

## Success Metrics

### User Adoption:
- ✅ 15K+ wallet users by end of Phase 1
- ✅ 2K+ daily active users
- ✅ 20K+ monthly transactions

### Payment Metrics:
- ✅ 10 Riverbase merchants accepting crypto
- ✅ $10K+ monthly payment volume
- ✅ 100+ daily payment transactions

### Sports Metrics:
- ✅ 1 Kun Khmer event with NFT tickets
- ✅ 2 teams with fan tokens
- ✅ 5K+ users engaged in sports features

### DeFi Metrics:
- ✅ $50K+ monthly DEX volume
- ✅ 5+ active trading pairs
- ✅ 100+ liquidity providers

### KHRt Metrics (when launched):
- ✅ 100 users in closed beta
- ✅ $10K minted in first month
- ✅ 100%+ reserve ratio maintained

---

## Critical Success Factors

### 1. Riverbase = Your Moat
- 100+ merchants ready to accept crypto
- **This is your unfair advantage**
- Prioritize TASK-005 (Riverbase Plugin)
- Get 10 merchants live ASAP

### 2. Start Banking Negotiations NOW
- KHRt depends entirely on banking partnerships
- **Start discussions immediately**
- Build UI in parallel, launch when ready
- Have backup: sUSD works even if KHRt delayed

### 3. Focus on Payment UX
- Payments must be FAST (< 5 second confirmation)
- Payments must be SIMPLE (scan QR → pay → done)
- Merchants must trust it (auto-convert to fiat)

### 4. Sports = Engagement Loop
- StadiumX users → CPL Play predictions → Need tokens
- Need tokens → Use DEX → Provide liquidity
- Kun Khmer events → NFT tickets → More engagement
- Engagement → Fan tokens → Deeper loyalty

---

## Team Allocation

**Person 1 - Frontend [@frontend]:**
- Phase 1: Wallet + DEX UI (6 weeks)
- Phase 2: POS App + KHRt UI (6 weeks)
- Phase 3: CPL + Tickets + Fans (6 weeks)

**Person 2 - Backend [@backend]:**
- Phase 0: Banking partnership work (20 weeks)
- Phase 1: Notification system (2 weeks)
- Phase 2: Riverbase plugin + Banking APIs (10 weeks)
- Phase 3: Support + scaling (6 weeks)

**Person 3 - Fullstack [@fullstack] (if available):**
- Phase 1: Monorepo + Infrastructure (2 weeks)
- Phase 2: Testing + Integration (10 weeks)
- Phase 3: NFT contracts + Fan tokens (6 weeks)

**With Claude Code:**
- Generate boilerplate for UI components
- Write tests for payment flows
- Create API client code
- Generate mock data for testing
- Can reduce effort by 30-40%

---

## Risk Management

### High Risks:
| Risk | Impact | Mitigation |
|------|--------|-----------|
| Banking partnerships delayed | CRITICAL | Build pallet now, launch when ready. Use sUSD meanwhile. |
| Riverbase integration complex | HIGH | Work closely with Riverbase team. Start small (3 merchants). |
| Payment UX slow | HIGH | Optimize for speed. Use WebSocket for real-time updates. |
| Low merchant adoption | MEDIUM | Incentive program. Hand-hold first 10 merchants. |

### Medium Risks:
| Risk | Impact | Mitigation |
|------|--------|-----------|
| CPL Play integration breaks | MEDIUM | Thorough testing. Have rollback plan. |
| NFT ticket complexity | MEDIUM | Start simple. Add features iteratively. |
| DEX liquidity low | MEDIUM | Bootstrap with team funds. Incentivize LPs. |

---

## Launch Strategy

### Phase 1 Launch (Week 28): "Enhanced Wallet + DEX"
- **Users:** 15K (from StadiumX)
- **Features:** Multi-token wallet, DEX trading
- **Goal:** Establish baseline usage

### Phase 2 Launch (Week 40): "Crypto Payments Live"
- **Users:** 20K+
- **Merchants:** 10 Riverbase pilots
- **Features:** Payment plugin, POS app
- **Goal:** Real-world payment adoption

### Phase 3 Launch (Week 48): "Sports Ecosystem"
- **Users:** 30K+
- **Merchants:** 25+
- **Features:** NFT tickets, Fan tokens, Enhanced CPL
- **Goal:** Integrated ecosystem with network effects

### Post-Launch (Month 13+): "KHRt + Expansion"
- **When:** Banking partnerships confirmed
- **Features:** KHRt minting/burning, More merchants
- **Goal:** Full payment network with local currency

---

## Next Steps

1. **Week 0:** Review with team, align on priorities
2. **Week 0:** Start banking partnership outreach
3. **Week 0:** Begin design work (UI mockups)
4. **Week 1-20:** Wait for V4 infrastructure, do planning
5. **Week 21:** Start TASK-001 (Monorepo setup)
6. **Weekly:** Team sync, adjust plan as needed

**Key Principle:** Build in small iterations, get real users ASAP, iterate based on feedback.

**Let's ship! 🚀**
