# KHRt Compliance Framework

## Important Notice

This document outlines the proposed compliance framework for KHRt as we work towards becoming a regulated Digital Asset Service Provider (DASP) under the National Bank of Cambodia's guidelines. All procedures and policies described here are subject to regulatory approval and may be modified based on regulatory requirements.

## Regulatory Approach

### 1. DASP Registration
- Intention to register as a DASP
- Compliance with NBC Prakas
- Regular reporting requirements
- Ongoing regulatory engagement

### 2. Regulatory Framework
- Digital asset regulations
- Payment systems law
- Financial institution law
- Anti-money laundering law

## KYC/AML Procedures

### 1. Customer Due Diligence

#### Individual Accounts
- Basic Information
  - Full name
  - Date of birth
  - Nationality
  - Current address
  - Phone number
  - Email address

- Identity Verification
  - Government ID
  - Proof of address
  - Biometric verification
  - Video verification (if required)

#### Business Accounts
- Company Information
  - Business registration
  - Operating license
  - Company structure
  - Beneficial owners
  - Board members

- Business Verification
  - Corporate documents
  - Director identification
  - Shareholder information
  - Business address

### 2. Risk Assessment

#### Risk Factors
- Geographic location
- Transaction patterns
- Business type
- Account history
- Source of funds

#### Risk Levels
1. Low Risk
   - Individual accounts
   - Small transaction volumes
   - Domestic transactions
   - Clear source of funds

2. Medium Risk
   - Business accounts
   - Regular international transactions
   - Higher transaction volumes
   - Multiple account connections

3. High Risk
   - Complex business structures
   - High-risk jurisdictions
   - Unusual transaction patterns
   - Political exposure

### 3. Transaction Monitoring

```typescript
// Transaction Monitoring System
interface TransactionMonitoring {
    // Risk Scoring
    calculateRiskScore(transaction: Transaction): number;
    
    // Pattern Detection
    detectPattern(transactions: Transaction[]): Pattern[];
    
    // Alert Generation
    generateAlert(risk: Risk): Alert;
    
    // Reporting
    generateReport(timeframe: TimeFrame): Report;
}

// Implementation Example
class KHRtMonitoring implements TransactionMonitoring {
    async monitorTransaction(tx: Transaction) {
        const riskScore = this.calculateRiskScore(tx);
        
        if (riskScore > THRESHOLD) {
            await this.generateAlert({
                type: 'HIGH_RISK_TRANSACTION',
                details: tx,
                score: riskScore
            });
        }
        
        await this.updatePatternAnalysis(tx);
    }
}
```

## Operational Controls

### 1. Transaction Limits

```typescript
interface TransactionLimits {
    // Daily Limits
    individualDailyLimit: number;
    businessDailyLimit: number;
    
    // Monthly Limits
    individualMonthlyLimit: number;
    businessMonthlyLimit: number;
    
    // Transaction Limits
    maxSingleTransaction: number;
    minSingleTransaction: number;
}

// Proposed Initial Limits
const KHRtLimits: TransactionLimits = {
    individualDailyLimit: 40_000_000,    // 10,000 USD
    businessDailyLimit: 200_000_000,     // 50,000 USD
    individualMonthlyLimit: 400_000_000,  // 100,000 USD
    businessMonthlyLimit: 2_000_000_000, // 500,000 USD
    maxSingleTransaction: 20_000_000,    // 5,000 USD
    minSingleTransaction: 4_000          // 1 USD
};
```

### 2. Reporting Requirements

#### Regular Reports
- Daily transaction reports
- Monthly volume reports
- Quarterly compliance reports
- Annual audit reports

#### Incident Reports
- Suspicious transactions
- System outages
- Security incidents
- Compliance breaches

### 3. Record Keeping

```typescript
interface RecordKeeping {
    // Transaction Records
    storeTransaction(tx: Transaction): Promise<void>;
    retrieveTransaction(txId: string): Promise<Transaction>;
    
    // Customer Records
    storeCustomerData(customer: Customer): Promise<void>;
    retrieveCustomerData(customerId: string): Promise<Customer>;
    
    // Compliance Records
    storeComplianceEvent(event: ComplianceEvent): Promise<void>;
    retrieveComplianceEvents(filter: EventFilter): Promise<ComplianceEvent[]>;
}
```

## Risk Management

### 1. Risk Assessment Framework

```typescript
interface RiskAssessment {
    // Customer Risk
    assessCustomerRisk(customer: Customer): RiskScore;
    
    // Transaction Risk
    assessTransactionRisk(tx: Transaction): RiskScore;
    
    // Overall Risk
    calculateOverallRisk(): RiskMetrics;
}

// Risk Categories
enum RiskCategory {
    LOW = 'LOW',
    MEDIUM = 'MEDIUM',
    HIGH = 'HIGH',
    CRITICAL = 'CRITICAL'
}

// Risk Scoring
interface RiskScore {
    category: RiskCategory;
    score: number;
    factors: RiskFactor[];
    recommendations: string[];
}
```

### 2. Incident Response

```typescript
interface IncidentResponse {
    // Incident Handling
    reportIncident(incident: Incident): Promise<void>;
    escalateIncident(incidentId: string): Promise<void>;
    resolveIncident(incidentId: string): Promise<void>;
    
    // Communication
    notifyStakeholders(notification: Notification): Promise<void>;
    generateIncidentReport(incidentId: string): Promise<Report>;
}
```

## Compliance Technology

### 1. Automated Screening

```typescript
interface ComplianceScreening {
    // Sanctions Screening
    checkSanctions(entity: Entity): Promise<ScreeningResult>;
    
    // PEP Screening
    checkPEP(individual: Individual): Promise<ScreeningResult>;
    
    // Adverse Media
    checkAdverseMedia(entity: Entity): Promise<ScreeningResult>;
}
```

### 2. Monitoring Systems

```typescript
interface MonitoringSystems {
    // Real-time Monitoring
    monitorTransactions(tx: Transaction): Promise<MonitoringResult>;
    
    // Pattern Detection
    detectPatterns(data: TransactionData[]): Promise<Pattern[]>;
    
    // Alert Generation
    generateAlert(trigger: AlertTrigger): Promise<Alert>;
}
```

## Training & Updates

### 1. Staff Training
- Initial compliance training
- Regular updates
- Role-specific training
- Assessment and certification

### 2. Policy Updates
- Regular review process
- Regulatory change management
- Policy documentation
- Implementation tracking

## Emergency Procedures

### 1. Suspicious Activity
- Immediate reporting
- Account freezing
- Investigation process
- Authority notification

### 2. System Issues
- Service suspension
- Customer communication
- Problem resolution
- Service restoration

## Future Development

This framework will evolve based on:
- Regulatory guidance
- Market conditions
- Technology advances
- Risk landscape

## Legal Disclaimer

This compliance framework is a proposal and is subject to:
1. Regulatory approval
2. Legal review
3. Risk assessment
4. Operational validation

The final implementation may differ based on regulatory requirements and practical considerations.
