# Analysis: Reserved vs Non-Reserved Validators in Selendra

Based on the codebase analysis, here are the **key differences** between reserved and non-reserved validators:

## **1. Persistence & Immunity**

**Reserved Validators:**
- **Permanent members** of the committee
- **Cannot be banned** from the committee
- Always included in every session
- Protected from performance-based removal

**Non-Reserved Validators:**
- **Can be banned** for poor performance
- Subject to removal based on:
  - Insufficient block production
  - Insufficient finalization performance
- Subject to rotation and committee selection

## **2. Committee Selection**

**Reserved Validators:**
- **All reserved validators** are chosen for every session
- Selection happens via `reserved_seats` parameter
- Rotated through but always present in committee

**Non-Reserved Validators:**
- Selected based on `non_reserved_seats` parameter
- Only a subset participates in each session
- Rotation algorithm 
  - Chosen from range: `n * seats` to `(n + 1) * seats` where `n` is session index
  - Not all non-reserved validators are in committee at the same time

## **3. Election Process**

**Reserved Validators:**
- Pre-defined list stored in `NextEraReservedValidators`
- Must be staking (filtered by staking validators)
- Not subject to election competition

**Non-Reserved Validators:**
Two modes based on `ElectionOpenness`:
- **Permissioned Mode**: Only validators from `NextEraNonReservedValidators` list (filtered by staking + not banned)
- **Permissionless Mode**: **Any** staking validator not banned and not in reserved set


## **Summary Table**

| Feature | Reserved Validators | Non-Reserved Validators |
|---------|-------------------|------------------------|
| **Ban Immunity** | ✅ Cannot be banned | ❌ Can be banned |
| **Committee Presence** | Always all included | Rotated subset |
| **Finality Participation** | All participate | Subset participates |
| **Election** | Pre-defined list | Elected (Permissionless) or List (Permissioned) |
| **Performance Impact** | Reward adjustment only | Ban + removal from committee |
| **Use Case** | Core/Foundation nodes | Community/External validators |

## **Design Rationale**

This two-tier system provides:
1. **Stability**: Reserved validators ensure network core stability
2. **Decentralization**: Non-reserved slots allow community participation
3. **Quality Control**: Performance-based bans maintain network health
4. **Flexibility**: Permissioned/Permissionless modes adapt to network maturity

## **Key Implementation Details**
Reserved validators return `false` for `can_ban()`, making them immune to banning.

The rotation ensures:
1. `reserved_seats` validators chosen from reserved set
2. `non_reserved_seats` validators chosen from non-reserved set
3. Selection uses session index for deterministic rotation
4. Finality committee: all reserved + `non_reserved_finality_seats` from non-reserved

### Election Process Flow
Location: `/pallets/elections/src/lib.rs:303-380`

**Permissioned Mode:**
```
1. Get staking validators
2. Filter reserved validators (must be staking)
3. Filter non-reserved from NextEraNonReservedValidators (must be staking + not banned)
4. Combine for final committee
```

**Permissionless Mode:**
```
1. Get all staking validators
2. Filter reserved validators (must be staking)
3. Get ALL eligible validators (staking + not banned + not reserved)
4. Combine for final committee
```

## **Validator Lifecycle**

### Reserved Validator Lifecycle
```
Setup (Governance) → Staking → Committee → Performance Tracking → Reward Adjustment
                                    ↑                                       ↓
                                    └───────────────────────────────────────┘
                                         (Always remains in committee)
```

### Non-Reserved Validator Lifecycle
```
Setup/Election → Staking → Committee → Performance Tracking → Good Performance → Continue
                               ↑                                                      ↓
                               │                                  Poor Performance    │
                               │                                         ↓            │
                               └──────────── Ban (N eras) ←─────────────┘←───────────┘
```

## **Configuration Parameters**

### Committee Seats Configuration
- `reserved_seats`: Number of reserved validators in committee
- `non_reserved_seats`: Number of non-reserved validators in committee  
- `non_reserved_finality_seats`: Non-reserved validators participating in finality

### Ban Configuration (Production)
- `minimal_expected_performance`: Performance threshold (Perbill)
- `underperformed_session_count_threshold`: Sessions before ban
- `clean_session_counter_delay`: Session counter reset interval
- `ban_period`: Number of eras for ban duration

### Ban Configuration (Finality)
- `minimal_expected_performance`: ABFT performance threshold (u16)
- `underperformed_session_count_threshold`: Sessions before ban
- `ban_period`: Number of eras for ban duration
- `clean_session_counter_delay`: Session counter reset interval

## **Best Practices**

### For Reserved Validators
- Should be run by trusted foundation/core team
- Must maintain high uptime (rewards affected if not)
- Provide network stability and bootstrapping
- Typically 10-30% of total committee size

### For Non-Reserved Validators
- Community-run validators
- Must monitor performance closely to avoid bans
- Subject to competition in Permissionless mode
- Typically 70-90% of total committee size

### Network Configuration
- **Early Stage**: Permissioned mode with curated non-reserved list
- **Mature Network**: Permissionless mode for full decentralization
- **Reserved Ratio**: Keep 10-30% reserved for stability
- **Finality Seats**: Balance between decentralization and consensus efficiency
