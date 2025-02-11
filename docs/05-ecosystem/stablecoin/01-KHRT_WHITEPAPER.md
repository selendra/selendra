# KHRt: Cambodia's Digital Riel Stablecoin

## Important Notice

This whitepaper outlines the vision and proposed implementation of the KHRt (Khmer Riel Token) stablecoin project. The information presented here represents our current plans and aspirations, which may evolve as we progress with development, regulatory engagement, and market conditions.

**Regulatory Status**: KHRt intends to operate as a regulated Digital Asset Service Provider (DASP) in compliance with the National Bank of Cambodia's (NBC) Prakas and relevant regulations. We are actively engaging with regulatory authorities and working towards obtaining necessary approvals.

**Forward-Looking Statements**: This document contains forward-looking statements about our plans and objectives. These are based on current expectations and projections about future events and may change significantly as the project evolves.

## Executive Summary

KHRt (Khmer Riel Token) is proposed as Cambodia's pioneering stablecoin project, designed to be pegged 1:1 to the Khmer Riel (KHR). Built on Selendra Network, KHRt aims to bridge traditional finance with blockchain technology, with the goal of facilitating digital payments, remittances, and financial inclusion across Cambodia and Southeast Asia.

## Vision & Mission

### Vision
To propose a digital standard for Khmer Riel transactions in the digital economy, aligned with Cambodia's vision for financial digitalization and inclusion.

### Mission
We aim to:
- Facilitate seamless digital transactions in Khmer Riel
- Explore solutions for reducing remittance costs for Cambodian workers
- Support Cambodia's financial digitalization initiatives
- Promote financial inclusion through blockchain technology

### Regulatory Alignment
- Actively pursue DASP licensing under NBC guidelines
- Engage with regulatory authorities for compliance framework
- Implement industry best practices for digital assets
- Contribute to the development of Cambodia's digital asset ecosystem

## Technical Architecture

### 1. Token Standard
```solidity
contract KHRt is ERC20, Pausable, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    
    constructor() ERC20("Khmer Riel Token", "KHRt") {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }
    
    function mint(address to, uint256 amount) external {
        require(hasRole(MINTER_ROLE, msg.sender), "Must have minter role");
        _mint(to, amount);
    }
    
    function burn(address from, uint256 amount) external {
        require(hasRole(BURNER_ROLE, msg.sender), "Must have burner role");
        _burn(from, amount);
    }
}
```

### 2. Reserve System
```solidity
contract KHRtReserve {
    struct ReserveData {
        uint256 totalReserves;
        uint256 reserveRatio;
        mapping(address => bool) reserveBanks;
    }
    
    ReserveData public reserve;
    
    function addReserveBank(address bank) external onlyOwner {
        reserve.reserveBanks[bank] = true;
    }
    
    function updateReserves(uint256 amount, bool increase) external {
        require(reserve.reserveBanks[msg.sender], "Not authorized");
        if (increase) {
            reserve.totalReserves += amount;
        } else {
            require(reserve.totalReserves >= amount, "Insufficient reserves");
            reserve.totalReserves -= amount;
        }
        reserve.reserveRatio = calculateReserveRatio();
    }
}
```

### 3. Price Oracle
```solidity
contract KHRtOracle {
    struct PriceData {
        uint256 price;
        uint256 timestamp;
        address provider;
    }
    
    mapping(bytes32 => PriceData) public prices;
    
    function updatePrice(uint256 newPrice) external onlyOracle {
        bytes32 key = keccak256(abi.encodePacked(block.timestamp));
        prices[key] = PriceData({
            price: newPrice,
            timestamp: block.timestamp,
            provider: msg.sender
        });
        emit PriceUpdated(newPrice, block.timestamp);
    }
}
```

## Economic Model

### 1. Peg Mechanism
- 1:1 peg to Khmer Riel (KHR)
- 100% backed by KHR reserves
- Transparent reserve reporting
- Regular audits

### 2. Minting & Burning
- Licensed financial institutions can mint/burn KHRt
- Automated compliance checks
- Real-time reserve ratio monitoring
- Multi-signature authorization

### 3. Stability Mechanisms
- Algorithmic reserve management
- Dynamic fee adjustment
- Emergency pause functionality
- Market making partnerships

## Use Cases

### 1. Digital Payments
- Retail payments
- E-commerce integration
- Bill payments
- Government services

### 2. Cross-border Remittances
- Worker remittances
- Business payments
- International trade
- Investment flows

### 3. Financial Services
- Lending platforms
- Savings products
- Investment vehicles
- Insurance services

### 4. Government Integration
- Tax payments
- Social benefits distribution
- Public service fees
- Government payroll

## Proposed Regulatory Framework

### 1. Intended Compliance Measures
KHRt intends to implement comprehensive compliance measures aligned with:
- National Bank of Cambodia's DASP framework
- Proposed Anti-Money Laundering (AML) standards
- Planned Know Your Customer (KYC) procedures
- Proposed transaction monitoring systems

*Note: These compliance measures are subject to regulatory approval and may be modified based on regulatory requirements and guidance.*

### 2. Risk Management
```solidity
contract KHRtCompliance {
    struct ComplianceConfig {
        uint256 dailyLimit;
        uint256 transactionLimit;
        bool kycRequired;
        mapping(address => bool) kycApproved;
    }
    
    ComplianceConfig public config;
    
    function setLimits(
        uint256 _dailyLimit,
        uint256 _transactionLimit
    ) external onlyAdmin {
        config.dailyLimit = _dailyLimit;
        config.transactionLimit = _transactionLimit;
    }
    
    function approveKYC(address user) external onlyCompliance {
        config.kycApproved[user] = true;
        emit KYCApproved(user);
    }
    
    function checkCompliance(
        address from,
        address to,
        uint256 amount
    ) external view returns (bool) {
        require(config.kycApproved[from], "KYC required");
        require(amount <= config.transactionLimit, "Exceeds transaction limit");
        require(
            getDailyVolume(from) + amount <= config.dailyLimit,
            "Exceeds daily limit"
        );
        return true;
    }
}
```

### 3. Governance
```solidity
contract KHRtGovernance {
    struct Proposal {
        uint256 id;
        address proposer;
        string description;
        uint256 forVotes;
        uint256 againstVotes;
        bool executed;
        mapping(address => bool) hasVoted;
    }
    
    mapping(uint256 => Proposal) public proposals;
    
    function propose(string memory description) external returns (uint256) {
        require(hasRole(PROPOSER_ROLE, msg.sender), "Not authorized");
        uint256 proposalId = getNextProposalId();
        proposals[proposalId] = Proposal({
            id: proposalId,
            proposer: msg.sender,
            description: description,
            forVotes: 0,
            againstVotes: 0,
            executed: false
        });
        emit ProposalCreated(proposalId, msg.sender, description);
        return proposalId;
    }
    
    function vote(uint256 proposalId, bool support) external {
        require(hasRole(VOTER_ROLE, msg.sender), "Not authorized");
        Proposal storage proposal = proposals[proposalId];
        require(!proposal.hasVoted[msg.sender], "Already voted");
        
        if (support) {
            proposal.forVotes += getVotingPower(msg.sender);
        } else {
            proposal.againstVotes += getVotingPower(msg.sender);
        }
        
        proposal.hasVoted[msg.sender] = true;
        emit Voted(proposalId, msg.sender, support);
    }
}
```

## Integration

### 1. Payment Integration
```typescript
interface KHRtPayment {
    // Payment Methods
    function pay(address to, uint256 amount): Promise<string>;
    function requestPayment(uint256 amount): Promise<string>;
    
    // Subscriptions
    function createSubscription(address merchant): Promise<string>;
    function processSubscription(string subscriptionId): Promise<string>;
    
    // Refunds
    function initiateRefund(string paymentId): Promise<string>;
    function processRefund(string refundId): Promise<string>;
}
```

### 2. Banking Integration
```typescript
interface KHRtBanking {
    // Bank Transfers
    function deposit(string bankAccount, uint256 amount): Promise<string>;
    function withdraw(string bankAccount, uint256 amount): Promise<string>;
    
    // Account Management
    function linkBankAccount(string bankAccount): Promise<boolean>;
    function unlinkBankAccount(string bankAccount): Promise<boolean>;
    
    // Statements
    function getTransactionHistory(): Promise<Transaction[]>;
    function generateStatement(DateRange range): Promise<Statement>;
}
```

## Roadmap

### Phase 1: Foundation (2025 Q1-Q2)
- Launch KHRt on Selendra mainnet
- Establish banking partnerships
- Implement core compliance framework
- Begin pilot program

### Phase 2: Expansion (2025 Q3-Q4)
- Integrate with major payment providers
- Launch cross-border remittance services
- Expand merchant network
- Develop government partnerships

### Phase 3: Innovation (2026+)
- Launch DeFi services
- Implement advanced features
- Expand regional presence
- Enhance interoperability

## Conclusion

KHRt represents an innovative proposal for Cambodia's financial digitalization journey. As we work towards becoming a regulated DASP and developing a stable, compliant digital representation of the Khmer Riel, we remain committed to supporting Cambodia's economic development in the digital age.

## Legal Disclaimer

1. **Development Status**: This whitepaper describes a project under development. Features, timelines, and technical specifications are subject to change.

2. **Regulatory Compliance**: KHRt is actively working towards regulatory compliance and DASP status. No services will be offered until appropriate regulatory approvals are obtained.

3. **Forward-Looking Statements**: This document contains forward-looking statements and projections. Actual results may differ materially from these projections due to various factors including regulatory requirements, technical challenges, or market conditions.

4. **No Investment Advice**: This whitepaper is for informational purposes only and does not constitute financial or investment advice. Any financial decisions should be made after careful consideration and consultation with appropriate professionals.

5. **Living Document**: This whitepaper is a living document that will be updated as the project evolves, regulatory framework develops, and new opportunities emerge.
