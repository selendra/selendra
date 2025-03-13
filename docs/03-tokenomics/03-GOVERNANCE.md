# Token Governance

## Overview

Selendra's governance system enables:
- On-chain voting
- Proposal creation
- Parameter updates
- Treasury management
- Protocol upgrades

## Governance System

### 1. Proposal System
```solidity
contract ProposalSystem {
    struct Proposal {
        uint256 id;
        address proposer;
        bytes32 description;
        uint256 startBlock;
        uint256 endBlock;
        ProposalState state;
        mapping(address => Vote) votes;
        uint256 forVotes;
        uint256 againstVotes;
    }
    
    mapping(uint256 => Proposal) public proposals;
    uint256 public proposalCount;
    
    function createProposal(
        bytes32 description,
        bytes[] calldata calls
    ) external returns (uint256) {
        require(
            token.balanceOf(msg.sender) >= proposalThreshold,
            "Insufficient tokens"
        );
        
        uint256 proposalId = ++proposalCount;
        Proposal storage proposal = proposals[proposalId];
        
        proposal.id = proposalId;
        proposal.proposer = msg.sender;
        proposal.description = description;
        proposal.startBlock = block.number + votingDelay;
        proposal.endBlock = proposal.startBlock + votingPeriod;
        proposal.state = ProposalState.Pending;
        
        emit ProposalCreated(proposalId, msg.sender, description);
        
        return proposalId;
    }
    
    function castVote(
        uint256 proposalId,
        bool support
    ) external {
        Proposal storage proposal = proposals[proposalId];
        require(
            proposal.state == ProposalState.Active,
            "Proposal not active"
        );
        
        uint256 votes = token.getPriorVotes(
            msg.sender,
            proposal.startBlock
        );
        
        require(votes > 0, "No voting power");
        
        if (support) {
            proposal.forVotes += votes;
        } else {
            proposal.againstVotes += votes;
        }
        
        proposal.votes[msg.sender] = Vote({
            support: support,
            votes: votes
        });
        
        emit VoteCast(msg.sender, proposalId, support, votes);
    }
}
```

### 2. Parameter Management
```solidity
contract ParameterManager {
    struct Parameter {
        bytes32 name;
        bytes value;
        uint256 lastUpdate;
        address lastUpdater;
    }
    
    mapping(bytes32 => Parameter) public parameters;
    
    function setParameter(
        bytes32 name,
        bytes calldata value
    ) external onlyGovernance {
        parameters[name] = Parameter({
            name: name,
            value: value,
            lastUpdate: block.timestamp,
            lastUpdater: msg.sender
        });
        
        emit ParameterUpdated(name, value);
    }
    
    function getParameter(
        bytes32 name
    ) external view returns (bytes memory) {
        return parameters[name].value;
    }
}
```

## Treasury Management

### 1. Treasury System
```solidity
contract Treasury {
    struct Spending {
        uint256 id;
        address beneficiary;
        uint256 amount;
        bytes32 description;
        uint256 unlockTime;
        bool executed;
    }
    
    mapping(uint256 => Spending) public spendings;
    uint256 public spendingCount;
    
    function proposeSpending(
        address beneficiary,
        uint256 amount,
        bytes32 description,
        uint256 unlockTime
    ) external onlyGovernance returns (uint256) {
        uint256 spendingId = ++spendingCount;
        
        spendings[spendingId] = Spending({
            id: spendingId,
            beneficiary: beneficiary,
            amount: amount,
            description: description,
            unlockTime: unlockTime,
            executed: false
        });
        
        emit SpendingProposed(spendingId, beneficiary, amount);
        
        return spendingId;
    }
    
    function executeSpending(
        uint256 spendingId
    ) external onlyGovernance {
        Spending storage spending = spendings[spendingId];
        require(!spending.executed, "Already executed");
        require(
            block.timestamp >= spending.unlockTime,
            "Not unlocked"
        );
        
        spending.executed = true;
        token.transfer(spending.beneficiary, spending.amount);
        
        emit SpendingExecuted(spendingId);
    }
}
```

### 2. Fund Management
```solidity
contract FundManager {
    struct Fund {
        bytes32 name;
        uint256 balance;
        address manager;
        FundStrategy strategy;
    }
    
    mapping(bytes32 => Fund) public funds;
    
    function createFund(
        bytes32 name,
        address manager,
        FundStrategy strategy
    ) external onlyGovernance {
        funds[name] = Fund({
            name: name,
            balance: 0,
            manager: manager,
            strategy: strategy
        });
        
        emit FundCreated(name, manager);
    }
    
    function allocateFunds(
        bytes32 fundName,
        uint256 amount
    ) external onlyGovernance {
        Fund storage fund = funds[fundName];
        
        token.transfer(address(fund.strategy), amount);
        fund.balance += amount;
        
        emit FundsAllocated(fundName, amount);
    }
}
```

## Protocol Upgrades

### 1. Upgrade System
```solidity
contract UpgradeSystem {
    struct Upgrade {
        uint256 id;
        address newImplementation;
        bytes32 description;
        uint256 scheduledTime;
        bool executed;
    }
    
    mapping(uint256 => Upgrade) public upgrades;
    uint256 public upgradeCount;
    
    function proposeUpgrade(
        address newImplementation,
        bytes32 description
    ) external onlyGovernance returns (uint256) {
        uint256 upgradeId = ++upgradeCount;
        
        upgrades[upgradeId] = Upgrade({
            id: upgradeId,
            newImplementation: newImplementation,
            description: description,
            scheduledTime: block.timestamp + upgradeDelay,
            executed: false
        });
        
        emit UpgradeProposed(upgradeId, newImplementation);
        
        return upgradeId;
    }
    
    function executeUpgrade(
        uint256 upgradeId
    ) external onlyGovernance {
        Upgrade storage upgrade = upgrades[upgradeId];
        require(!upgrade.executed, "Already executed");
        require(
            block.timestamp >= upgrade.scheduledTime,
            "Too early"
        );
        
        upgrade.executed = true;
        _upgrade(upgrade.newImplementation);
        
        emit UpgradeExecuted(upgradeId);
    }
}
```

### 2. Version Management
```solidity
contract VersionManager {
    struct Version {
        uint256 major;
        uint256 minor;
        uint256 patch;
        bytes32 description;
        uint256 deployTime;
    }
    
    Version[] public versions;
    
    function addVersion(
        uint256 major,
        uint256 minor,
        uint256 patch,
        bytes32 description
    ) external onlyGovernance {
        versions.push(Version({
            major: major,
            minor: minor,
            patch: patch,
            description: description,
            deployTime: block.timestamp
        }));
        
        emit VersionAdded(major, minor, patch);
    }
    
    function getCurrentVersion()
        external
        view
        returns (Version memory)
    {
        return versions[versions.length - 1];
    }
}
```

## Analytics

### 1. Governance Analytics
```typescript
interface GovernanceAnalytics {
    // Proposal Analytics
    getProposalStats(): Promise<ProposalStats>;
    getVotingHistory(): Promise<VotingHistory>;
    
    // Treasury Analytics
    getTreasuryBalance(): Promise<Balance>;
    getSpendingHistory(): Promise<SpendingHistory>;
    
    // Upgrade Analytics
    getUpgradeHistory(): Promise<UpgradeHistory>;
    getVersionHistory(): Promise<VersionHistory>;
}
```

### 2. Participation Analytics
```typescript
interface ParticipationAnalytics {
    // Voter Analytics
    getVoterStats(): Promise<VoterStats>;
    getVoterParticipation(): Promise<ParticipationStats>;
    
    // Proposal Analytics
    getProposalSuccess(): Promise<SuccessStats>;
    getProposalTypes(): Promise<TypeStats>;
    
    // Custom Analytics
    getCustomMetrics(params: MetricsParams): Promise<CustomMetrics>;
    exportData(format: ExportFormat): Promise<ExportedData>;
}
```

## Integration

### 1. SDK Integration
```typescript
interface GovernanceSDK {
    // Proposal Operations
    createProposal(params: ProposalParams): Promise<string>;
    castVote(proposalId: string, support: boolean): Promise<string>;
    
    // Treasury Operations
    proposeSpending(params: SpendingParams): Promise<string>;
    executeSpending(spendingId: string): Promise<string>;
    
    // Upgrade Operations
    proposeUpgrade(params: UpgradeParams): Promise<string>;
    executeUpgrade(upgradeId: string): Promise<string>;
}
```

### 2. Event Monitoring
```typescript
interface EventMonitor {
    // Event Subscriptions
    onProposal(callback: ProposalCallback): Subscription;
    onVote(callback: VoteCallback): Subscription;
    onSpending(callback: SpendingCallback): Subscription;
    onUpgrade(callback: UpgradeCallback): Subscription;
    
    // Analytics
    getEventStats(): Promise<EventStats>;
    getParticipationStats(): Promise<ParticipationStats>;
}
```
