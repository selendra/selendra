# SELENDRA ECOSYSTEM: DEVELOPER TASKS

**Document Version:** 1.0
**Last Updated:** January 2025
**Team:** 2-3 developers + Claude Code
**Approach:** Vibecoding (iterative, fast iteration)

---

## HOW TO USE THIS DOCUMENT

1. **Tasks are organized by priority phases** (Phase 1 = highest priority)
2. **Each task is a complete app** with specs and instructions
3. **Work on tasks sequentially** within each phase when possible
4. **Dependencies are clearly marked** - complete those first
5. **Timeline estimates** are for 1 developer + Claude Code
6. **Acceptance criteria** define when task is complete

---

## PHASE 1: FOUNDATION (Q1 2025 - Months 1-3)

**Goal:** Build core DeFi infrastructure and enable STAR trading

---

### TASK 1.1: Decentralized Exchange (DEX)

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL
**Timeline:** 6-7 weeks
**Dependencies:** None (can start immediately)
**Team:** 1 developer (smart contracts) + 1 developer (frontend) + Claude Code

#### Purpose
Build AMM-based DEX to enable STAR trading and provide liquidity for all tokens on Selendra.

#### Technical Specs

**Smart Contracts:**
- Fork Uniswap V2 (simpler) or V3 (more features)
- Deploy factory contract
- Deploy router contract
- Deploy pair contracts for each trading pair
- Implement LP token minting/burning
- Fee structure: 0.3% standard, 0.05% for stablecoin pairs

**Priority Pools (Launch Order):**
1. STAR/KHRt
2. KHRt/sUSD
3. SEL/sUSD
4. sUSD/USDT
5. SEL/STAR

**Frontend:**
- React 18 + TypeScript + Vite
- wagmi v2 + viem for Web3 interactions
- TanStack Query for data fetching
- Tailwind CSS for styling

#### Development Instructions

**Step 1: Smart Contracts (3-4 weeks)**

```solidity
// 1. Clone Uniswap V2 core
git clone https://github.com/Uniswap/v2-core
git clone https://github.com/Uniswap/v2-periphery

// 2. Modify for Selendra network
// - Update network config in hardhat.config.ts
// - Change router address constants
// - Update fee denominators if needed

// 3. Deploy contracts
npx hardhat run scripts/deploy.ts --network selendra

// 4. Verify contracts on explorer
npx hardhat verify --network selendra CONTRACT_ADDRESS

// 5. Create initial liquidity pools
// Use scripts/createPairs.ts
```

**Key Files:**
- `contracts/UniswapV2Factory.sol` - Factory for creating pairs
- `contracts/UniswapV2Pair.sol` - Pair implementation
- `contracts/UniswapV2Router02.sol` - Router for swaps
- `scripts/deploy.ts` - Deployment script
- `scripts/createPairs.ts` - Create initial pairs

**Step 2: Frontend (2 weeks)**

```bash
# 1. Create new React app
npm create vite@latest dex -- --template react-ts
cd dex
npm install wagmi viem @tanstack/react-query

# 2. Set up Web3 connection
# - Create wagmiConfig with Selendra chain
# - Set up WagmiConfig provider
# - Connect wallet component

# 3. Implement swap interface
# - Token select dropdowns
# - Amount inputs with max button
# - Price impact display
# - Slippage tolerance settings
# - Swap button with transaction handling

# 4. Implement liquidity interface
# - Add liquidity form (dual token inputs)
# - Remove liquidity form
# - LP token balance display
# - Pool share percentage

# 5. Implement pool analytics
# - Total TVL
# - 24h volume per pair
# - APY calculations
# - Historical charts (optional)
```

**Key Components:**
- `src/components/Swap.tsx` - Swap interface
- `src/components/AddLiquidity.tsx` - Add liquidity form
- `src/components/RemoveLiquidity.tsx` - Remove liquidity form
- `src/components/PoolList.tsx` - List of all pools
- `src/hooks/useSwap.ts` - Swap logic hook
- `src/hooks/useLiquidity.ts` - Liquidity logic hook

**Step 3: Testing (1 week)**

```bash
# Smart contract tests
npx hardhat test

# Frontend tests
npm run test

# Integration tests on testnet
# - Test all swap combinations
# - Test adding/removing liquidity
# - Test edge cases (zero amounts, max amounts)
```

#### Acceptance Criteria

- [ ] Factory and Router contracts deployed on Selendra mainnet
- [ ] At least 5 initial pairs created (STAR/KHRt, KHRt/sUSD, SEL/sUSD, sUSD/USDT, SEL/STAR)
- [ ] Users can swap between any paired tokens
- [ ] Users can add liquidity and receive LP tokens
- [ ] Users can remove liquidity and burn LP tokens
- [ ] Price impact is displayed before swaps
- [ ] Slippage protection works
- [ ] Contracts verified on block explorer
- [ ] Frontend deployed to production (Vercel/Netlify)
- [ ] $100K+ initial liquidity seeded across pools

#### Configuration

```typescript
// config/contracts.ts
export const DEX_CONTRACTS = {
  factory: '0x...', // After deployment
  router: '0x...',  // After deployment
}

export const INITIAL_PAIRS = [
  { token0: 'STAR', token1: 'KHRt' },
  { token0: 'KHRt', token1: 'sUSD' },
  { token0: 'SEL', token1: 'sUSD' },
  { token0: 'sUSD', token1: 'USDT' },
  { token0: 'SEL', token1: 'STAR' },
]
```

---

### TASK 1.2: Enhanced Custodial Wallet

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL
**Timeline:** 4 weeks (incremental enhancements)
**Dependencies:** None (enhance existing wallets)
**Team:** 1-2 developers + Claude Code

#### Purpose
Enhance existing custodial wallets on StadiumX to support multi-token, DEX integration, and Baray integration.

#### Technical Specs

**Platform:** React Native (iOS + Android) + Web
**Current State:** 30K users with email/password login holding STAR + player cards
**Backend:** Node.js + TypeScript + PostgreSQL + Redis

**Key Features to Add:**
1. Multi-token support (SEL, USDT, USDC, sUSD, KHRt, STAR, fan tokens)
2. In-app DEX swaps
3. Baray integration (KHQR on-ramp, bank off-ramp)
4. Staking interface
5. Portfolio dashboard

#### Development Instructions

**Step 1: Multi-Token Support (1 week)**

```typescript
// 1. Update database schema
// migrations/add_multi_token_support.sql
ALTER TABLE wallets ADD COLUMN tokens JSONB DEFAULT '{}';

// tokens structure:
{
  "SEL": { balance: "1000.00", decimals: 18 },
  "STAR": { balance: "5000.00", decimals: 18 },
  "KHRt": { balance: "100000.00", decimals: 18 }
}

// 2. Create token balance service
// services/tokenBalance.ts
export class TokenBalanceService {
  async getBalance(userId: string, tokenAddress: string): Promise<string>
  async updateBalance(userId: string, tokenAddress: string, amount: string): Promise<void>
  async getAllBalances(userId: string): Promise<TokenBalances>
}

// 3. Add token list configuration
// config/tokens.ts
export const SUPPORTED_TOKENS = [
  { symbol: 'SEL', address: '0x...', decimals: 18, icon: '/icons/sel.png' },
  { symbol: 'STAR', address: '0x...', decimals: 18, icon: '/icons/star.png' },
  { symbol: 'KHRt', address: '0x...', decimals: 18, icon: '/icons/khrt.png' },
  { symbol: 'sUSD', address: '0x...', decimals: 18, icon: '/icons/susd.png' },
  // ... more tokens
]

// 4. Update wallet UI to show all tokens
// components/WalletHome.tsx
```

**Step 2: In-App DEX Integration (1 week)**

```typescript
// 1. Create swap service
// services/swapService.ts
export class SwapService {
  async getQuote(fromToken: string, toToken: string, amount: string): Promise<Quote>
  async executeSwap(userId: string, fromToken: string, toToken: string, amount: string): Promise<Transaction>
}

// 2. Build swap UI
// components/Swap.tsx
// - Token select (from/to)
// - Amount input
// - Price display with refresh
// - Slippage settings
// - Swap button (gasless for custodial)

// 3. Handle transactions in background
// - Sign transaction with custodial key
// - Submit to blockchain
// - Update balances on confirmation
// - Show transaction status to user
```

**Step 3: Baray Integration (1 week)**

```typescript
// 1. Integrate Baray API
// services/barayService.ts
export class BarayService {
  async generateKHQR(userId: string, amount: string): Promise<KHQRCode>
  async checkPaymentStatus(paymentId: string): Promise<PaymentStatus>
  async initiateWithdrawal(userId: string, bankAccount: BankAccount, amount: string): Promise<Withdrawal>
}

// 2. Build on-ramp UI
// components/AddMoney.tsx
// - Enter amount in KHR
// - Generate KHQR code
// - Display QR for user to scan
// - Poll for payment confirmation
// - Mint KHRt to user wallet

// 3. Build off-ramp UI
// components/CashOut.tsx
// - Select saved bank account or add new
// - Enter amount in KHRt
// - Show equivalent in KHR
// - Confirm and process withdrawal
// - Burn KHRt
// - Send KHR to bank via Baray
```

**Step 4: Staking Interface (1 week)**

```typescript
// 1. Create staking service
// services/stakingService.ts
export class StakingService {
  async getValidators(): Promise<Validator[]>
  async stake(userId: string, validatorId: string, amount: string): Promise<Transaction>
  async unstake(userId: string, stakeId: string): Promise<Transaction>
  async claimRewards(userId: string, stakeId: string): Promise<Transaction>
  async getStakingPositions(userId: string): Promise<StakingPosition[]>
}

// 2. Build staking UI
// components/Staking.tsx
// - Validator list with APY
// - Stake form
// - Active stakes list
// - Rewards display
// - Unstake button with cooldown timer
```

#### Acceptance Criteria

- [ ] Users can see balances for SEL, USDT, USDC, sUSD, KHRt, STAR
- [ ] Users can swap between tokens in-app (gasless)
- [ ] Users can add money via KHQR (generates code, mints KHRt on payment)
- [ ] Users can cash out to bank (burns KHRt, sends KHR to bank)
- [ ] Users can stake SEL tokens to validators
- [ ] Users can claim staking rewards
- [ ] Portfolio dashboard shows total value in KHR and USD
- [ ] Transaction history shows all activities
- [ ] Biometric authentication works (Face ID/fingerprint)
- [ ] Khmer and English language support

#### Configuration

```typescript
// config/wallet.ts
export const WALLET_CONFIG = {
  gaslessTransactions: true, // Custodial wallets = we pay gas
  supportedTokens: SUPPORTED_TOKENS,
  barayApiUrl: 'https://api.baray.io',
  stakingMinAmount: '100', // 100 SEL minimum
  defaultSlippage: 0.5, // 0.5%
}
```

---

### TASK 1.3: Bridge Integration (LayerZero/Axelar)

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL
**Timeline:** 6-8 weeks
**Dependencies:** None
**Team:** 1 developer + bridge protocol support + Claude Code

#### Purpose
Integrate existing bridge solution (LayerZero or Axelar) to bring USDT/USDC from Ethereum to Selendra.

#### Technical Specs

**Recommended:** LayerZero (more flexible)
**Alternative:** Axelar

**Supported Chains (Priority):**
1. Ethereum (for USDT/USDC)
2. BSC
3. Polygon
4. Arbitrum/Optimism

**Assets to Bridge:**
- USDT (most critical)
- USDC (most critical)
- WETH
- WBTC

#### Development Instructions

**Step 1: Choose and Set Up Bridge Protocol (1 week)**

```bash
# Option A: LayerZero
# 1. Contact LayerZero team for Selendra integration
# 2. Deploy LayerZero endpoint on Selendra
# 3. Configure trusted remotes

# Option B: Axelar
# 1. Contact Axelar team for Selendra integration
# 2. Deploy Axelar gateway on Selendra
# 3. Configure chain connections
```

**Step 2: Deploy Bridge Contracts (2-3 weeks)**

```solidity
// Using LayerZero example:

// 1. Deploy OFT (Omnichain Fungible Token) contracts
// contracts/USDT_OFT.sol
contract USDT_OFT is OFT {
    constructor(address _lzEndpoint) OFT("Tether USD", "USDT", _lzEndpoint) {}
}

// 2. Deploy for each token
// - USDT_OFT
// - USDC_OFT
// - WETH_OFT
// - WBTC_OFT

// 3. Configure trusted remotes
// scripts/configureBridge.ts
await usdtOFT.setTrustedRemote(
  ethereumChainId,
  ethers.utils.solidityPack(['address', 'address'], [ethereumUSDT, selendraUSDT])
)

// 4. Set min/max amounts and fees
await usdtOFT.setMinDstGas(ethereumChainId, 0, 200000)
```

**Step 3: Build Bridge Frontend (2 weeks)**

```typescript
// 1. Create bridge interface
// components/Bridge.tsx

interface BridgeProps {
  fromChain: Chain
  toChain: Chain
  token: Token
  amount: string
}

// Features:
// - Source chain selector (Ethereum, BSC, Polygon, etc.)
// - Destination chain (Selendra)
// - Token selector (USDT, USDC, WETH, WBTC)
// - Amount input
// - Estimated time (15-30 minutes typical)
// - Estimated fees (gas + bridge fee)
// - Bridge button
// - Transaction tracking

// 2. Create bridge service
// services/bridgeService.ts
export class BridgeService {
  async estimateFees(from: Chain, to: Chain, token: Token, amount: string): Promise<BridgeFees>
  async bridge(from: Chain, to: Chain, token: Token, amount: string): Promise<Transaction>
  async trackBridge(txHash: string): Promise<BridgeStatus>
}

// 3. Add transaction tracking
// components/BridgeHistory.tsx
// - List of bridge transactions
// - Status (pending, confirmed, completed, failed)
// - Links to explorers on both chains
// - Estimated time remaining
```

**Step 4: Security & Testing (2 weeks)**

```typescript
// 1. Set rate limits
await bridgeContract.setRateLimit(token, maxPerDay)

// 2. Add emergency pause
await bridgeContract.pause() // In case of issue

// 3. Configure multi-sig for admin functions
// Use Gnosis Safe with 3/5 signers

// 4. Test thoroughly on testnet
// - Bridge small amounts
// - Bridge large amounts
// - Test all supported tokens
// - Test all chain pairs
// - Test edge cases (insufficient balance, network issues)
```

#### Acceptance Criteria

- [ ] LayerZero or Axelar integrated on Selendra
- [ ] Users can bridge USDT from Ethereum to Selendra
- [ ] Users can bridge USDC from Ethereum to Selendra
- [ ] Users can bridge WETH and WBTC (optional at launch)
- [ ] Bridge interface shows clear fees and time estimates
- [ ] Transaction tracking works (shows status on both chains)
- [ ] Rate limiting prevents abuse
- [ ] Emergency pause mechanism in place
- [ ] Multi-sig controls admin functions
- [ ] At least $500K successfully bridged in testing
- [ ] Security audit completed

#### Configuration

```typescript
// config/bridge.ts
export const BRIDGE_CONFIG = {
  protocol: 'layerzero', // or 'axelar'

  supportedChains: [
    { id: 1, name: 'Ethereum', rpc: 'https://eth.llamarpc.com' },
    { id: 56, name: 'BSC', rpc: 'https://bsc.llamarpc.com' },
    { id: 137, name: 'Polygon', rpc: 'https://polygon.llamarpc.com' },
  ],

  supportedTokens: {
    ethereum: [
      { symbol: 'USDT', address: '0xdac17f958d2ee523a2206206994597c13d831ec7' },
      { symbol: 'USDC', address: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48' },
    ],
    // ... more chains
  },

  rateLimits: {
    USDT: { maxPerDay: '1000000', maxPerTx: '100000' },
    USDC: { maxPerDay: '1000000', maxPerTx: '100000' },
  },
}
```

---

### TASK 1.4: Staking Portal

**Priority:** ⭐⭐⭐⭐ HIGH
**Timeline:** 4-6 weeks
**Dependencies:** Staking should exist at protocol level (Substrate pallet)
**Team:** 1 developer + Claude Code

#### Purpose
Build web interface for users to stake SEL tokens to validators and earn rewards.

#### Technical Specs

**Platform:** Web app
**Integration:** Substrate staking pallet (assumes this exists at protocol level)
**Frontend:** React + TypeScript + wagmi/viem or Polkadot.js

#### Development Instructions

**Step 1: Set Up Substrate Connection (1 week)**

```typescript
// 1. Install Polkadot.js API
npm install @polkadot/api @polkadot/extension-dapp

// 2. Create API service
// services/substrateApi.ts
import { ApiPromise, WsProvider } from '@polkadot/api'

export class SubstrateApi {
  private api: ApiPromise

  async connect() {
    const provider = new WsProvider('wss://rpc.selendra.org')
    this.api = await ApiPromise.create({ provider })
  }

  async getValidators(): Promise<Validator[]> {
    const validators = await this.api.query.staking.validators.entries()
    return validators.map(([key, exposure]) => ({
      address: key.args[0].toString(),
      commission: exposure.commission.toNumber(),
      // ... more validator data
    }))
  }

  async stake(validatorAddress: string, amount: string): Promise<string> {
    // Create and submit stake transaction
  }
}
```

**Step 2: Build Validator List (1 week)**

```typescript
// components/ValidatorList.tsx
interface Validator {
  address: string
  name?: string
  commission: number // 0-100%
  totalStake: string
  apy: number
  uptime: number // 0-100%
  isActive: boolean
}

// Features:
// - Sort by APY, commission, total stake
// - Filter by active/inactive
// - Search by name/address
// - Validator details modal (click for more info)
// - Commission and APY clearly displayed
```

**Step 3: Build Staking Interface (2 weeks)**

```typescript
// components/StakeForm.tsx
// - Select validator from list
// - Enter amount to stake
// - Show APY calculation
// - Show minimum stake requirement
// - Stake button

// components/StakingPositions.tsx
// - List of user's active stakes
// - Validator info for each stake
// - Amount staked
// - Rewards earned (real-time)
// - Claim rewards button
// - Unstake button with cooldown warning

// components/UnstakeForm.tsx
// - Select stake to unstake
// - Show cooldown period (e.g., 28 days)
// - Confirm unstake button
// - Countdown timer during cooldown
```

**Step 4: Add Analytics Dashboard (1 week)**

```typescript
// components/StakingDashboard.tsx
// - Total SEL staked on network
// - Network staking ratio (staked / total supply)
// - User's total staked
// - User's total rewards earned
// - APY calculator (enter amount, see projected rewards)
// - Historical rewards chart
```

#### Acceptance Criteria

- [ ] Users can view list of all validators
- [ ] Users can see validator details (commission, APY, uptime, total stake)
- [ ] Users can stake SEL to a validator
- [ ] Users can see their staking positions
- [ ] Users can see real-time rewards accumulation
- [ ] Users can claim rewards to wallet
- [ ] Users can unstake (with cooldown period)
- [ ] APY calculator works
- [ ] Dashboard shows network staking statistics
- [ ] Mobile responsive
- [ ] Khmer + English language support

#### Configuration

```typescript
// config/staking.ts
export const STAKING_CONFIG = {
  rpcEndpoint: 'wss://rpc.selendra.org',
  minimumStake: '100', // 100 SEL minimum
  cooldownPeriod: 28, // 28 days
  maxValidatorsPerUser: 16, // Can stake to max 16 validators
  rewardsDistributionFrequency: 'per_block', // or 'daily'
}
```

---

### TASK 1.5: sUSD Stablecoin (Simple 1:1 Model)

**Priority:** ⭐⭐⭐⭐ HIGH
**Timeline:** 4-6 weeks
**Dependencies:** Bridge (need USDT/USDC on Selendra first)
**Team:** 1 developer + auditor + Claude Code

#### Purpose
Create simple 1:1 wrapped stablecoin to bridge USDT/USDC to Selendra for DEX trading and KHRt collateral.

#### Technical Specs

**Model:** 1:1 deposit/redeem (NOT over-collateralized)
**Backing:** 100% USDT/USDC reserves
**Smart Contracts:** Solidity 0.8.x

#### Development Instructions

**Step 1: Smart Contracts (2-3 weeks)**

```solidity
// contracts/sUSD.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract sUSD is ERC20, AccessControl, Pausable {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    IERC20 public usdt;
    IERC20 public usdc;

    uint256 public minimumDeposit = 10 * 1e6; // 10 USDT/USDC

    constructor(address _usdt, address _usdc) ERC20("Selendra USD", "sUSD") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        usdt = IERC20(_usdt);
        usdc = IERC20(_usdc);
    }

    // Deposit USDT and get sUSD 1:1
    function depositUSDT(uint256 amount) external whenNotPaused {
        require(amount >= minimumDeposit, "Amount too small");

        // Transfer USDT from user to contract
        usdt.transferFrom(msg.sender, address(this), amount);

        // Mint sUSD to user (1:1)
        _mint(msg.sender, amount * 1e12); // Convert 6 decimals to 18
    }

    // Deposit USDC and get sUSD 1:1
    function depositUSDC(uint256 amount) external whenNotPaused {
        require(amount >= minimumDeposit, "Amount too small");

        // Transfer USDC from user to contract
        usdc.transferFrom(msg.sender, address(this), amount);

        // Mint sUSD to user (1:1)
        _mint(msg.sender, amount * 1e12); // Convert 6 decimals to 18
    }

    // Redeem sUSD for USDT
    function redeemForUSDT(uint256 amount) external whenNotPaused {
        require(amount >= minimumDeposit * 1e12, "Amount too small");

        // Burn sUSD from user
        _burn(msg.sender, amount);

        // Transfer USDT to user (1:1)
        usdt.transfer(msg.sender, amount / 1e12); // Convert 18 decimals to 6
    }

    // Redeem sUSD for USDC
    function redeemForUSDC(uint256 amount) external whenNotPaused {
        require(amount >= minimumDeposit * 1e12, "Amount too small");

        // Burn sUSD from user
        _burn(msg.sender, amount);

        // Transfer USDC to user (1:1)
        usdc.transfer(msg.sender, amount / 1e12); // Convert 18 decimals to 6
    }

    // View functions
    function getReserves() external view returns (uint256 usdtReserve, uint256 usdcReserve) {
        usdtReserve = usdt.balanceOf(address(this));
        usdcReserve = usdc.balanceOf(address(this));
    }

    function totalSupply() public view override returns (uint256) {
        return super.totalSupply();
    }

    // Admin functions
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    function setMinimumDeposit(uint256 _minimum) external onlyRole(DEFAULT_ADMIN_ROLE) {
        minimumDeposit = _minimum;
    }
}
```

```bash
# Deploy script
# scripts/deploy-susd.ts
async function main() {
  const sUSD = await ethers.deployContract("sUSD", [USDT_ADDRESS, USDC_ADDRESS])
  await sUSD.waitForDeployment()
  console.log("sUSD deployed to:", await sUSD.getAddress())
}

# Deploy
npx hardhat run scripts/deploy-susd.ts --network selendra

# Verify
npx hardhat verify --network selendra CONTRACT_ADDRESS USDT_ADDRESS USDC_ADDRESS
```

**Step 2: Build Frontend (1-2 weeks)**

```typescript
// components/sUSDInterface.tsx
import { useState } from 'react'
import { useContractWrite, useBalance } from 'wagmi'

export function sUSDInterface() {
  const [amount, setAmount] = useState('')
  const [selectedToken, setSelectedToken] = useState<'USDT' | 'USDC'>('USDT')
  const [mode, setMode] = useState<'deposit' | 'redeem'>('deposit')

  // Deposit function
  const { write: deposit } = useContractWrite({
    address: SUSD_CONTRACT_ADDRESS,
    abi: sUSD_ABI,
    functionName: selectedToken === 'USDT' ? 'depositUSDT' : 'depositUSDC',
  })

  // Redeem function
  const { write: redeem } = useContractWrite({
    address: SUSD_CONTRACT_ADDRESS,
    abi: sUSD_ABI,
    functionName: selectedToken === 'USDT' ? 'redeemForUSDT' : 'redeemForUSDC',
  })

  return (
    <div className="susd-interface">
      {/* Mode tabs: Deposit | Redeem */}
      <div className="tabs">
        <button onClick={() => setMode('deposit')}>Deposit</button>
        <button onClick={() => setMode('redeem')}>Redeem</button>
      </div>

      {mode === 'deposit' ? (
        <div>
          <h3>Deposit {selectedToken} → Get sUSD</h3>
          <select value={selectedToken} onChange={(e) => setSelectedToken(e.target.value)}>
            <option value="USDT">USDT</option>
            <option value="USDC">USDC</option>
          </select>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Enter amount"
          />
          <div>You will receive: {amount} sUSD</div>
          <button onClick={() => deposit({ args: [parseAmount(amount)] })}>
            Deposit
          </button>
        </div>
      ) : (
        <div>
          <h3>Redeem sUSD → Get {selectedToken}</h3>
          <select value={selectedToken} onChange={(e) => setSelectedToken(e.target.value)}>
            <option value="USDT">USDT</option>
            <option value="USDC">USDC</option>
          </select>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Enter sUSD amount"
          />
          <div>You will receive: {amount} {selectedToken}</div>
          <button onClick={() => redeem({ args: [parseAmount(amount)] })}>
            Redeem
          </button>
        </div>
      )}
    </div>
  )
}

// Proof of Reserves Dashboard
// components/sUSDReserves.tsx
export function sUSDReserves() {
  const { data: reserves } = useContractRead({
    address: SUSD_CONTRACT_ADDRESS,
    abi: sUSD_ABI,
    functionName: 'getReserves',
  })

  const { data: totalSupply } = useContractRead({
    address: SUSD_CONTRACT_ADDRESS,
    abi: sUSD_ABI,
    functionName: 'totalSupply',
  })

  const reserveRatio = (reserves.usdt + reserves.usdc) / totalSupply * 100

  return (
    <div className="reserves-dashboard">
      <h2>Proof of Reserves</h2>
      <div>USDT Reserves: {formatAmount(reserves.usdt)}</div>
      <div>USDC Reserves: {formatAmount(reserves.usdc)}</div>
      <div>Total Reserves: {formatAmount(reserves.usdt + reserves.usdc)}</div>
      <div>Total sUSD Minted: {formatAmount(totalSupply)}</div>
      <div>Reserve Ratio: {reserveRatio.toFixed(2)}%</div>
      {reserveRatio < 100 && <div className="warning">⚠️ Reserves below 100%!</div>}
    </div>
  )
}
```

**Step 3: Security Audit (1-2 weeks)**

```bash
# 1. Run static analysis
npm install -g slither
slither contracts/sUSD.sol

# 2. Run tests
npx hardhat test

# 3. Get professional audit
# - Contact CertiK, OpenZeppelin, or similar
# - Provide contract code and documentation
# - Fix any issues found
# - Get audit report

# 4. Set up monitoring
# - Alert if reserve ratio drops below 100%
# - Alert on large deposits/redeems
# - Monitor for unusual activity
```

#### Acceptance Criteria

- [ ] sUSD contract deployed on Selendra mainnet
- [ ] Users can deposit USDT and receive sUSD 1:1
- [ ] Users can deposit USDC and receive sUSD 1:1
- [ ] Users can redeem sUSD for USDT 1:1
- [ ] Users can redeem sUSD for USDC 1:1
- [ ] Minimum deposit/redeem is 10 tokens
- [ ] Emergency pause function works
- [ ] Multi-sig controls admin functions
- [ ] Proof of Reserves dashboard is public
- [ ] Reserve ratio is always >= 100%
- [ ] Contract verified on block explorer
- [ ] Security audit completed with no critical issues
- [ ] At least $100K in initial reserves

#### Configuration

```typescript
// config/susd.ts
export const SUSD_CONFIG = {
  contractAddress: '0x...', // After deployment
  usdtAddress: '0x...', // Bridged USDT on Selendra
  usdcAddress: '0x...', // Bridged USDC on Selendra
  minimumDeposit: 10,
  adminMultisig: '0x...', // Gnosis Safe address
}
```

---

## PHASE 2: STABLECOIN LAUNCH (Q2 2025 - Months 4-6)

**Goal:** Launch KHRt with Baray integration and start merchant adoption

---

### TASK 2.1: KHRt Stablecoin System

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Your key differentiator
**Timeline:** 10-12 weeks
**Dependencies:** Baray API access (✅ have), sUSD or USDT/USDC for crypto collateral
**Team:** 2 developers (smart contracts + backend) + 1 frontend + legal counsel + Claude Code

#### Purpose
Create dual-collateralized stablecoin pegged to Cambodian Riel (1 KHRt = 1 KHR), backed by KHR in banks via Baray + crypto collateral.

#### Technical Specs

**Model:** Dual-collateralized RWA-backed stablecoin
- On-chain: sUSD, USDT, or USDC
- Off-chain: KHR in bank accounts (ABA, ACLEDA, WING via Baray)
- Start with 100% fiat backing, evolve to hybrid

**Components:**
1. Smart contracts (on-chain minting/burning)
2. Reserve management backend (bank integration)
3. Minting service (KHQR payments → auto-mint)
4. Proof of Reserves dashboard (public transparency)
5. User interfaces (add money, cash out)

#### Development Instructions

**Step 1: Smart Contracts (3-4 weeks)**

```solidity
// contracts/KHRt.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract KHRt is ERC20, AccessControl, Pausable {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    // Track reserves
    uint256 public fiatReserves; // KHR in banks (off-chain, updated by oracle)
    uint256 public cryptoReserves; // sUSD/USDT on-chain

    // Events
    event Minted(address indexed to, uint256 amount, string paymentId);
    event Burned(address indexed from, uint256 amount, string withdrawalId);
    event ReservesUpdated(uint256 fiatReserves, uint256 cryptoReserves);

    constructor() ERC20("Cambodian Riel Token", "KHRt") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    // Mint KHRt (only by authorized minter backend)
    function mint(address to, uint256 amount, string memory paymentId)
        external
        onlyRole(MINTER_ROLE)
        whenNotPaused
    {
        require(amount > 0, "Amount must be positive");
        _mint(to, amount);
        emit Minted(to, amount, paymentId);
    }

    // Burn KHRt (for withdrawals)
    function burn(address from, uint256 amount, string memory withdrawalId)
        external
        onlyRole(BURNER_ROLE)
        whenNotPaused
    {
        require(amount > 0, "Amount must be positive");
        _burn(from, amount);
        emit Burned(from, amount, withdrawalId);
    }

    // Update reserve amounts (called by oracle)
    function updateReserves(uint256 _fiatReserves, uint256 _cryptoReserves)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        fiatReserves = _fiatReserves;
        cryptoReserves = _cryptoReserves;
        emit ReservesUpdated(_fiatReserves, _cryptoReserves);
    }

    // View collateralization ratio
    function collateralizationRatio() external view returns (uint256) {
        uint256 supply = totalSupply();
        if (supply == 0) return 0;
        return ((fiatReserves + cryptoReserves) * 100) / supply;
    }

    // Admin functions
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
```

**Step 2: Reserve Management Backend (3-4 weeks)**

```typescript
// backend/services/reserveManager.ts
import axios from 'axios'

export class ReserveManager {
  private barayApiUrl: string
  private bankAccounts: BankAccount[]

  constructor() {
    this.barayApiUrl = process.env.BARAY_API_URL
    this.bankAccounts = [
      { bank: 'ABA', accountNumber: '...', apiKey: '...' },
      { bank: 'ACLEDA', accountNumber: '...', apiKey: '...' },
      { bank: 'WING', accountNumber: '...', apiKey: '...' },
    ]
  }

  // Get total KHR in all bank accounts
  async getTotalFiatReserves(): Promise<number> {
    let total = 0

    for (const account of this.bankAccounts) {
      const balance = await this.getBankBalance(account)
      total += balance
    }

    return total
  }

  // Get balance from specific bank via Baray
  private async getBankBalance(account: BankAccount): Promise<number> {
    const response = await axios.get(`${this.barayApiUrl}/balance`, {
      headers: { 'Authorization': `Bearer ${account.apiKey}` },
      params: { accountNumber: account.accountNumber }
    })

    return response.data.balance
  }

  // Get total crypto reserves (sUSD/USDT on-chain)
  async getTotalCryptoReserves(): Promise<number> {
    // Query blockchain for sUSD/USDT balance in reserve wallet
    const web3 = new Web3(SELENDRA_RPC)
    const sUSD = new web3.eth.Contract(SUSD_ABI, SUSD_ADDRESS)
    const balance = await sUSD.methods.balanceOf(RESERVE_WALLET).call()
    return Number(balance)
  }

  // Update on-chain reserve tracking (run hourly)
  async updateOnChainReserves(): Promise<void> {
    const fiatReserves = await this.getTotalFiatReserves()
    const cryptoReserves = await this.getTotalCryptoReserves()

    // Update smart contract
    const contract = new web3.eth.Contract(KHRT_ABI, KHRT_ADDRESS)
    await contract.methods.updateReserves(fiatReserves, cryptoReserves).send({
      from: ADMIN_ADDRESS,
      gas: 100000
    })

    // Log to database for audit trail
    await db.reserveUpdates.create({
      timestamp: new Date(),
      fiatReserves,
      cryptoReserves,
      totalSupply: await contract.methods.totalSupply().call(),
      ratio: ((fiatReserves + cryptoReserves) / totalSupply * 100)
    })

    // Alert if ratio < 100%
    if (ratio < 100) {
      await this.sendAlert('CRITICAL: Reserve ratio below 100%!')
    }
  }

  // Send KHR to user's bank account via Baray
  async sendToBank(userId: string, bankAccount: string, amountKHR: number): Promise<string> {
    const response = await axios.post(`${this.barayApiUrl}/transfer`, {
      from: this.bankAccounts[0].accountNumber, // Use primary account
      to: bankAccount,
      amount: amountKHR,
      reference: `KHRt withdrawal for user ${userId}`
    }, {
      headers: { 'Authorization': `Bearer ${this.bankAccounts[0].apiKey}` }
    })

    return response.data.transactionId
  }
}

// Cron job to update reserves hourly
// scripts/updateReserves.ts
import cron from 'node-cron'

const reserveManager = new ReserveManager()

// Run every hour
cron.schedule('0 * * * *', async () => {
  console.log('Updating reserves...')
  await reserveManager.updateOnChainReserves()
  console.log('Reserves updated successfully')
})
```

**Step 3: Minting Service (2-3 weeks)**

```typescript
// backend/services/mintingService.ts
export class MintingService {
  private khrtContract: Contract
  private reserveManager: ReserveManager

  // Handle KHQR payment → mint KHRt
  async handleKHQRPayment(paymentId: string): Promise<void> {
    // 1. Get payment details from Baray
    const payment = await this.barayApi.getPayment(paymentId)

    // 2. Verify payment is confirmed
    if (payment.status !== 'CONFIRMED') {
      throw new Error('Payment not confirmed')
    }

    // 3. Check if already processed
    const existing = await db.payments.findOne({ paymentId })
    if (existing) {
      throw new Error('Payment already processed')
    }

    // 4. Get user's wallet address
    const user = await db.users.findOne({ id: payment.userId })
    if (!user || !user.walletAddress) {
      throw new Error('User wallet not found')
    }

    // 5. Calculate KHRt amount (1 KHR = 1 KHRt)
    const khrtAmount = payment.amountKHR // 1:1

    // 6. Mint KHRt to user's wallet
    const tx = await this.khrtContract.mint(
      user.walletAddress,
      khrtAmount,
      paymentId
    )

    // 7. Wait for confirmation
    await tx.wait()

    // 8. Record in database
    await db.payments.create({
      paymentId,
      userId: user.id,
      amountKHR: payment.amountKHR,
      amountKHRt: khrtAmount,
      txHash: tx.hash,
      timestamp: new Date(),
      type: 'DEPOSIT'
    })

    // 9. Notify user
    await this.notificationService.send(user.id, {
      title: 'KHRt Received',
      body: `${khrtAmount} KHRt has been added to your wallet`
    })
  }

  // Handle KHRt withdrawal → send KHR to bank
  async handleWithdrawal(userId: string, amountKHRt: number, bankAccount: string): Promise<string> {
    // 1. Get user's wallet address
    const user = await db.users.findOne({ id: userId })

    // 2. Verify user has enough KHRt
    const balance = await this.khrtContract.balanceOf(user.walletAddress)
    if (balance < amountKHRt) {
      throw new Error('Insufficient balance')
    }

    // 3. Check daily limit
    const todayTotal = await this.getTodayWithdrawals(userId)
    if (todayTotal + amountKHRt > DAILY_LIMIT) {
      throw new Error('Daily withdrawal limit exceeded')
    }

    // 4. Burn KHRt from user's wallet
    const withdrawalId = generateId()
    const burnTx = await this.khrtContract.burn(
      user.walletAddress,
      amountKHRt,
      withdrawalId
    )
    await burnTx.wait()

    // 5. Send KHR to user's bank via Baray
    const transferId = await this.reserveManager.sendToBank(
      userId,
      bankAccount,
      amountKHRt // 1:1 conversion
    )

    // 6. Record withdrawal
    await db.withdrawals.create({
      withdrawalId,
      userId,
      amountKHRt,
      amountKHR: amountKHRt,
      bankAccount,
      transferId,
      burnTxHash: burnTx.hash,
      timestamp: new Date(),
      status: 'COMPLETED'
    })

    return withdrawalId
  }
}

// Webhook to listen for Baray payments
// routes/webhooks.ts
app.post('/webhooks/baray/payment', async (req, res) => {
  const { paymentId, status } = req.body

  if (status === 'CONFIRMED') {
    // Process payment asynchronously
    await mintingService.handleKHQRPayment(paymentId)
  }

  res.status(200).send('OK')
})
```

**Step 4: User Interfaces (2 weeks)**

```typescript
// components/AddMoney.tsx (KHQR on-ramp)
export function AddMoney() {
  const [amount, setAmount] = useState('')
  const [qrCode, setQrCode] = useState<string | null>(null)
  const [status, setStatus] = useState<'idle' | 'generating' | 'waiting' | 'completed'>('idle')

  async function generateKHQR() {
    setStatus('generating')

    // Call backend to generate KHQR
    const response = await fetch('/api/khqr/generate', {
      method: 'POST',
      body: JSON.stringify({ amount, userId: currentUser.id })
    })

    const { qrCode, paymentId } = await response.json()
    setQrCode(qrCode)
    setStatus('waiting')

    // Poll for payment confirmation
    const interval = setInterval(async () => {
      const statusRes = await fetch(`/api/khqr/status/${paymentId}`)
      const { status } = await statusRes.json()

      if (status === 'CONFIRMED') {
        setStatus('completed')
        clearInterval(interval)
      }
    }, 3000) // Check every 3 seconds
  }

  return (
    <div className="add-money">
      <h2>Add Money</h2>

      {status === 'idle' && (
        <div>
          <label>Amount (KHR)</label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Enter amount in KHR"
          />
          <div>You will receive: {amount} KHRt</div>
          <button onClick={generateKHQR}>Generate KHQR Code</button>
        </div>
      )}

      {status === 'waiting' && qrCode && (
        <div>
          <h3>Scan to Pay</h3>
          <QRCode value={qrCode} size={256} />
          <p>Waiting for payment confirmation...</p>
          <div className="spinner" />
        </div>
      )}

      {status === 'completed' && (
        <div className="success">
          <h3>✅ Payment Confirmed!</h3>
          <p>{amount} KHRt has been added to your wallet</p>
        </div>
      )}
    </div>
  )
}

// components/CashOut.tsx (Bank off-ramp)
export function CashOut() {
  const [amount, setAmount] = useState('')
  const [bankAccount, setBankAccount] = useState('')
  const [status, setStatus] = useState<'idle' | 'processing' | 'completed'>('idle')

  async function withdraw() {
    setStatus('processing')

    const response = await fetch('/api/khrt/withdraw', {
      method: 'POST',
      body: JSON.stringify({
        userId: currentUser.id,
        amount,
        bankAccount
      })
    })

    if (response.ok) {
      setStatus('completed')
    } else {
      const error = await response.json()
      alert(error.message)
      setStatus('idle')
    }
  }

  return (
    <div className="cash-out">
      <h2>Cash Out</h2>

      {status === 'idle' && (
        <div>
          <label>Amount (KHRt)</label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Enter amount in KHRt"
          />
          <div>You will receive: {amount} KHR in your bank</div>

          <label>Bank Account</label>
          <select value={bankAccount} onChange={(e) => setBankAccount(e.target.value)}>
            <option value="">Select saved account</option>
            {savedAccounts.map(acc => (
              <option key={acc.id} value={acc.accountNumber}>
                {acc.bank} - {acc.accountNumber}
              </option>
            ))}
          </select>

          <button onClick={withdraw}>Cash Out</button>
        </div>
      )}

      {status === 'processing' && (
        <div>
          <h3>Processing withdrawal...</h3>
          <div className="spinner" />
        </div>
      )}

      {status === 'completed' && (
        <div className="success">
          <h3>✅ Withdrawal Complete!</h3>
          <p>{amount} KHR has been sent to your bank account</p>
        </div>
      )}
    </div>
  )
}

// components/ReservesProof.tsx (Public dashboard)
export function ReservesProof() {
  const { data: reserves } = useContractRead({
    address: KHRT_ADDRESS,
    abi: KHRT_ABI,
    functionName: 'fiatReserves'
  })

  const { data: cryptoReserves } = useContractRead({
    address: KHRT_ADDRESS,
    abi: KHRT_ABI,
    functionName: 'cryptoReserves'
  })

  const { data: totalSupply } = useContractRead({
    address: KHRT_ADDRESS,
    abi: KHRT_ABI,
    functionName: 'totalSupply'
  })

  const ratio = ((reserves + cryptoReserves) / totalSupply * 100).toFixed(2)

  return (
    <div className="reserves-proof">
      <h2>Proof of Reserves</h2>
      <div className="reserves-grid">
        <div className="reserve-item">
          <h3>Fiat Reserves (KHR in Banks)</h3>
          <div className="amount">{formatNumber(reserves)} KHR</div>
          <div className="banks">
            <span>ABA: {formatNumber(abaBalance)}</span>
            <span>ACLEDA: {formatNumber(acledaBalance)}</span>
            <span>WING: {formatNumber(wingBalance)}</span>
          </div>
        </div>

        <div className="reserve-item">
          <h3>Crypto Reserves (sUSD/USDT)</h3>
          <div className="amount">{formatNumber(cryptoReserves)} USD</div>
          <a href={`${EXPLORER_URL}/address/${RESERVE_WALLET}`}>
            View on Explorer →
          </a>
        </div>

        <div className="reserve-item">
          <h3>Total KHRt Minted</h3>
          <div className="amount">{formatNumber(totalSupply)} KHRt</div>
        </div>

        <div className="reserve-item">
          <h3>Collateralization Ratio</h3>
          <div className={`amount ${ratio < 100 ? 'warning' : 'success'}`}>
            {ratio}%
          </div>
          {ratio < 100 && <div className="alert">⚠️ Below 100%</div>}
        </div>
      </div>

      <div className="historical-chart">
        <h3>Historical Reserve Ratio</h3>
        {/* Chart showing ratio over time */}
      </div>

      <div className="audit-reports">
        <h3>Third-Party Audit Reports</h3>
        <ul>
          <li><a href="/audits/2025-01.pdf">January 2025 Audit Report</a></li>
          {/* More audit reports */}
        </ul>
      </div>
    </div>
  )
}
```

**Step 5: Conservative Launch Strategy (3 months beta)**

```typescript
// config/khrt-launch.ts
export const LAUNCH_PHASES = {
  // Month 1-2: Development + Audits
  development: {
    duration: '8 weeks',
    tasks: ['Smart contracts', 'Backend', 'Audits', 'Testing']
  },

  // Month 3: Closed Beta
  closedBeta: {
    maxUsers: 100,
    maxSupply: 10000 * 4000, // $10K USD worth (40M KHR)
    inviteOnly: true,
    monitoring: 'Real-time, manual review of every transaction'
  },

  // Month 4: Expanded Beta
  expandedBeta: {
    maxUsers: 1000,
    maxSupply: 100000 * 4000, // $100K USD worth (400M KHR)
    waitlist: true,
    monitoring: 'Real-time with automated alerts'
  },

  // Month 5: Open Beta
  openBeta: {
    maxUsers: 10000,
    maxSupply: 500000 * 4000, // $500K USD worth (2B KHR)
    publicSignup: true,
    monitoring: 'Automated with escalation'
  },

  // Month 6: Public Launch
  publicLaunch: {
    maxUsers: Infinity,
    maxSupply: 5000000 * 4000, // $5M USD cap initially (20B KHR)
    fullMarketing: true
  }
}
```

#### Acceptance Criteria

- [ ] KHRt smart contract deployed on mainnet
- [ ] Reserve management backend running with hourly updates
- [ ] Integration with Baray API working (KHQR + bank transfers)
- [ ] Users can add money via KHQR and receive KHRt
- [ ] Users can cash out KHRt to bank account
- [ ] Proof of Reserves dashboard is public and accurate
- [ ] Reserve ratio stays above 100% at all times
- [ ] Multi-sig controls all admin functions
- [ ] Emergency pause works
- [ ] KYC/AML integrated
- [ ] Daily/monthly withdrawal limits enforced
- [ ] Closed beta completed with 100 users
- [ ] No critical bugs found in beta
- [ ] Security audit completed
- [ ] Legal clearance from National Bank of Cambodia

#### Configuration

```typescript
// config/khrt.ts
export const KHRT_CONFIG = {
  contractAddress: '0x...', // After deployment
  reserveWallet: '0x...', // Multi-sig for crypto reserves

  baray: {
    apiUrl: 'https://api.baray.io',
    apiKey: process.env.BARAY_API_KEY,
    webhookSecret: process.env.BARAY_WEBHOOK_SECRET
  },

  banks: [
    { name: 'ABA', accountNumber: '...', apiKey: '...' },
    { name: 'ACLEDA', accountNumber: '...', apiKey: '...' },
    { name: 'WING', accountNumber: '...', apiKey: '...' },
  ],

  limits: {
    minDeposit: 10000, // 10,000 KHR minimum (~$2.50)
    minWithdrawal: 10000,
    dailyWithdrawalLimit: 4000000, // 4M KHR (~$1,000)
    monthlyWithdrawalLimit: 40000000, // 40M KHR (~$10,000)
  },

  monitoring: {
    alertEmail: 'alerts@selendra.org',
    criticalThreshold: 100, // Alert if reserves < 100%
    warningThreshold: 110, // Warning if reserves < 110%
  }
}
```

---

### TASK 2.2: Merchant Payment Gateway (Riverbase Plugin)

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL
**Timeline:** 8-10 weeks (4-6 weeks for Riverbase plugin + 4 weeks for standalone)
**Dependencies:** KHRt, wallet, Baray integration
**Team:** 2 developers (backend + apps) + merchant partnerships + Claude Code

#### Purpose
Build crypto payment gateway with **Riverbase.app plugin as highest priority** to enable 100+ existing merchants to accept crypto payments quickly.

#### Technical Specs

**Components:**
1. **Riverbase Plugin** (PRIORITY 1) - Integrate into existing Riverbase platform
2. Point-of-Sale App (for in-person payments)
3. REST API (for custom integrations)
4. E-commerce plugins (WooCommerce, Shopify - later)

**Riverbase Advantage:**
- 100+ SME stores already on platform
- Merchants already trust Baray
- Can enable crypto with simple toggle
- Reach 50+ merchants in weeks instead of months

#### Development Instructions

**Step 1: Riverbase Plugin (4-6 weeks) - HIGHEST PRIORITY**

```typescript
// This gets integrated into Riverbase codebase
// riverbase-plugin/cryptoPayments.ts

export class CryptoPaymentsPlugin {
  // Enable/disable crypto for a merchant
  async enableCrypto(merchantId: string): Promise<void> {
    await db.merchants.update(merchantId, {
      cryptoEnabled: true,
      acceptedTokens: ['KHRt', 'sUSD', 'STAR', 'SEL']
    })
  }

  // Generate payment request
  async createPayment(merchantId: string, amount: number, currency: 'KHR'): Promise<Payment> {
    const merchant = await db.merchants.findOne(merchantId)

    // Generate unique payment ID
    const paymentId = generatePaymentId()

    // Create payment record
    const payment = await db.payments.create({
      paymentId,
      merchantId,
      amount,
      currency,
      status: 'PENDING',
      acceptedTokens: merchant.acceptedTokens,
      createdAt: new Date(),
      expiresAt: new Date(Date.now() + 15 * 60 * 1000) // 15 min expiry
    })

    // Generate payment URL
    const paymentUrl = `https://pay.riverbase.app/${paymentId}`

    return { paymentId, paymentUrl, qrCode: generateQR(paymentUrl) }
  }

  // Check payment status
  async checkPayment(paymentId: string): Promise<PaymentStatus> {
    const payment = await db.payments.findOne(paymentId)

    if (!payment) throw new Error('Payment not found')

    // Check on-chain if payment received
    if (payment.status === 'PENDING') {
      const received = await this.checkBlockchain(payment)
      if (received) {
        await db.payments.update(paymentId, {
          status: 'CONFIRMED',
          paidAt: new Date(),
          txHash: received.txHash,
          token: received.token,
          amountPaid: received.amount
        })
      }
    }

    return payment
  }

  // Merchant settlement
  async settleMerchant(merchantId: string): Promise<Settlement> {
    const merchant = await db.merchants.findOne(merchantId)
    const pendingPayments = await this.getPendingSettlement(merchantId)

    const total = pendingPayments.reduce((sum, p) => sum + p.amount, 0)

    // Merchant's settlement preference
    switch (merchant.settlementPreference) {
      case 'HOLD_CRYPTO':
        // Keep in merchant's crypto wallet
        return { method: 'HOLD', amount: total }

      case 'AUTO_CONVERT_KHRT':
        // Convert everything to KHRt and hold
        const khrtAmount = await this.convertToKHRt(total)
        return { method: 'CONVERT', token: 'KHRt', amount: khrtAmount }

      case 'CASH_OUT_BANK':
        // Convert to KHRt, then cash out via Baray
        const khrAmount = await this.convertAndCashOut(merchantId, total)
        return { method: 'BANK', amount: khrAmount }
    }
  }
}

// Integration into Riverbase checkout
// riverbase-plugin/CheckoutPage.tsx
export function CheckoutPage({ orderId, totalKHR }: Props) {
  const merchant = useMerchant()
  const [paymentMethod, setPaymentMethod] = useState<'card' | 'bank' | 'crypto'>('card')

  return (
    <div className="checkout">
      <h2>Payment</h2>

      <div className="payment-methods">
        <button onClick={() => setPaymentMethod('card')}>Credit Card</button>
        <button onClick={() => setPaymentMethod('bank')}>Bank Transfer</button>

        {/* NEW: Crypto option if merchant enabled */}
        {merchant.cryptoEnabled && (
          <button onClick={() => setPaymentMethod('crypto')}>
            💎 Crypto (KHRt, STAR, sUSD)
          </button>
        )}
      </div>

      {paymentMethod === 'crypto' && (
        <CryptoPayment orderId={orderId} amount={totalKHR} merchant={merchant} />
      )}
    </div>
  )
}

// riverbase-plugin/CryptoPayment.tsx
function CryptoPayment({ orderId, amount, merchant }: Props) {
  const [payment, setPayment] = useState<Payment | null>(null)
  const [selectedToken, setSelectedToken] = useState<Token>('KHRt')

  useEffect(() => {
    // Create payment request
    createPayment(merchant.id, amount, 'KHR').then(setPayment)

    // Poll for payment status
    const interval = setInterval(async () => {
      const status = await checkPayment(payment.paymentId)
      if (status === 'CONFIRMED') {
        // Payment received!
        window.location.href = `/order/${orderId}/success`
      }
    }, 3000)

    return () => clearInterval(interval)
  }, [])

  if (!payment) return <div>Loading...</div>

  return (
    <div className="crypto-payment">
      <h3>Pay with Crypto</h3>

      <div className="token-selector">
        {merchant.acceptedTokens.map(token => (
          <button
            key={token}
            onClick={() => setSelectedToken(token)}
            className={selectedToken === token ? 'selected' : ''}
          >
            {token}
          </button>
        ))}
      </div>

      <div className="amount">
        <div>Amount: {amount} KHR</div>
        <div>≈ {convertAmount(amount, selectedToken)} {selectedToken}</div>
      </div>

      <QRCode value={payment.paymentUrl} size={256} />

      <div className="instructions">
        <p>1. Open your Selendra wallet</p>
        <p>2. Scan QR code or copy address below</p>
        <p>3. Send {convertAmount(amount, selectedToken)} {selectedToken}</p>
      </div>

      <div className="payment-address">
        <code>{payment.address}</code>
        <button onClick={() => copy(payment.address)}>Copy</button>
      </div>

      <div className="waiting">
        Waiting for payment...
        <div className="spinner" />
      </div>
    </div>
  )
}

// Admin panel for Riverbase merchants
// riverbase-plugin/MerchantDashboard.tsx
function CryptoSettings() {
  const merchant = useMerchant()

  return (
    <div className="crypto-settings">
      <h2>Crypto Payments</h2>

      <div className="enable-toggle">
        <label>
          <input
            type="checkbox"
            checked={merchant.cryptoEnabled}
            onChange={(e) => updateMerchant({ cryptoEnabled: e.target.checked })}
          />
          Accept crypto payments
        </label>
      </div>

      {merchant.cryptoEnabled && (
        <div>
          <h3>Accepted Tokens</h3>
          <div className="token-checkboxes">
            {['KHRt', 'sUSD', 'STAR', 'SEL'].map(token => (
              <label key={token}>
                <input
                  type="checkbox"
                  checked={merchant.acceptedTokens.includes(token)}
                  onChange={(e) => toggleToken(token, e.target.checked)}
                />
                {token}
              </label>
            ))}
          </div>

          <h3>Settlement Preference</h3>
          <select
            value={merchant.settlementPreference}
            onChange={(e) => updateMerchant({ settlementPreference: e.target.value })}
          >
            <option value="HOLD_CRYPTO">Hold in crypto wallet</option>
            <option value="AUTO_CONVERT_KHRT">Auto-convert to KHRt</option>
            <option value="CASH_OUT_BANK">Auto cash-out to bank (via Baray)</option>
          </select>

          <h3>Analytics</h3>
          <div className="stats">
            <div>Total crypto payments: {merchant.stats.cryptoPaymentsCount}</div>
            <div>Total volume: {merchant.stats.cryptoVolumeKHR} KHR</div>
            <div>Avg transaction: {merchant.stats.avgCryptoPayment} KHR</div>
          </div>
        </div>
      )}
    </div>
  )
}
```

**Step 2: Point-of-Sale App (2-3 weeks) - For in-person payments**

```typescript
// Mobile app: React Native
// apps/pos/PaymentScreen.tsx

export function PaymentScreen() {
  const [amount, setAmount] = useState('')
  const [paymentRequest, setPaymentRequest] = useState<PaymentRequest | null>(null)

  async function createPaymentRequest() {
    const req = await fetch('/api/payments/create', {
      method: 'POST',
      body: JSON.stringify({
        merchantId: currentMerchant.id,
        amount: parseFloat(amount),
        currency: 'KHR'
      })
    }).then(r => r.json())

    setPaymentRequest(req)

    // Start polling for payment
    pollForPayment(req.paymentId)
  }

  async function pollForPayment(paymentId: string) {
    const interval = setInterval(async () => {
      const status = await checkPaymentStatus(paymentId)
      if (status === 'CONFIRMED') {
        clearInterval(interval)
        // Show success screen
        setPaymentComplete(true)
        // Print receipt
        await printReceipt(paymentId)
      }
    }, 2000)
  }

  return (
    <View style={styles.container}>
      {!paymentRequest ? (
        <View>
          <Text style={styles.label}>Enter Amount (KHR)</Text>
          <TextInput
            style={styles.input}
            value={amount}
            onChangeText={setAmount}
            keyboardType="numeric"
            placeholder="0"
          />
          <TouchableOpacity style={styles.button} onPress={createPaymentRequest}>
            <Text>Generate Payment QR</Text>
          </TouchableOpacity>
        </View>
      ) : (
        <View style={styles.paymentView}>
          <Text style={styles.title}>Customer: Scan to Pay</Text>
          <QRCode value={paymentRequest.paymentUrl} size={300} />
          <Text style={styles.amount}>{amount} KHR</Text>
          <Text style={styles.status}>Waiting for payment...</Text>
          <ActivityIndicator size="large" />
        </View>
      )}
    </View>
  )
}
```

**Step 3: REST API (1 week) - For custom integrations**

```typescript
// backend/routes/payments.ts
import express from 'express'

const router = express.Router()

// Create payment
router.post('/create', authenticateMerchant, async (req, res) => {
  const { amount, currency } = req.body
  const merchantId = req.merchant.id

  const payment = await paymentService.createPayment(merchantId, amount, currency)

  res.json(payment)
})

// Check payment status
router.get('/status/:paymentId', async (req, res) => {
  const { paymentId } = req.params

  const status = await paymentService.checkPayment(paymentId)

  res.json(status)
})

// Webhook for payment confirmation (merchants can subscribe)
router.post('/webhook', verifyWebhookSignature, async (req, res) => {
  const { paymentId, status } = req.body

  if (status === 'CONFIRMED') {
    // Notify merchant
    await notificationService.notifyMerchant(payment.merchantId, {
      type: 'PAYMENT_RECEIVED',
      paymentId,
      amount: payment.amount
    })
  }

  res.status(200).send('OK')
})

export default router

// API Documentation
/**
 * Merchant Payment Gateway API
 *
 * Authentication: API key via Bearer token
 *
 * POST /api/payments/create
 * Body: { amount: number, currency: 'KHR' }
 * Returns: { paymentId, paymentUrl, qrCode, expiresAt }
 *
 * GET /api/payments/status/:paymentId
 * Returns: { paymentId, status, amount, paidAt, txHash }
 *
 * Webhook: POST to your URL when payment confirmed
 * Payload: { paymentId, status: 'CONFIRMED', amount, token, txHash }
 */
```

#### Acceptance Criteria

**Riverbase Plugin:**
- [ ] Crypto payment toggle in Riverbase merchant dashboard
- [ ] Merchants can select accepted tokens (KHRt, sUSD, STAR, SEL)
- [ ] Crypto payment option shows on checkout for enabled merchants
- [ ] Payment QR code generation works
- [ ] Payment status polling and confirmation works
- [ ] Merchant settlement preferences work (hold/convert/cash-out)
- [ ] 10 Riverbase merchants in beta (June)
- [ ] 30 Riverbase merchants live (July)
- [ ] 50+ Riverbase merchants live (August)

**POS App:**
- [ ] Can generate payment QR code with amount
- [ ] Polls and confirms payment automatically
- [ ] Prints receipt after payment
- [ ] Works offline (queues transactions)
- [ ] Multi-employee support

**API:**
- [ ] API documentation published
- [ ] Webhook notifications work
- [ ] Rate limiting in place
- [ ] API keys managed securely

#### Configuration

```typescript
// config/payments.ts
export const PAYMENTS_CONFIG = {
  riverbase: {
    apiUrl: 'https://api.riverbase.app',
    pluginVersion: '1.0.0',
    featureFlag: 'crypto_payments_enabled'
  },

  fees: {
    percentage: 0.005, // 0.5%
    flatFee: 0, // No flat fee
    maxFee: 100000 // Max 100K KHR (~$25)
  },

  settlement: {
    frequency: 'daily', // or 'instant'
    minimumAmount: 40000, // 40K KHR minimum (~$10)
  },

  paymentExpiry: 15 * 60, // 15 minutes
  confirmationsRequired: 1, // 1 block confirmation
}
```

---

## PHASE 3: SPORTS EXPANSION (Q3 2025 - Months 7-9)

**Goal:** Prepare for Kun Khmer explosion with enhanced sports features

---

### TASK 3.1: Enhanced CPL Play Prediction Platform

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL - Key engagement driver for Kun Khmer launch
**Timeline:** 6-8 weeks
**Dependencies:** Enhanced wallet, KHRt (for multi-token)
**Team:** 1-2 developers + data feed integration + Claude Code

#### Purpose
Expand prediction platform to support Kun Khmer (10 events/week), add advanced prediction types, and improve engagement features for mass adoption.

#### Technical Specs

**Platform:** Web + Mobile (React Native)
**Current State:** Basic football outcome predictions
**Backend:** Node.js + TypeScript + PostgreSQL

**Key Enhancements:**
1. Kun Khmer prediction types (fights, KO/decision, rounds)
2. Advanced football predictions (score, players, in-game events)
3. Social features (friends, challenges, leaderboards)
4. Multi-token support (STAR, KHRt, sUSD)
5. Live predictions during matches

#### Development Instructions

**Step 1: Kun Khmer Prediction Types (2 weeks)**

```typescript
// types/predictions.ts
export interface KunKhmerPrediction {
  fightId: string
  type: 'OUTCOME' | 'METHOD' | 'ROUND' | 'DURATION' | 'TECHNIQUE'
  prediction: any
  stake: number
  token: 'STAR' | 'KHRt' | 'sUSD'
}

export const KUN_KHMER_PREDICTION_TYPES = {
  OUTCOME: {
    options: ['FIGHTER_A_WIN', 'FIGHTER_B_WIN', 'DRAW'],
    payout: { base: 1.5, boosted: 2.0 }
  },
  METHOD: {
    options: ['KO', 'DECISION', 'SUBMISSION', 'TKO'],
    payout: { base: 2.0, boosted: 3.0 }
  },
  ROUND: {
    options: ['ROUND_1', 'ROUND_2', 'ROUND_3', 'ROUND_4', 'ROUND_5'],
    payout: { base: 5.0, boosted: 8.0 }
  },
  DURATION: {
    options: ['UNDER_3_MIN', '3_TO_6_MIN', '6_TO_9_MIN', 'OVER_9_MIN'],
    payout: { base: 3.0, boosted: 5.0 }
  },
  TECHNIQUE: {
    options: ['ELBOW', 'KNEE', 'PUNCH', 'KICK', 'CLINCH'],
    payout: { base: 4.0, boosted: 6.0 }
  }
}

// backend/services/kunKhmerPredictions.ts
export class KunKhmerPredictionService {
  async createPrediction(userId: string, prediction: KunKhmerPrediction): Promise<string> {
    // 1. Validate fight exists and is not started
    const fight = await db.fights.findOne({ id: prediction.fightId })
    if (!fight || fight.status !== 'UPCOMING') {
      throw new Error('Fight not available for predictions')
    }

    // 2. Deduct stake from user's wallet
    await walletService.deductBalance(userId, prediction.stake, prediction.token)

    // 3. Create prediction record
    const predId = await db.predictions.create({
      userId,
      fightId: prediction.fightId,
      type: prediction.type,
      prediction: prediction.prediction,
      stake: prediction.stake,
      token: prediction.token,
      potentialPayout: this.calculatePayout(prediction),
      status: 'PENDING',
      createdAt: new Date()
    })

    // 4. Add to pool
    await db.predictionPools.addToPool(prediction.fightId, prediction.stake, prediction.token)

    return predId
  }

  async resolvePredictions(fightId: string, result: FightResult): Promise<void> {
    // 1. Get all predictions for this fight
    const predictions = await db.predictions.find({ fightId, status: 'PENDING' })

    // 2. Determine winners
    for (const pred of predictions) {
      const isCorrect = this.checkPrediction(pred, result)

      if (isCorrect) {
        const payout = pred.potentialPayout
        await walletService.addBalance(pred.userId, payout, pred.token)
        await db.predictions.update(pred.id, { status: 'WON', payout })

        // Send notification
        await notificationService.send(pred.userId, {
          title: 'Prediction Won! 🎉',
          body: `You won ${payout} ${pred.token}!`
        })
      } else {
        await db.predictions.update(pred.id, { status: 'LOST' })
      }
    }

    // 3. Update user stats
    await this.updateUserStats(predictions)
  }

  private checkPrediction(pred: Prediction, result: FightResult): boolean {
    switch (pred.type) {
      case 'OUTCOME':
        return pred.prediction === result.winner
      case 'METHOD':
        return pred.prediction === result.method
      case 'ROUND':
        return pred.prediction === `ROUND_${result.round}`
      case 'DURATION':
        return this.checkDuration(pred.prediction, result.duration)
      case 'TECHNIQUE':
        return pred.prediction === result.finishingTechnique
      default:
        return false
    }
  }
}
```

**Step 2: Advanced Football Predictions (1-2 weeks)**

```typescript
// Add more prediction types for football
export const FOOTBALL_PREDICTION_TYPES = {
  EXACT_SCORE: {
    type: 'input', // User inputs score like "2-1"
    payout: { base: 10.0, boosted: 15.0 }
  },
  FIRST_GOAL_SCORER: {
    type: 'select', // Select from team roster
    payout: { base: 8.0, boosted: 12.0 }
  },
  HALF_TIME_FULL_TIME: {
    options: ['HOME_HOME', 'HOME_DRAW', 'HOME_AWAY', 'DRAW_HOME', ...],
    payout: { base: 6.0, boosted: 9.0 }
  },
  TOTAL_GOALS: {
    options: ['UNDER_1_5', 'UNDER_2_5', 'UNDER_3_5', 'OVER_2_5', 'OVER_3_5'],
    payout: { base: 2.0, boosted: 3.0 }
  },
  BOTH_TEAMS_SCORE: {
    options: ['YES', 'NO'],
    payout: { base: 1.8, boosted: 2.5 }
  },
  PLAYER_PERFORMANCE: {
    metrics: ['GOALS', 'ASSISTS', 'YELLOW_CARD', 'RED_CARD'],
    payout: { base: 4.0, boosted: 6.0 }
  }
}

// components/PredictionForm.tsx
export function PredictionForm({ event }: Props) {
  const [predictionType, setPredictionType] = useState('OUTCOME')
  const [prediction, setPrediction] = useState<any>(null)
  const [stake, setStake] = useState('')
  const [token, setToken] = useState<'STAR' | 'KHRt' | 'sUSD'>('STAR')

  const predictionTypes = event.sport === 'FOOTBALL'
    ? FOOTBALL_PREDICTION_TYPES
    : KUN_KHMER_PREDICTION_TYPES

  async function submitPrediction() {
    await predictionService.createPrediction(currentUser.id, {
      eventId: event.id,
      type: predictionType,
      prediction,
      stake: parseFloat(stake),
      token
    })

    toast.success('Prediction submitted!')
  }

  return (
    <div className="prediction-form">
      <h3>Make Your Prediction</h3>

      {/* Event Info */}
      <div className="event-info">
        {event.sport === 'FOOTBALL' ? (
          <div>{event.homeTeam} vs {event.awayTeam}</div>
        ) : (
          <div>{event.fighterA} vs {event.fighterB}</div>
        )}
        <div>{formatDate(event.startTime)}</div>
      </div>

      {/* Prediction Type Selector */}
      <div className="prediction-type">
        <label>Prediction Type</label>
        <select value={predictionType} onChange={(e) => setPredictionType(e.target.value)}>
          {Object.keys(predictionTypes).map(type => (
            <option key={type} value={type}>
              {formatPredictionType(type)}
            </option>
          ))}
        </select>
        <div className="payout-info">
          Potential payout: {predictionTypes[predictionType].payout.base}x
        </div>
      </div>

      {/* Prediction Options */}
      <div className="prediction-options">
        {renderPredictionOptions(predictionType, prediction, setPrediction)}
      </div>

      {/* Stake Input */}
      <div className="stake-input">
        <label>Stake Amount</label>
        <div className="input-group">
          <input
            type="number"
            value={stake}
            onChange={(e) => setStake(e.target.value)}
            placeholder="0"
          />
          <select value={token} onChange={(e) => setToken(e.target.value)}>
            <option value="STAR">STAR</option>
            <option value="KHRt">KHRt</option>
            <option value="sUSD">sUSD</option>
          </select>
        </div>
        <div className="balance">
          Available: {userBalance[token]} {token}
        </div>
      </div>

      {/* Potential Winnings */}
      <div className="potential-winnings">
        <div>Potential Winnings:</div>
        <div className="amount">
          {calculatePotentialWinnings(stake, predictionTypes[predictionType].payout.base)} {token}
        </div>
      </div>

      <button onClick={submitPrediction} className="submit-button">
        Submit Prediction
      </button>
    </div>
  )
}
```

**Step 3: Social Features & Leaderboards (2 weeks)**

```typescript
// components/Leaderboard.tsx
export function Leaderboard() {
  const [timeframe, setTimeframe] = useState<'weekly' | 'monthly' | 'alltime'>('weekly')
  const [sport, setSport] = useState<'all' | 'football' | 'kunkhmer'>('all')
  const { data: leaderboard } = useQuery(['leaderboard', timeframe, sport], () =>
    api.getLeaderboard(timeframe, sport)
  )

  return (
    <div className="leaderboard">
      <h2>Top Predictors 🏆</h2>

      {/* Filters */}
      <div className="filters">
        <select value={timeframe} onChange={(e) => setTimeframe(e.target.value)}>
          <option value="weekly">This Week</option>
          <option value="monthly">This Month</option>
          <option value="alltime">All Time</option>
        </select>
        <select value={sport} onChange={(e) => setSport(e.target.value)}>
          <option value="all">All Sports</option>
          <option value="football">Football</option>
          <option value="kunkhmer">Kun Khmer</option>
        </select>
      </div>

      {/* Leaderboard List */}
      <div className="leaderboard-list">
        {leaderboard.map((user, index) => (
          <div key={user.id} className={`leaderboard-item rank-${index + 1}`}>
            <div className="rank">{index + 1}</div>
            <div className="avatar">
              <img src={user.avatar} alt={user.name} />
            </div>
            <div className="info">
              <div className="name">{user.name}</div>
              <div className="stats">
                {user.correctPredictions} correct • {user.winRate}% win rate
              </div>
            </div>
            <div className="winnings">
              +{formatNumber(user.totalWinnings)} STAR
            </div>
          </div>
        ))}
      </div>

      {/* User's Position */}
      {currentUser && (
        <div className="user-position">
          <div>Your Position: #{currentUser.rank}</div>
          <div>Winnings: {currentUser.totalWinnings} STAR</div>
        </div>
      )}
    </div>
  )
}

// components/FriendsChallenges.tsx
export function FriendsChallenges() {
  const [friends] = useFriends()
  const [selectedFriend, setSelectedFriend] = useState<User | null>(null)

  async function challengeFriend(friendId: string, event: Event, stake: number) {
    await api.createChallenge({
      challengerId: currentUser.id,
      opponentId: friendId,
      eventId: event.id,
      stake,
      expiresAt: event.startTime
    })

    toast.success(`Challenge sent to ${friend.name}!`)
  }

  return (
    <div className="friends-challenges">
      <h3>Challenge Your Friends</h3>

      {/* Friend List */}
      <div className="friend-list">
        {friends.map(friend => (
          <div key={friend.id} className="friend-item">
            <img src={friend.avatar} alt={friend.name} />
            <div className="name">{friend.name}</div>
            <div className="stats">
              {friend.winRate}% win rate
            </div>
            <button onClick={() => setSelectedFriend(friend)}>
              Challenge
            </button>
          </div>
        ))}
      </div>

      {/* Active Challenges */}
      <div className="active-challenges">
        <h4>Your Challenges</h4>
        {/* List of pending/active challenges */}
      </div>
    </div>
  )
}

// backend/services/achievements.ts
export class AchievementService {
  achievements = {
    FIRST_PREDICTION: {
      title: 'First Step',
      description: 'Made your first prediction',
      reward: 10, // 10 STAR
      icon: '🎯'
    },
    WIN_STREAK_5: {
      title: 'Hot Streak',
      description: '5 correct predictions in a row',
      reward: 50,
      icon: '🔥'
    },
    WIN_STREAK_10: {
      title: 'Unstoppable',
      description: '10 correct predictions in a row',
      reward: 200,
      icon: '⚡'
    },
    KUN_KHMER_EXPERT: {
      title: 'Kun Khmer Expert',
      description: '20 correct Kun Khmer predictions',
      reward: 100,
      icon: '🥊'
    },
    HIGH_ROLLER: {
      title: 'High Roller',
      description: 'Won a prediction with 1000+ STAR stake',
      reward: 500,
      icon: '💎'
    }
  }

  async checkAndAwardAchievements(userId: string): Promise<Achievement[]> {
    const userStats = await db.userStats.findOne({ userId })
    const awarded: Achievement[] = []

    // Check each achievement
    for (const [key, achievement] of Object.entries(this.achievements)) {
      const hasAchievement = await db.achievements.hasAchievement(userId, key)
      if (!hasAchievement && this.checkCriteria(key, userStats)) {
        // Award achievement
        await db.achievements.create({ userId, achievementKey: key, awardedAt: new Date() })
        await walletService.addBalance(userId, achievement.reward, 'STAR')
        awarded.push({ key, ...achievement })
      }
    }

    return awarded
  }
}
```

**Step 4: Live Predictions (2 weeks)**

```typescript
// Real-time predictions during matches/fights
export class LivePredictionService {
  async createLivePrediction(userId: string, eventId: string, prediction: LivePrediction): Promise<string> {
    // 1. Check if event is currently live
    const event = await db.events.findOne({ id: eventId })
    if (event.status !== 'LIVE') {
      throw new Error('Event is not live')
    }

    // 2. Check if prediction type is allowed in current game state
    const allowedTypes = this.getAllowedPredictionTypes(event)
    if (!allowedTypes.includes(prediction.type)) {
      throw new Error('This prediction type is not available right now')
    }

    // 3. Odds are dynamic based on current score/situation
    const odds = this.calculateLiveOdds(event, prediction.type, prediction.prediction)

    // 4. Create prediction with reduced stake window (1-2 minutes)
    const predId = await db.predictions.create({
      userId,
      eventId,
      type: prediction.type,
      prediction: prediction.prediction,
      stake: prediction.stake,
      token: prediction.token,
      odds,
      isLive: true,
      expiresAt: new Date(Date.now() + 2 * 60 * 1000), // 2 min window
      status: 'PENDING'
    })

    return predId
  }

  // Example live predictions
  getLiveFootballPredictions(match: Match) {
    return {
      NEXT_GOAL: {
        options: ['HOME', 'AWAY', 'NONE'],
        odds: this.calculateOddsBasedOnScore(match.score)
      },
      NEXT_YELLOW_CARD: {
        options: ['HOME', 'AWAY', 'NONE'],
        odds: this.calculateOddsBasedOnFouls(match.fouls)
      },
      NEXT_CORNER: {
        options: ['HOME', 'AWAY'],
        odds: this.calculateOddsBasedOnPossession(match.possession)
      }
    }
  }
}

// components/LiveEventPredictions.tsx
export function LiveEventPredictions({ eventId }: Props) {
  const { data: event } = useLiveEvent(eventId) // WebSocket connection
  const [predictions, setPredictions] = useState<LivePrediction[]>([])

  useEffect(() => {
    // Subscribe to live prediction options
    const socket = io('wss://api.selendra.org')
    socket.on(`event:${eventId}:live-predictions`, (options) => {
      setPredictions(options)
    })

    return () => socket.disconnect()
  }, [eventId])

  return (
    <div className="live-predictions">
      <h3>🔴 Live Predictions</h3>
      <div className="live-status">
        {event.sport === 'FOOTBALL' && (
          <div>
            <div className="score">{event.homeScore} - {event.awayScore}</div>
            <div className="time">{event.minute}'</div>
          </div>
        )}
        {event.sport === 'KUN_KHMER' && (
          <div>
            <div className="round">Round {event.currentRound}</div>
            <div className="time">{event.roundTime}</div>
          </div>
        )}
      </div>

      <div className="live-prediction-options">
        {predictions.map(pred => (
          <LivePredictionCard
            key={pred.type}
            prediction={pred}
            onSubmit={(stake, token) => submitLivePrediction(pred, stake, token)}
          />
        ))}
      </div>
    </div>
  )
}
```

#### Acceptance Criteria

- [ ] Kun Khmer prediction types fully implemented (outcome, method, round, duration, technique)
- [ ] Advanced football predictions work (exact score, first scorer, half-time/full-time, player performance)
- [ ] Multi-token support (STAR, KHRt, sUSD) for all predictions
- [ ] Leaderboards show top predictors (weekly, monthly, all-time)
- [ ] Friends can challenge each other
- [ ] Achievements system awards badges and bonuses
- [ ] Live predictions during matches/fights
- [ ] Push notifications for results and winnings
- [ ] Mobile app updated with all features
- [ ] 10 events/week Kun Khmer schedule ready
- [ ] Data feeds integrated for auto-resolution

#### Configuration

```typescript
// config/predictions.ts
export const PREDICTIONS_CONFIG = {
  minStake: {
    STAR: 1,
    KHRt: 4000, // ~$1
    sUSD: 1
  },
  maxStake: {
    STAR: 10000,
    KHRt: 40000000, // ~$10K
    sUSD: 10000
  },
  predictionWindow: {
    beforeEvent: 5 * 60 * 1000, // 5 minutes before start
    live: 2 * 60 * 1000 // 2 minutes for live predictions
  },
  payoutMultipliers: {
    kunKhmer: {
      OUTCOME: 1.5,
      METHOD: 2.0,
      ROUND: 5.0,
      DURATION: 3.0,
      TECHNIQUE: 4.0
    },
    football: {
      OUTCOME: 1.5,
      EXACT_SCORE: 10.0,
      FIRST_SCORER: 8.0,
      HALF_TIME_FULL_TIME: 6.0,
      TOTAL_GOALS: 2.0,
      PLAYER_PERFORMANCE: 4.0
    }
  }
}
```

---

### TASK 3.2: NFT Ticketing System

**Priority:** ⭐⭐⭐⭐ HIGH - Ready for Kun Khmer launch
**Timeline:** 6-8 weeks
**Dependencies:** Wallet integration, StadiumX integration
**Team:** 1 developer (smart contracts + integration) + Claude Code

#### Purpose
Convert all tickets to ERC-721 NFTs for fraud prevention, collectibility, and secondary market trading.

#### Technical Specs

**Smart Contracts:** ERC-721 standard
**Metadata:** IPFS storage
**Marketplace:** Built-in secondary market

#### Development Instructions

**Step 1: Smart Contracts (3 weeks)**

```solidity
// contracts/TicketNFT.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract TicketNFT is ERC721, ERC721URIStorage, AccessControl {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    struct Ticket {
        string eventId;
        string eventName;
        uint256 eventDate;
        string seat;
        string venue;
        bool used;
        uint256 originalPrice;
    }

    mapping(uint256 => Ticket) public tickets;
    mapping(uint256 => bool) public transferable; // Some tickets may be non-transferable

    event TicketMinted(uint256 indexed tokenId, address indexed owner, string eventId);
    event TicketUsed(uint256 indexed tokenId, uint256 usedAt);
    event TicketTransferred(uint256 indexed tokenId, address indexed from, address indexed to, uint256 price);

    constructor() ERC721("Selendra Event Ticket", "SET") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
    }

    function mintTicket(
        address to,
        string memory eventId,
        string memory eventName,
        uint256 eventDate,
        string memory seat,
        string memory venue,
        uint256 originalPrice,
        bool _transferable,
        string memory tokenURI
    ) public onlyRole(MINTER_ROLE) returns (uint256) {
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();

        _safeMint(to, newTokenId);
        _setTokenURI(newTokenId, tokenURI);

        tickets[newTokenId] = Ticket({
            eventId: eventId,
            eventName: eventName,
            eventDate: eventDate,
            seat: seat,
            venue: venue,
            used: false,
            originalPrice: originalPrice
        });

        transferable[newTokenId] = _transferable;

        emit TicketMinted(newTokenId, to, eventId);

        return newTokenId;
    }

    function useTicket(uint256 tokenId) external {
        require(_exists(tokenId), "Ticket does not exist");
        require(ownerOf(tokenId) == msg.sender || hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not authorized");
        require(!tickets[tokenId].used, "Ticket already used");

        tickets[tokenId].used = true;
        emit TicketUsed(tokenId, block.timestamp);
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId,
        uint256 batchSize
    ) internal virtual override {
        super._beforeTokenTransfer(from, to, tokenId, batchSize);

        // Prevent transfer if not transferable (season passes, etc.)
        if (from != address(0) && to != address(0)) {
            require(transferable[tokenId], "This ticket is non-transferable");
            require(!tickets[tokenId].used, "Cannot transfer used ticket");
        }
    }

    // Override required functions
    function _burn(uint256 tokenId) internal override(ERC721, ERC721URIStorage) {
        super._burn(tokenId);
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, AccessControl, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}

// contracts/TicketMarketplace.sol
contract TicketMarketplace {
    TicketNFT public ticketContract;

    struct Listing {
        uint256 tokenId;
        address seller;
        uint256 price;
        bool active;
    }

    mapping(uint256 => Listing) public listings;

    uint256 public platformFee = 250; // 2.5% (basis points)
    address public feeRecipient;

    event TicketListed(uint256 indexed tokenId, address indexed seller, uint256 price);
    event TicketSold(uint256 indexed tokenId, address indexed seller, address indexed buyer, uint256 price);
    event ListingCancelled(uint256 indexed tokenId);

    constructor(address _ticketContract, address _feeRecipient) {
        ticketContract = TicketNFT(_ticketContract);
        feeRecipient = _feeRecipient;
    }

    function listTicket(uint256 tokenId, uint256 price) external {
        require(ticketContract.ownerOf(tokenId) == msg.sender, "Not ticket owner");
        require(ticketContract.getApproved(tokenId) == address(this) ||
                ticketContract.isApprovedForAll(msg.sender, address(this)), "Marketplace not approved");

        listings[tokenId] = Listing({
            tokenId: tokenId,
            seller: msg.sender,
            price: price,
            active: true
        });

        emit TicketListed(tokenId, msg.sender, price);
    }

    function buyTicket(uint256 tokenId) external payable {
        Listing storage listing = listings[tokenId];
        require(listing.active, "Listing not active");
        require(msg.value >= listing.price, "Insufficient payment");

        // Calculate fees
        uint256 fee = (listing.price * platformFee) / 10000;
        uint256 sellerAmount = listing.price - fee;

        // Transfer ticket
        ticketContract.safeTransferFrom(listing.seller, msg.sender, tokenId);

        // Transfer payments
        payable(listing.seller).transfer(sellerAmount);
        payable(feeRecipient).transfer(fee);

        // Mark listing as inactive
        listing.active = false;

        emit TicketSold(tokenId, listing.seller, msg.sender, listing.price);

        // Refund excess payment
        if (msg.value > listing.price) {
            payable(msg.sender).transfer(msg.value - listing.price);
        }
    }

    function cancelListing(uint256 tokenId) external {
        require(listings[tokenId].seller == msg.sender, "Not seller");
        require(listings[tokenId].active, "Listing not active");

        listings[tokenId].active = false;
        emit ListingCancelled(tokenId);
    }
}
```

**Step 2: Metadata & IPFS Integration (1 week)**

```typescript
// backend/services/ticketMetadata.ts
import { create } from 'ipfs-http-client'
import { readFileSync } from 'fs'

export class TicketMetadataService {
  private ipfs: any

  constructor() {
    this.ipfs = create({ url: 'https://ipfs.infura.io:5001' })
  }

  async generateTicketMetadata(ticket: TicketData): Promise<string> {
    // 1. Create metadata object
    const metadata = {
      name: `${ticket.eventName} - ${ticket.seat}`,
      description: `Official ticket for ${ticket.eventName} at ${ticket.venue}`,
      image: await this.generateTicketImage(ticket),
      attributes: [
        { trait_type: 'Event', value: ticket.eventName },
        { trait_type: 'Date', value: new Date(ticket.eventDate).toLocaleDateString() },
        { trait_type: 'Venue', value: ticket.venue },
        { trait_type: 'Seat', value: ticket.seat },
        { trait_type: 'Sport', value: ticket.sport },
        { trait_type: 'Original Price', value: ticket.price, display_type: 'number' },
        { trait_type: 'Club', value: ticket.club }
      ],
      external_url: `https://selendra.org/tickets/${ticket.id}`
    }

    // 2. Upload to IPFS
    const result = await this.ipfs.add(JSON.stringify(metadata))
    return `ipfs://${result.path}`
  }

  async generateTicketImage(ticket: TicketData): Promise<string> {
    // Generate ticket image with event details, QR code, etc.
    // This could use Canvas API or external service
    const image = await this.createTicketGraphic(ticket)
    const result = await this.ipfs.add(image)
    return `ipfs://${result.path}`
  }

  private async createTicketGraphic(ticket: TicketData): Promise<Buffer> {
    // Use canvas or image manipulation library
    // Include: Event name, date, venue, seat, QR code, branding
    // Return image buffer
  }
}

// backend/services/ticketMinting.ts
export class TicketMintingService {
  private contract: Contract
  private metadataService: TicketMetadataService

  async mintTicketForPurchase(purchase: TicketPurchase): Promise<string> {
    // 1. Generate metadata and upload to IPFS
    const metadataURI = await this.metadataService.generateTicketMetadata({
      eventName: purchase.event.name,
      eventDate: purchase.event.date,
      venue: purchase.event.venue,
      seat: purchase.seat,
      sport: purchase.event.sport,
      price: purchase.price,
      club: purchase.event.club,
      id: purchase.id
    })

    // 2. Mint NFT to user's wallet
    const tx = await this.contract.mintTicket(
      purchase.userWalletAddress,
      purchase.event.id,
      purchase.event.name,
      purchase.event.date,
      purchase.seat,
      purchase.event.venue,
      purchase.price,
      purchase.transferable, // Can be resold?
      metadataURI
    )

    await tx.wait()

    // 3. Return token ID
    const tokenId = await this.getTokenIdFromTx(tx)
    return tokenId
  }
}
```

**Step 3: StadiumX Integration (2 weeks)**

```typescript
// Integrate NFT ticketing into existing StadiumX flow

// services/ticketingIntegration.ts
export class TicketingIntegrationService {
  async handleTicketPurchase(order: TicketOrder): Promise<void> {
    // 1. Process payment (existing Baray integration)
    const payment = await barayService.processPayment(order.amount, order.userId)

    if (payment.status === 'SUCCESS') {
      // 2. Create ticket purchase record
      const purchase = await db.ticketPurchases.create({
        userId: order.userId,
        eventId: order.eventId,
        seat: order.seat,
        price: order.amount,
        transferable: order.ticketType !== 'SEASON_PASS',
        status: 'PENDING_MINT'
      })

      // 3. Mint NFT
      const tokenId = await ticketMintingService.mintTicketForPurchase(purchase)

      // 4. Update purchase record
      await db.ticketPurchases.update(purchase.id, {
        tokenId,
        status: 'MINTED',
        mintedAt: new Date()
      })

      // 5. Notify user
      await notificationService.send(order.userId, {
        title: 'Ticket NFT Minted! 🎫',
        body: `Your ticket for ${order.eventName} is ready in your wallet`
      })
    }
  }

  async verifyTicketAtVenue(tokenId: string, scannerId: string): Promise<VerificationResult> {
    // Venue staff scans QR code on ticket NFT
    // 1. Check if ticket exists and is valid
    const ticket = await ticketContract.tickets(tokenId)
    if (!ticket) {
      return { valid: false, reason: 'Ticket not found' }
    }

    // 2. Check if not already used
    if (ticket.used) {
      return { valid: false, reason: 'Ticket already used' }
    }

    // 3. Check if event date is today
    const today = new Date().toDateString()
    const eventDate = new Date(ticket.eventDate * 1000).toDateString()
    if (today !== eventDate) {
      return { valid: false, reason: 'Invalid date' }
    }

    // 4. Mark as used
    await ticketContract.useTicket(tokenId)

    // 5. Log entry
    await db.venueEntries.create({
      tokenId,
      scannerId,
      entryTime: new Date(),
      eventId: ticket.eventId
    })

    return {
      valid: true,
      ticket: {
        eventName: ticket.eventName,
        seat: ticket.seat,
        holder: await ticketContract.ownerOf(tokenId)
      }
    }
  }
}
```

**Step 4: Secondary Marketplace UI (2 weeks)**

```typescript
// components/TicketMarketplace.tsx
export function TicketMarketplace() {
  const [listings, setListings] = useState<TicketListing[]>([])
  const [filter, setFilter] = useState<{ sport?: string, club?: string, priceRange?: [number, number] }>({})

  useEffect(() => {
    loadListings(filter)
  }, [filter])

  async function loadListings(filter: any) {
    const data = await api.getTicketListings(filter)
    setListings(data)
  }

  return (
    <div className="ticket-marketplace">
      <h2>Ticket Marketplace</h2>

      {/* Filters */}
      <div className="filters">
        <select onChange={(e) => setFilter({ ...filter, sport: e.target.value })}>
          <option value="">All Sports</option>
          <option value="FOOTBALL">Football</option>
          <option value="KUN_KHMER">Kun Khmer</option>
        </select>
        {/* More filters */}
      </div>

      {/* Listings Grid */}
      <div className="listings-grid">
        {listings.map(listing => (
          <TicketCard
            key={listing.tokenId}
            listing={listing}
            onBuy={() => buyTicket(listing.tokenId, listing.price)}
          />
        ))}
      </div>
    </div>
  )
}

// components/MyTickets.tsx
export function MyTickets() {
  const { data: tickets } = useQuery('myTickets', () => api.getMyTickets())
  const [selectedTicket, setSelectedTicket] = useState<Ticket | null>(null)

  async function listTicketForSale(tokenId: string, price: number) {
    // 1. Approve marketplace contract
    await ticketContract.approve(MARKETPLACE_ADDRESS, tokenId)

    // 2. List on marketplace
    await marketplaceContract.listTicket(tokenId, price)

    toast.success('Ticket listed for sale!')
  }

  return (
    <div className="my-tickets">
      <h2>My Tickets</h2>

      <div className="tickets-grid">
        {tickets.map(ticket => (
          <div key={ticket.tokenId} className="ticket-card">
            <img src={ticket.image} alt={ticket.eventName} />
            <div className="ticket-details">
              <h3>{ticket.eventName}</h3>
              <div>{formatDate(ticket.eventDate)}</div>
              <div>Seat: {ticket.seat}</div>
              <div>Venue: {ticket.venue}</div>
              {ticket.used && <div className="used-badge">Used</div>}
            </div>

            {!ticket.used && (
              <div className="ticket-actions">
                <button onClick={() => showQRCode(ticket.tokenId)}>
                  Show QR
                </button>
                {ticket.transferable && (
                  <button onClick={() => setSelectedTicket(ticket)}>
                    Sell
                  </button>
                )}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Sell Modal */}
      {selectedTicket && (
        <SellTicketModal
          ticket={selectedTicket}
          onSubmit={(price) => listTicketForSale(selectedTicket.tokenId, price)}
          onClose={() => setSelectedTicket(null)}
        />
      )}
    </div>
  )
}
```

#### Acceptance Criteria

- [ ] Ticket NFT contract deployed on Selendra
- [ ] Marketplace contract deployed
- [ ] Every StadiumX ticket purchase auto-mints NFT
- [ ] Users can view NFT tickets in wallet
- [ ] QR codes work for venue entry
- [ ] Venue scanning app verifies and marks tickets as used
- [ ] Secondary marketplace allows buying/selling
- [ ] Non-transferable tickets (season passes) cannot be listed
- [ ] Platform takes 2.5% fee on secondary sales
- [ ] Ticket history (all events attended) viewable
- [ ] Mobile app shows ticket collection

#### Configuration

```typescript
// config/tickets.ts
export const TICKETS_CONFIG = {
  contractAddress: '0x...', // After deployment
  marketplaceAddress: '0x...', // After deployment
  ipfsGateway: 'https://ipfs.io/ipfs/',
  platformFee: 250, // 2.5% in basis points
  feeRecipient: '0x...', // Treasury address
  transferableByDefault: true,
  nonTransferableTypes: ['SEASON_PASS', 'VIP_MEMBERSHIP']
}
```

---

### TASK 3.3: Fan Token Platform & Loyalty Program

**Priority:** ⭐⭐⭐⭐ HIGH - Strong engagement and revenue tool
**Timeline:** 8-10 weeks (Fan Tokens 6-8 weeks + Loyalty 2-3 weeks)
**Dependencies:** DEX (for trading), wallet
**Team:** 1 developer (smart contracts) + 1 developer (frontend) + Claude Code

#### Purpose
Launch ERC-20 fan tokens for clubs/teams with governance, exclusive access, revenue sharing, and integrated loyalty rewards program to drive retention.

#### Technical Specs

**Smart Contracts:** ERC-20 + Governance
**Pilot Launch:** 2-3 tokens (September 2025)
**Full Launch:** All 11 CPL clubs + national team (Q1 2026)

#### Development Instructions

**Step 1: Fan Token Smart Contracts (3 weeks)**

```solidity
// contracts/FanToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract FanToken is ERC20, AccessControl, Pausable {
    bytes32 public constant GOVERNANCE_ROLE = keccak256("GOVERNANCE_ROLE");

    string public clubId;
    uint256 public constant MAX_SUPPLY = 100_000_000 * 10**18; // 100M tokens

    // Revenue sharing
    uint256 public totalRevenue;
    uint256 public revenuePerToken;
    mapping(address => uint256) public lastClaimedRevenue;

    // Governance proposals
    struct Proposal {
        string description;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 deadline;
        bool executed;
        mapping(address => bool) hasVoted;
    }

    uint256 public proposalCount;
    mapping(uint256 => Proposal) public proposals;

    event RevenueDistributed(uint256 amount, uint256 revenuePerToken);
    event RevenueClaimed(address indexed holder, uint256 amount);
    event ProposalCreated(uint256 indexed proposalId, string description, uint256 deadline);
    event Voted(uint256 indexed proposalId, address indexed voter, bool support, uint256 votes);

    constructor(
        string memory name,
        string memory symbol,
        string memory _clubId
    ) ERC20(name, symbol) {
        clubId = _clubId;
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(GOVERNANCE_ROLE, msg.sender);
    }

    function mint(address to, uint256 amount) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        _mint(to, amount);
    }

    // Revenue sharing
    function distributeRevenue(uint256 amount) external payable onlyRole(DEFAULT_ADMIN_ROLE) {
        require(msg.value == amount, "Incorrect payment");
        require(totalSupply() > 0, "No tokens minted");

        totalRevenue += amount;
        revenuePerToken += (amount * 1e18) / totalSupply();

        emit RevenueDistributed(amount, revenuePerToken);
    }

    function claimRevenue() external {
        uint256 balance = balanceOf(msg.sender);
        require(balance > 0, "No tokens held");

        uint256 owed = (balance * (revenuePerToken - lastClaimedRevenue[msg.sender])) / 1e18;
        require(owed > 0, "No revenue to claim");

        lastClaimedRevenue[msg.sender] = revenuePerToken;
        payable(msg.sender).transfer(owed);

        emit RevenueClaimed(msg.sender, owed);
    }

    function pendingRevenue(address holder) external view returns (uint256) {
        uint256 balance = balanceOf(holder);
        return (balance * (revenuePerToken - lastClaimedRevenue[holder])) / 1e18;
    }

    // Governance
    function createProposal(string memory description, uint256 votingPeriod)
        external
        onlyRole(GOVERNANCE_ROLE)
        returns (uint256)
    {
        proposalCount++;
        Proposal storage proposal = proposals[proposalCount];
        proposal.description = description;
        proposal.deadline = block.timestamp + votingPeriod;

        emit ProposalCreated(proposalCount, description, proposal.deadline);
        return proposalCount;
    }

    function vote(uint256 proposalId, bool support) external {
        Proposal storage proposal = proposals[proposalId];
        require(block.timestamp < proposal.deadline, "Voting ended");
        require(!proposal.hasVoted[msg.sender], "Already voted");

        uint256 votes = balanceOf(msg.sender);
        require(votes > 0, "No voting power");

        proposal.hasVoted[msg.sender] = true;

        if (support) {
            proposal.votesFor += votes;
        } else {
            proposal.votesAgainst += votes;
        }

        emit Voted(proposalId, msg.sender, support, votes);
    }
}
```

**Step 2: Loyalty Rewards Backend (2-3 weeks)**

```typescript
// backend/services/loyaltyService.ts
export class LoyaltyService {
  // Tiers based on lifetime STAR earned
  tiers = {
    BRONZE: { min: 0, max: 1000, benefits: { discount: 0, priority: false, multiplier: 1.0 } },
    SILVER: { min: 1000, max: 10000, benefits: { discount: 5, priority: true, multiplier: 1.2 } },
    GOLD: { min: 10000, max: 50000, benefits: { discount: 10, priority: true, multiplier: 1.5 } },
    PLATINUM: { min: 50000, max: Infinity, benefits: { discount: 15, priority: true, multiplier: 2.0 } }
  }

  async getUserTier(userId: string): Promise<TierInfo> {
    const stats = await db.userStats.findOne({ userId })
    const lifetimeEarned = stats.lifetimeStarEarned

    for (const [tierName, tier] of Object.entries(this.tiers)) {
      if (lifetimeEarned >= tier.min && lifetimeEarned < tier.max) {
        return { name: tierName, benefits: tier.benefits, progress: lifetimeEarned }
      }
    }

    return { name: 'BRONZE', benefits: this.tiers.BRONZE.benefits, progress: 0 }
  }

  async awardPoints(userId: string, amount: number, reason: string): Promise<void> {
    // Award STAR tokens
    await walletService.addBalance(userId, amount, 'STAR')

    // Update lifetime stats
    await db.userStats.increment(userId, {
      lifetimeStarEarned: amount,
      totalActivities: 1
    })

    // Log transaction
    await db.loyaltyTransactions.create({
      userId,
      amount,
      reason,
      timestamp: new Date()
    })

    // Check for tier upgrade
    const newTier = await this.getUserTier(userId)
    const prevTier = await db.userProfiles.getTier(userId)

    if (newTier.name !== prevTier) {
      await this.handleTierUpgrade(userId, prevTier, newTier.name)
    }
  }

  private async handleTierUpgrade(userId: string, fromTier: string, toTier: string): Promise<void> {
    // Update user tier
    await db.userProfiles.update(userId, { tier: toTier })

    // Send notification
    await notificationService.send(userId, {
      title: `🎉 Tier Upgraded to ${toTier}!`,
      body: `You've unlocked ${this.tiers[toTier].benefits.discount}% discounts and more!`
    })

    // Award bonus
    const bonuses = {
      SILVER: 100,
      GOLD: 500,
      PLATINUM: 2000
    }
    if (bonuses[toTier]) {
      await this.awardPoints(userId, bonuses[toTier], `${toTier} tier upgrade bonus`)
    }
  }

  // Ways to earn points
  async trackEventAttendance(userId: string, eventId: string): Promise<void> {
    const event = await db.events.findOne({ id: eventId })
    const tier = await this.getUserTier(userId)

    // Base points: 10-50 depending on ticket type
    let points = event.ticketType === 'VIP' ? 50 : event.ticketType === 'PREMIUM' ? 30 : 10

    // Apply multiplier based on tier
    points = Math.floor(points * tier.benefits.multiplier)

    await this.awardPoints(userId, points, `Attended ${event.name}`)
  }

  async trackPrediction(userId: string, predictionId: string, won: boolean): Promise<void> {
    if (won) {
      const prediction = await db.predictions.findOne({ id: predictionId })
      const tier = await this.getUserTier(userId)

      // Bonus STAR for winning prediction
      const bonus = Math.floor(prediction.stake * 0.1 * tier.benefits.multiplier)
      await this.awardPoints(userId, bonus, `Prediction bonus for ${prediction.eventName}`)
    }
  }

  async trackPurchase(userId: string, purchaseAmount: number): Promise<void> {
    const tier = await this.getUserTier(userId)

    // Cashback: 5-10% in STAR
    const cashback = Math.floor(purchaseAmount * 0.05 * tier.benefits.multiplier)
    await this.awardPoints(userId, cashback, 'Purchase cashback')
  }

  async trackReferral(referrerId: string, newUserId: string): Promise<void> {
    // Referrer gets 100 STAR
    await this.awardPoints(referrerId, 100, 'Referral bonus')

    // New user gets 50 STAR welcome bonus
    await this.awardPoints(newUserId, 50, 'Welcome bonus')
  }

  async processDailyChallenge(userId: string, challengeType: string): Promise<void> {
    const challenges = {
      PREDICT_TODAY: { points: 5, description: 'Make a prediction today' },
      WIN_PREDICTION: { points: 20, description: 'Win a prediction' },
      ATTEND_EVENT: { points: 30, description: 'Attend an event' },
      REFER_FRIEND: { points: 100, description: 'Refer a friend' }
    }

    if (challenges[challengeType]) {
      await this.awardPoints(userId, challenges[challengeType].points, challenges[challengeType].description)
    }
  }
}
```

**Step 3: Fan Token Launch & Management UI (2-3 weeks)**

```typescript
// components/FanTokenLaunch.tsx
export function FanTokenLaunch({ clubId }: Props) {
  const [allocation, setAllocation] = useState({
    publicSale: 30, // 30%
    team: 20,       // 20%
    treasury: 30,   // 30%
    liquidity: 20   // 20%
  })

  async function launchToken() {
    // 1. Deploy fan token contract
    const contract = await deployFanToken({
      name: `${club.name} Fan Token`,
      symbol: club.tokenSymbol,
      clubId
    })

    // 2. Mint initial supply
    const totalSupply = 100_000_000

    // Public sale allocation
    await contract.mint(PUBLIC_SALE_ADDRESS, totalSupply * allocation.publicSale / 100)

    // Team vesting (2-year vest)
    await contract.mint(TEAM_VESTING_ADDRESS, totalSupply * allocation.team / 100)

    // Treasury
    await contract.mint(TREASURY_ADDRESS, totalSupply * allocation.treasury / 100)

    // DEX liquidity
    await contract.mint(LIQUIDITY_ADDRESS, totalSupply * allocation.liquidity / 100)

    // 3. Create DEX pool
    await dexService.createPool(contract.address, KHRT_ADDRESS, {
      tokenAmount: totalSupply * allocation.liquidity / 100,
      khrtAmount: calculateInitialLiquidity(totalSupply, allocation.liquidity)
    })

    // 4. Start public sale
    await publicSaleService.startSale({
      tokenAddress: contract.address,
      price: calculateTokenPrice(club),
      duration: 7 * 24 * 60 * 60 * 1000 // 7 days
    })
  }

  return (
    <div className="token-launch">
      <h2>Launch {club.name} Fan Token</h2>

      {/* Allocation Chart */}
      <div className="allocation">
        <PieChart data={allocation} />
      </div>

      {/* Token Economics */}
      <div className="economics">
        <div>Total Supply: 100,000,000 {club.tokenSymbol}</div>
        <div>Initial Price: {calculateTokenPrice(club)} KHRt</div>
        <div>Market Cap: ${calculateMarketCap(club)}</div>
      </div>

      <button onClick={launchToken}>Launch Token</button>
    </div>
  )
}

// components/FanTokenDashboard.tsx (For Club Admins)
export function FanTokenDashboard({ clubId }: Props) {
  const { data: token } = useQuery(['fanToken', clubId], () => api.getFanToken(clubId))
  const { data: holders } = useQuery(['tokenHolders', clubId], () => api.getTokenHolders(clubId))

  async function distributeRevenue(amount: number) {
    await fanTokenContract.distributeRevenue(amount, { value: amount })
    toast.success('Revenue distributed to token holders!')
  }

  async function createProposal(description: string, votingPeriod: number) {
    await fanTokenContract.createProposal(description, votingPeriod)
    toast.success('Proposal created!')
  }

  return (
    <div className="fan-token-dashboard">
      <h2>{token.name} Management</h2>

      {/* Token Stats */}
      <div className="stats-grid">
        <div className="stat-card">
          <div className="label">Total Holders</div>
          <div className="value">{holders.length}</div>
        </div>
        <div className="stat-card">
          <div className="label">Market Cap</div>
          <div className="value">${formatNumber(token.marketCap)}</div>
        </div>
        <div className="stat-card">
          <div className="label">Total Revenue Shared</div>
          <div className="value">${formatNumber(token.totalRevenue)}</div>
        </div>
        <div className="stat-card">
          <div className="label">Active Proposals</div>
          <div className="value">{token.activeProposals}</div>
        </div>
      </div>

      {/* Revenue Distribution */}
      <div className="revenue-section">
        <h3>Distribute Revenue to Holders</h3>
        <input type="number" placeholder="Amount in USD" />
        <button onClick={() => distributeRevenue(amount)}>
          Distribute
        </button>
        <div className="info">
          Revenue will be distributed proportionally to all token holders
        </div>
      </div>

      {/* Governance */}
      <div className="governance-section">
        <h3>Create Governance Proposal</h3>
        <textarea placeholder="Proposal description" />
        <select>
          <option value={7 * 24 * 60 * 60}>7 days voting</option>
          <option value={14 * 24 * 60 * 60}>14 days voting</option>
        </select>
        <button onClick={() => createProposal(description, period)}>
          Create Proposal
        </button>
      </div>

      {/* Active Proposals */}
      <div className="proposals-section">
        <h3>Active Proposals</h3>
        {token.proposals.map(proposal => (
          <ProposalCard key={proposal.id} proposal={proposal} />
        ))}
      </div>
    </div>
  )
}

// components/LoyaltyDashboard.tsx (For Users)
export function LoyaltyDashboard() {
  const { data: tier } = useQuery('userTier', () => api.getUserTier())
  const { data: history } = useQuery('loyaltyHistory', () => api.getLoyaltyHistory())
  const { data: challenges } = useQuery('dailyChallenges', () => api.getDailyChallenges())

  return (
    <div className="loyalty-dashboard">
      <h2>Loyalty Rewards</h2>

      {/* Current Tier */}
      <div className="tier-card">
        <div className="tier-badge">
          <span className={`badge ${tier.name.toLowerCase()}`}>{tier.name}</span>
        </div>
        <div className="tier-benefits">
          <div>✓ {tier.benefits.discount}% discount on merchandise</div>
          <div>✓ {tier.benefits.multiplier}x prediction multiplier</div>
          {tier.benefits.priority && <div>✓ Priority ticket access</div>}
        </div>
        <div className="progress">
          <div className="bar" style={{ width: `${tier.progress}%` }} />
          <div className="label">
            {tier.earned} / {tier.nextTierRequirement} STAR to {tier.nextTier}
          </div>
        </div>
      </div>

      {/* Daily Challenges */}
      <div className="challenges">
        <h3>Daily Challenges</h3>
        {challenges.map(challenge => (
          <div key={challenge.id} className={`challenge ${challenge.completed ? 'completed' : ''}`}>
            <div className="icon">{challenge.icon}</div>
            <div className="info">
              <div className="title">{challenge.title}</div>
              <div className="reward">+{challenge.reward} STAR</div>
            </div>
            {challenge.completed ? (
              <div className="completed-badge">✓</div>
            ) : (
              <button>Complete</button>
            )}
          </div>
        ))}
      </div>

      {/* Earning History */}
      <div className="history">
        <h3>Recent Activity</h3>
        {history.map(item => (
          <div key={item.id} className="history-item">
            <div className="date">{formatDate(item.timestamp)}</div>
            <div className="reason">{item.reason}</div>
            <div className="amount">+{item.amount} STAR</div>
          </div>
        ))}
      </div>
    </div>
  )
}
```

#### Acceptance Criteria

**Fan Tokens:**
- [ ] Fan token contracts deployed for pilot clubs (2-3)
- [ ] DEX pools created with initial liquidity
- [ ] Public sale mechanism works
- [ ] Token holders can vote on proposals
- [ ] Revenue distribution works (test with $1K)
- [ ] Club dashboard shows holder stats and revenue
- [ ] Users can buy/sell tokens on DEX
- [ ] Users can stake tokens for boosted benefits

**Loyalty Program:**
- [ ] Four tiers implemented (Bronze, Silver, Gold, Platinum)
- [ ] Points awarded for attendance, predictions, purchases, referrals
- [ ] Tier benefits work (discounts, priority access, multipliers)
- [ ] Daily challenges display and can be completed
- [ ] Loyalty history viewable
- [ ] Automatic tier upgrades with bonuses
- [ ] Push notifications for tier upgrades and rewards

#### Configuration

```typescript
// config/fanTokens.ts
export const FAN_TOKENS_CONFIG = {
  maxSupply: 100_000_000,
  initialAllocation: {
    publicSale: 30,
    team: 20,
    treasury: 30,
    liquidity: 20
  },
  vestingPeriods: {
    team: 24, // 24 months
    advisors: 12 // 12 months
  },
  governanceSettings: {
    minProposalThreshold: 10000, // Need 10K tokens to create proposal
    votingPeriod: 7 * 24 * 60 * 60, // 7 days
    quorum: 10 // 10% of supply must vote
  }
}

// config/loyalty.ts
export const LOYALTY_CONFIG = {
  tiers: {
    BRONZE: { min: 0, discount: 0, multiplier: 1.0 },
    SILVER: { min: 1000, discount: 5, multiplier: 1.2 },
    GOLD: { min: 10000, discount: 10, multiplier: 1.5 },
    PLATINUM: { min: 50000, discount: 15, multiplier: 2.0 }
  },
  earningRates: {
    eventAttendance: { base: 10, vip: 50 },
    predictionBonus: 0.1, // 10% of stake
    purchaseCashback: 0.05, // 5%
    referralBonus: 100,
    welcomeBonus: 50
  },
  dailyChallenges: [
    { id: 'PREDICT_TODAY', reward: 5 },
    { id: 'WIN_PREDICTION', reward: 20 },
    { id: 'ATTEND_EVENT', reward: 30 },
    { id: 'REFER_FRIEND', reward: 100 }
  ]
}
```

---

## PHASE 4: KUN KHMER LAUNCH (Q4 2025 - Months 10-11)

**Goal:** Massive user growth through Kun Khmer, scale merchant network

**Note:** Most infrastructure is ready from Phase 3. Phase 4 focuses on scaling, adding final features, and marketing push for Kun Khmer launch in November 2025.

---

## PHASE 5: SCALE & EXPAND (2026 - Month 12+)

**Goal:** Regional dominance and ecosystem maturation

**Note:** Phase 5 tasks include Fantasy Sports, Lending/Borrowing, Developer Tools, Grants Portal, Launchpad, and other advanced features. These are documented in SELENDRA_ECOSYSTEM_ROADMAP.md with full specifications. Detailed developer tasks can be added as needed based on priorities after successful Phase 3-4 launches.

---

## Notes for Developers

### General Guidelines

1. **Use vibecoding approach**: Iterate fast, don't over-engineer
2. **Test frequently**: On testnet before mainnet
3. **Security first**: Audit all smart contracts, use multi-sig for admin functions
4. **Mobile-first**: Most users on mobile, optimize for that
5. **Khmer language**: Support Khmer + English everywhere
6. **Monitor everything**: Set up alerts for critical metrics
7. **Document as you go**: Update this file with learnings

### Key Principles

- **Custodial wallets** = You handle gas, signing, key management
- **Capital efficiency** matters for small team
- **KHRt is the key differentiator** - focus complexity there
- **Riverbase = fast merchant adoption** - prioritize that plugin
- **Start conservative, scale gradually** - especially for KHRt

### Tech Stack Summary

**Frontend:** React 18, TypeScript, Vite, Tailwind, wagmi/viem
**Mobile:** React Native
**Backend:** Node.js, TypeScript, PostgreSQL, Redis
**Smart Contracts:** Solidity 0.8.x, Hardhat, OpenZeppelin
**Infrastructure:** AWS/GCP, Vercel, SubQuery/The Graph

---

**Document Status:** DRAFT v1.0
**Last Updated:** January 2025
