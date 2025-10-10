---
title: Become a Validator
section: Node Operations
order: 6
---

# Become a Validator

Join the Selendra network as a validator and help secure the chain.

## Requirements

### Hardware
- **CPU**: 8+ cores (modern processor)
- **RAM**: 32GB minimum
- **Storage**: 500GB NVMe SSD
- **Network**: 1Gbps connection, static IP

### Stake
- Minimum stake: TBD SEL tokens
- Competitive stake recommended for active set

## Setup Steps

### 1. Run a Full Node

First, run a synced full node (see [Run a Node](./run-a-node.md)).

### 2. Generate Session Keys

```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933
```

Save the returned session keys.

### 3. Bond Tokens

Use the Polkadot.js Apps interface:

1. Navigate to Network > Staking > Account actions
2. Click "New stake"
3. Select your stash and controller accounts
4. Enter amount to bond
5. Submit transaction

### 4. Set Session Keys

1. Go to Network > Staking > Account actions
2. Click "Set Session Key" on your stash
3. Paste the session keys from step 2
4. Submit transaction

### 5. Validate

Click "Validate" and set your commission rate.

## Maintenance

- Monitor node uptime and performance
- Keep software updated
- Maintain sufficient stake to stay in active set
- Respond quickly to any issues

## Slashing

Validators can be slashed for:
- **Downtime**: Offline for extended periods
- **Equivocation**: Double-signing blocks

Keep backups and monitoring in place to avoid slashing.
