# Selendra v4 Agent Prompt Guide

## Overview

This guide provides templates and examples for writing effective prompts to activate and coordinate the 11 specialized agents building Selendra Network v4. Each prompt should be specific, reference whitepaper requirements, define integration points, and set clear success criteria.

## General Agent Activation Pattern

```
@[agent-name] I need you to [specific task] for Selendra v4. 

Context: [brief context from whitepaper/requirements]
Deliverable: [specific outcome expected]
Integration: [which other agents you'll need to coordinate with]
Success Criteria: [how we measure success]
```

## Phase-Based Agent Activation

### Phase 0: Network Foundation

#### Governance Democracy Architect
```
@governance-democracy-architect We need to implement the permissionless network transition outlined in the whitepaper. 

Context: Selendra v3 currently has limited validator participation. We need to transition to open validator selection while maintaining enterprise features and security.

Deliverable: Complete democracy pallet implementation with council, treasury, and validator selection governance

Integration: Coordinate with @consensus-runtime-architect for validator mechanics and @ops-infrastructure-manager for deployment

Success Criteria: Network successfully opens to new validators, community controls governance, all business features preserved
```

#### Consensus Runtime Architect
```
@consensus-runtime-architect Support the permissionless transition by modifying validator selection from MAX_NOMINATORS=1 to 16 and implementing performance-based ranking.

Context: Current v3 network has validator approval requirements that need removal for permissionless operation

Deliverable: Updated committee management pallet supporting open validator participation with automatic performance scoring

Integration: Work with @governance-democracy-architect for governance integration and @network-p2p-specialist for validator communication

Success Criteria: Support up to 1,000+ validators, maintain 99.9% uptime, seamless validator onboarding
```

#### Operations Infrastructure Manager
```
@ops-infrastructure-manager Prepare infrastructure for permissionless validator onboarding and network monitoring during the transition.

Context: Network will scale from 50+ to potentially 1,000+ validators across global geography

Deliverable: Automated validator onboarding, comprehensive monitoring, production deployment coordination

Integration: Support @governance-democracy-architect governance execution and @consensus-runtime-architect validator management

Success Criteria: 95%+ validator onboarding success rate, <15 minute incident response, 99.9% network uptime
```

### Phase 1: Smart Contract Accounts

#### Account Abstraction Architect
```
@account-abstraction-architect Implement the core account abstraction system described in Section 2 of the whitepaper - make every user account a smart contract with social recovery.

Context: This is Selendra's key innovation. Users should never lose access to accounts, apps should pay fees, and blockchain should feel like web2.

Deliverable: Complete account abstraction pallet with guardian recovery, session keys, and paymaster framework

Integration: Coordinate with @evm-integration-specialist and @wasm-runtime-specialist for runtime compatibility, @testing-qa-specialist for security validation

Success Criteria: 100% accounts become smart contracts, 99%+ recovery success rate, gasless transactions enabled
```

#### EVM Integration Specialist
```
@evm-integration-specialist Ensure 100% Ethereum compatibility while supporting account abstraction and cross-runtime communication with WASM.

Context: Existing Ethereum developers should migrate seamlessly. Support 36M gas per block, MetaMask compatibility, zero code changes required.

Deliverable: Optimized Frontier integration, EVM account abstraction support, EVM↔WASM bridge protocols

Integration: Collaborate with @wasm-runtime-specialist on cross-runtime APIs and @account-abstraction-architect on EVM smart contract accounts

Success Criteria: Zero migration issues, 50+ Ethereum projects migrated, full tooling compatibility
```

#### WASM Runtime Specialist
```
@wasm-runtime-specialist Build high-performance WASM runtime optimized for real-world applications like gaming, DeFi calculations, and identity verification.

Context: WASM should provide 5-10x performance advantage over EVM for computationally intensive applications

Deliverable: Native WASM execution engine, WASM account abstraction implementation, WASM-optimized parallel processing

Integration: Cross-runtime communication with @evm-integration-specialist, parallel optimization with @parallel-execution-architect

Success Criteria: 5-10x EVM performance, native account abstraction <1ms validation, 20+ WASM applications
```

### Phase 2: Parallel Processing

#### Parallel Execution Architect
```
@parallel-execution-architect Implement the parallel processing system from Section 3 to achieve 3-5x throughput improvement targeting 1,200-4,000 TPS.

Context: Most transactions don't conflict (Sophea→Pisach, Sreypov buying coffee, Dara updating credentials). We need to process these simultaneously.

Deliverable: Transaction conflict detection, multi-threaded execution engine, state partitioning with rollback mechanisms

Integration: Critical coordination with @consensus-runtime-architect for block validation and @network-p2p-specialist for parallel transaction propagation

Success Criteria: 1,200+ TPS sustained, <1ms additional latency, 100% correctness in conflict detection
```

#### Network P2P Specialist
```
@network-p2p-specialist Optimize network protocols to support parallel transaction propagation and increased validator count.

Context: Parallel processing requires coordinated transaction distribution across up to 1,000+ validators

Deliverable: Enhanced P2P protocols, parallel transaction propagation, optimized validator communication

Integration: Support @parallel-execution-architect transaction distribution and @consensus-runtime-architect validator coordination

Success Criteria: <100ms transaction propagation, support 1,000+ validators, effective DoS protection
```

### Phase 3: Production Excellence

#### Developer Experience Architect
```
@developer-experience-architect Build SDK and tooling that makes Selendra's account abstraction and dual runtime accessible to developers.

Context: Developers should easily build loyalty programs, asset tokenization, identity systems without blockchain complexity

Deliverable: Comprehensive SDK, developer tools for EVM/WASM, integration examples, documentation

Integration: APIs from @account-abstraction-architect, tooling for both @evm-integration-specialist and @wasm-runtime-specialist

Success Criteria: 500+ developers onboarded, satisfaction >4.5/5, 50+ production applications
```

#### Bridge Interoperability Architect
```
@bridge-interoperability-architect Implement secure cross-chain bridges to Ethereum and other major networks as described in Section 8.5.

Context: Users need to bring existing assets to Selendra for lower fees while maintaining interoperability

Deliverable: Multi-signature bridge infrastructure, asset transfer protocols, cross-chain communication

Integration: Bridge security validation with @testing-qa-specialist, smart contract account integration with @account-abstraction-architect

Success Criteria: Bridges to 5+ networks, >$10M secure transfers, <5 minute confirmation times
```

#### Testing QA Specialist
```
@testing-qa-specialist Create comprehensive testing framework for all v4 components with focus on security, performance, and migration validation.

Context: We're building critical financial infrastructure. Zero tolerance for bugs in production. Need v3→v4 migration testing.

Deliverable: Complete test coverage, automated performance benchmarking, security audit coordination, migration testing

Integration: Test deliverables from all other agents, coordinate security audits across all components

Success Criteria: 90%+ code coverage, zero critical bugs in production, successful v3→v4 migration validation
```

## Multi-Agent Coordination Prompts

### Cross-Runtime Integration
```
@account-abstraction-architect @evm-integration-specialist @wasm-runtime-specialist 

We need native account abstraction working seamlessly across both EVM and WASM runtimes. This is critical for user experience.

Requirements:
- Consistent API across both runtimes
- Guardian recovery works for both EVM and WASM accounts
- Session keys support both execution environments
- Fee delegation functions identically in both runtimes

Coordination Plan:
1. @account-abstraction-architect: Define unified account abstraction interface
2. @evm-integration-specialist: Implement EVM-compatible version
3. @wasm-runtime-specialist: Implement native WASM version
4. All: Joint testing and validation

Success: Users can't tell which runtime their application uses - it just works.
```

### Performance Integration
```
@parallel-execution-architect @consensus-runtime-architect @network-p2p-specialist

We need parallel processing integrated at the consensus level to achieve 4,000 TPS target.

Requirements:
- Parallel block validation in consensus
- Network protocols supporting parallel transaction propagation
- State consistency across all parallel execution threads
- Fallback to sequential processing when needed

Coordination Plan:
1. @parallel-execution-architect: Design conflict detection and parallel execution engine
2. @consensus-runtime-architect: Integrate parallel validation with AlephBFT finality
3. @network-p2p-specialist: Implement parallel transaction distribution protocols
4. All: Joint performance testing and optimization

Success: 1,200-4,000 TPS sustained with 2-3 second finality maintained
```

### Security Integration
```
@testing-qa-specialist @account-abstraction-architect @bridge-interoperability-architect

Critical security validation needed for account abstraction and cross-chain functionality.

Requirements:
- Guardian system security audit
- Session key permission model validation
- Bridge multi-signature security review
- Cross-chain asset transfer security testing

Coordination Plan:
1. @account-abstraction-architect: Provide security model documentation
2. @bridge-interoperability-architect: Provide bridge architecture and threat model
3. @testing-qa-specialist: Coordinate external security audits and penetration testing
4. All: Joint security review sessions and vulnerability assessment

Success: Zero security incidents, successful external audits, comprehensive threat model coverage
```

## Real-World Application Prompts

### Asset Tokenization
```
@account-abstraction-architect @evm-integration-specialist Build support for asset tokenization applications with smart contract accounts.

Context: Real estate and commodity tokenization requires seamless user experience without private key management

Requirements:
- Smart contract accounts for asset holders
- Session keys for automated dividend distributions
- Fee delegation for property management companies
- EVM compatibility for existing tokenization standards

Success: Property owners can tokenize assets and investors can buy tokens without blockchain complexity
```

### Cross-Business Loyalty Programs
```
@account-abstraction-architect @wasm-runtime-specialist @parallel-execution-architect Enable cross-business loyalty programs with high-performance processing.

Context: Multiple businesses issuing loyalty tokens, customers swapping between programs, high transaction volume

Requirements:
- Gasless transactions for customer interactions
- High-performance WASM for real-time point calculations
- Parallel processing for high-volume loyalty transactions
- Session keys for automatic point earning/spending

Success: Customers can earn and spend loyalty points across businesses without apps, cards, or blockchain knowledge
```

### Self-Sovereign Identity
```
@wasm-runtime-specialist @account-abstraction-architect Build identity verification system with zero-knowledge proofs.

Context: Universities issue credentials, employers verify instantly, users control their data completely

Requirements:
- Zero-knowledge proof implementations in WASM
- Social recovery for identity account protection
- Session keys for credential verification
- Privacy-preserving credential sharing

Success: Users can prove qualifications without revealing private details, no central database can be hacked
```

## Troubleshooting and Clarification Prompts

### When Agents Need Clarification
```
@[agent-name] You mentioned [specific issue]. Here's additional context:

From whitepaper Section X: [relevant quote]
Current v3 implementation: [specific details]
Integration requirement: [specific coordination needed]
Timeline constraint: [if applicable]

Please proceed with [specific next step].
```

### For Urgent Coordination
```
@[agent1] @[agent2] URGENT: Integration issue needs immediate resolution.

Problem: [specific technical issue]
Impact: [what breaks if not resolved]
Proposed solution: [suggestion]
Needed from each agent: [specific actions]
Deadline: [when needed]
```

### For Status Updates
```
@[agent-name] Please provide status update on [specific deliverable].

Current milestone: [expected progress]
Integration dependencies: [what's needed from other agents]
Blockers: [any issues preventing progress]
Next steps: [what's planned next]
Timeline: [expected completion]
```

### For Performance Issues
```
@[agent-name] Performance benchmarks show [specific metrics]. We need optimization.

Target performance: [specific requirements from whitepaper]
Current performance: [measured results]
Gap analysis: [difference between target and current]
Optimization needed: [specific improvements required]
Integration impact: [effects on other agent deliverables]
```

## Prompt Best Practices

### 1. Be Specific and Measurable
❌ Bad: "Build account abstraction"
✅ Good: "Implement guardian-based social recovery with 2-of-3 threshold signatures achieving 99%+ recovery success rate"

### 2. Reference Whitepaper Requirements
❌ Bad: "Make transactions faster"
✅ Good: "Implement parallel processing from Section 3 targeting 1,200-4,000 TPS improvement"

### 3. Define Clear Integration Points
❌ Bad: "Work with other teams"
✅ Good: "Coordinate with @consensus-runtime-architect for validator mechanics and @network-p2p-specialist for communication protocols"

### 4. Set Measurable Success Criteria
❌ Bad: "Make it work well"
✅ Good: "Support 1,000+ validators, maintain 99.9% uptime, <1ms additional latency"

### 5. Provide Sufficient Context
❌ Bad: "Fix the validator system"
✅ Good: "Current v3 has MAX_NOMINATORS=1 limiting participation. Need open validator selection for permissionless network"

### 6. Use Phase-Based Approach
❌ Bad: Activate all agents simultaneously
✅ Good: Start with Phase 0 foundation agents, then progressively activate others

### 7. Include Real-World Application Context
❌ Bad: "Build smart contracts"
✅ Good: "Enable loyalty programs where customers earn points across businesses without blockchain complexity"

## Agent Activation Sequence

### Week 1-2: Foundation
1. `@governance-democracy-architect` - Democracy pallet
2. `@consensus-runtime-architect` - Validator selection updates
3. `@ops-infrastructure-manager` - Infrastructure preparation

### Week 3-6: Core Innovation
4. `@account-abstraction-architect` - Smart contract accounts
5. `@testing-qa-specialist` - Security testing framework
6. `@evm-integration-specialist` - EVM compatibility

### Week 7-12: Performance
7. `@wasm-runtime-specialist` - High-performance runtime
8. `@parallel-execution-architect` - Parallel processing
9. `@network-p2p-specialist` - Network optimization

### Week 13+: Production
10. `@developer-experience-architect` - SDK and tooling
11. `@bridge-interoperability-architect` - Cross-chain connectivity

## Conclusion

Effective agent prompting requires:
- **Specificity**: Clear, measurable requirements
- **Context**: Whitepaper references and real-world applications
- **Integration**: Explicit coordination points with other agents
- **Success Criteria**: Measurable outcomes and validation methods
- **Phased Approach**: Sequential activation based on dependencies

Use this guide to systematically activate and coordinate all 11 agents for successful Selendra v4 development.