# Real World Asset (RWA) Tokenization on Selendra

## Overview

Selendra's RWA tokenization platform enables the creation, management, and trading of tokenized real-world assets including:
- Company Shares
- Government Bonds
- Real Estate
- Commodities
- Private Equity
- Infrastructure Projects

## Smart Contract Architecture

### 1. Asset Token Standard
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract RWAToken is ERC1155, AccessControl, Pausable {
    bytes32 public constant ISSUER_ROLE = keccak256("ISSUER_ROLE");
    bytes32 public constant COMPLIANCE_ROLE = keccak256("COMPLIANCE_ROLE");
    
    struct Asset {
        string assetType;      // "SHARE", "BOND", "REAL_ESTATE"
        string jurisdiction;   // Country code
        uint256 totalSupply;
        string metadata;       // IPFS hash of detailed documentation
        bool frozen;
    }
    
    mapping(uint256 => Asset) public assets;
    mapping(address => bool) public kycApproved;
    
    event AssetCreated(
        uint256 indexed tokenId,
        string assetType,
        string jurisdiction,
        uint256 totalSupply
    );
    
    constructor() ERC1155("https://assets.selendra.com/metadata/{id}.json") {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }
    
    function createAsset(
        uint256 tokenId,
        string memory assetType,
        string memory jurisdiction,
        uint256 totalSupply,
        string memory metadata
    ) external onlyRole(ISSUER_ROLE) {
        require(assets[tokenId].totalSupply == 0, "Asset already exists");
        
        assets[tokenId] = Asset({
            assetType: assetType,
            jurisdiction: jurisdiction,
            totalSupply: totalSupply,
            metadata: metadata,
            frozen: false
        });
        
        _mint(msg.sender, tokenId, totalSupply, "");
        
        emit AssetCreated(
            tokenId,
            assetType,
            jurisdiction,
            totalSupply
        );
    }
    
    function freezeAsset(uint256 tokenId) 
        external 
        onlyRole(COMPLIANCE_ROLE) 
    {
        assets[tokenId].frozen = true;
    }
    
    function unfreezeAsset(uint256 tokenId) 
        external 
        onlyRole(COMPLIANCE_ROLE) 
    {
        assets[tokenId].frozen = false;
    }
    
    function setKYCApproval(address investor, bool approved) 
        external 
        onlyRole(COMPLIANCE_ROLE) 
    {
        kycApproved[investor] = approved;
    }
    
    function _beforeTokenTransfer(
        address operator,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory amounts,
        bytes memory data
    ) internal virtual override {
        super._beforeTokenTransfer(operator, from, to, ids, amounts, data);
        
        require(!paused(), "Transfers paused");
        
        if (from != address(0) && to != address(0)) {
            require(kycApproved[from] && kycApproved[to], "KYC not approved");
            
            for (uint256 i = 0; i < ids.length; i++) {
                require(!assets[ids[i]].frozen, "Asset frozen");
            }
        }
    }
}
```

### 2. Asset Registry
```solidity
contract RWARegistry {
    struct AssetDetails {
        address tokenContract;
        uint256 tokenId;
        string assetType;
        string jurisdiction;
        address issuer;
        uint256 timestamp;
        bool verified;
    }
    
    mapping(bytes32 => AssetDetails) public registry;
    
    event AssetRegistered(
        bytes32 indexed assetId,
        address tokenContract,
        uint256 tokenId,
        string assetType
    );
    
    function registerAsset(
        address tokenContract,
        uint256 tokenId,
        string memory assetType,
        string memory jurisdiction
    ) external returns (bytes32) {
        bytes32 assetId = keccak256(
            abi.encodePacked(
                tokenContract,
                tokenId,
                block.timestamp
            )
        );
        
        registry[assetId] = AssetDetails({
            tokenContract: tokenContract,
            tokenId: tokenId,
            assetType: assetType,
            jurisdiction: jurisdiction,
            issuer: msg.sender,
            timestamp: block.timestamp,
            verified: false
        });
        
        emit AssetRegistered(
            assetId,
            tokenContract,
            tokenId,
            assetType
        );
        
        return assetId;
    }
}
```

### 3. Compliance Manager
```solidity
contract ComplianceManager {
    struct ComplianceRequirement {
        bool kycRequired;
        bool accreditationRequired;
        uint256 holdingPeriod;
        uint256 minimumInvestment;
    }
    
    mapping(bytes32 => ComplianceRequirement) public requirements;
    mapping(address => mapping(string => bool)) public investorAccreditation;
    
    function setComplianceRequirements(
        bytes32 assetId,
        ComplianceRequirement memory requirement
    ) external onlyAdmin {
        requirements[assetId] = requirement;
    }
    
    function checkCompliance(
        bytes32 assetId,
        address investor,
        uint256 amount
    ) external view returns (bool) {
        ComplianceRequirement memory req = requirements[assetId];
        
        if (req.kycRequired && !isKYCApproved(investor)) {
            return false;
        }
        
        if (req.accreditationRequired && 
            !investorAccreditation[investor]["ACCREDITED"]) {
            return false;
        }
        
        if (amount < req.minimumInvestment) {
            return false;
        }
        
        return true;
    }
}
```

## Asset Types Implementation

### 1. Company Shares
```solidity
contract ShareToken is RWAToken {
    struct ShareDetails {
        string companyName;
        string shareClass;
        uint256 parValue;
        uint256 dividendRights;
        uint256 votingRights;
    }
    
    mapping(uint256 => ShareDetails) public shareDetails;
    
    function issueShares(
        uint256 tokenId,
        string memory companyName,
        string memory shareClass,
        uint256 totalSupply,
        ShareDetails memory details
    ) external onlyRole(ISSUER_ROLE) {
        createAsset(
            tokenId,
            "SHARE",
            "KH",  // Cambodia
            totalSupply,
            ""
        );
        
        shareDetails[tokenId] = details;
    }
    
    function declareDividend(
        uint256 tokenId,
        uint256 amountPerShare
    ) external onlyRole(ISSUER_ROLE) {
        // Dividend distribution logic
    }
}
```

### 2. Government Bonds
```solidity
contract BondToken is RWAToken {
    struct BondDetails {
        string issuer;
        uint256 maturityDate;
        uint256 couponRate;
        uint256 couponInterval;
        uint256 parValue;
    }
    
    mapping(uint256 => BondDetails) public bondDetails;
    
    function issueBond(
        uint256 tokenId,
        string memory issuer,
        uint256 totalSupply,
        BondDetails memory details
    ) external onlyRole(ISSUER_ROLE) {
        createAsset(
            tokenId,
            "BOND",
            "KH",  // Cambodia
            totalSupply,
            ""
        );
        
        bondDetails[tokenId] = details;
    }
    
    function payCoupon(uint256 tokenId) 
        external 
        onlyRole(ISSUER_ROLE) 
    {
        // Coupon payment logic
    }
}
```

### 3. Real Estate
```solidity
contract RealEstateToken is RWAToken {
    struct PropertyDetails {
        string location;
        uint256 squareMeters;
        string propertyType;
        string titleDeed;
        uint256 rentalYield;
    }
    
    mapping(uint256 => PropertyDetails) public propertyDetails;
    
    function tokenizeProperty(
        uint256 tokenId,
        uint256 totalSupply,
        PropertyDetails memory details
    ) external onlyRole(ISSUER_ROLE) {
        createAsset(
            tokenId,
            "REAL_ESTATE",
            "KH",  // Cambodia
            totalSupply,
            ""
        );
        
        propertyDetails[tokenId] = details;
    }
    
    function distributeRent(
        uint256 tokenId,
        uint256 amountPerToken
    ) external onlyRole(ISSUER_ROLE) {
        // Rental distribution logic
    }
}
```

## Trading Platform

### 1. Order Book
```solidity
contract RWAExchange {
    struct Order {
        address trader;
        uint256 tokenId;
        uint256 amount;
        uint256 price;
        bool isBuyOrder;
        uint256 timestamp;
    }
    
    mapping(bytes32 => Order) public orders;
    mapping(uint256 => bytes32[]) public orderBook;
    
    function placeOrder(
        uint256 tokenId,
        uint256 amount,
        uint256 price,
        bool isBuyOrder
    ) external returns (bytes32) {
        bytes32 orderId = keccak256(
            abi.encodePacked(
                msg.sender,
                tokenId,
                amount,
                price,
                block.timestamp
            )
        );
        
        orders[orderId] = Order({
            trader: msg.sender,
            tokenId: tokenId,
            amount: amount,
            price: price,
            isBuyOrder: isBuyOrder,
            timestamp: block.timestamp
        });
        
        orderBook[tokenId].push(orderId);
        
        matchOrders(tokenId);
        
        return orderId;
    }
    
    function matchOrders(uint256 tokenId) internal {
        // Order matching logic
    }
}
```

### 2. Settlement
```solidity
contract RWASettlement {
    struct Settlement {
        bytes32 orderId;
        address buyer;
        address seller;
        uint256 tokenId;
        uint256 amount;
        uint256 price;
        uint256 timestamp;
    }
    
    mapping(bytes32 => Settlement) public settlements;
    
    function settleTransaction(
        bytes32 orderId,
        address buyer,
        address seller,
        uint256 tokenId,
        uint256 amount,
        uint256 price
    ) external {
        // Settlement logic
    }
}
```

## Regulatory Compliance

### 1. KYC/AML Integration
```solidity
contract KYCManager {
    struct KYCData {
        bool verified;
        uint256 verificationLevel;
        uint256 expiryDate;
        string jurisdiction;
    }
    
    mapping(address => KYCData) public kycData;
    
    function updateKYC(
        address investor,
        KYCData memory data
    ) external onlyVerifier {
        kycData[investor] = data;
    }
    
    function checkKYC(
        address investor,
        uint256 requiredLevel
    ) external view returns (bool) {
        KYCData memory data = kycData[investor];
        return data.verified && 
               data.verificationLevel >= requiredLevel &&
               data.expiryDate > block.timestamp;
    }
}
```

### 2. Reporting System
```solidity
contract RWAReporting {
    struct Transaction {
        address from;
        address to;
        uint256 tokenId;
        uint256 amount;
        uint256 price;
        uint256 timestamp;
    }
    
    mapping(uint256 => Transaction[]) public transactions;
    
    function recordTransaction(
        address from,
        address to,
        uint256 tokenId,
        uint256 amount,
        uint256 price
    ) external {
        transactions[tokenId].push(Transaction({
            from: from,
            to: to,
            tokenId: tokenId,
            amount: amount,
            price: price,
            timestamp: block.timestamp
        }));
    }
    
    function generateReport(
        uint256 tokenId,
        uint256 startTime,
        uint256 endTime
    ) external view returns (Transaction[] memory) {
        // Report generation logic
    }
}
```

## Integration APIs

### 1. Asset Management API
```typescript
interface AssetManagementAPI {
    createAsset(params: {
        assetType: string;
        details: AssetDetails;
        compliance: ComplianceRequirement;
    }): Promise<string>;
    
    getAssetDetails(assetId: string): Promise<AssetDetails>;
    
    updateAsset(
        assetId: string,
        updates: Partial<AssetDetails>
    ): Promise<void>;
    
    getInvestorHoldings(
        investor: string
    ): Promise<AssetHolding[]>;
}
```

### 2. Trading API
```typescript
interface TradingAPI {
    placeOrder(params: {
        assetId: string;
        amount: string;
        price: string;
        orderType: 'BUY' | 'SELL';
    }): Promise<string>;
    
    cancelOrder(orderId: string): Promise<void>;
    
    getOrderBook(assetId: string): Promise<Order[]>;
    
    getTradeHistory(assetId: string): Promise<Trade[]>;
}
```

## Development Timeline

### Q2 2025
- Launch company shares tokenization
- Implement basic trading functionality
- Deploy KYC/AML system

### Q3 2025
- Add government bond tokenization
- Enhance trading features
- Implement dividend distribution

### Q4 2025
- Launch real estate tokenization
- Add advanced reporting
- Implement cross-border trading

### Q1 2026
- Add more asset types
- Enhance regulatory compliance
- Scale platform capabilities
