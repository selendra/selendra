# Selendra Governance & Staking Guide

## 🏛️ Network Governance

Selendra employs a sophisticated governance system with multiple stakeholder groups:

### Validator Management

- **Reserved Validators**: Core infrastructure providers with guaranteed committee seats
- **Non-Reserved Validators**: Community validators competing for rotating seats
- **Committee Size**: Configurable through governance (default: 4-10 validators)
- **Era Duration**: Multiple sessions with automatic rotation

### Governance Mechanisms

#### 1. Committee Management

```bash
# Change validator set (requires root privileges)
selendra-node governance change-validators \
  --reserved-validators="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" \
  --non-reserved-validators="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y" \
  --committee-size=4

# Set election openness
selendra-node governance set-elections-openness permissionless
```

#### 2. Ban Management

```bash
# Configure ban thresholds
selendra-node governance set-ban-config \
  --minimal-expected-performance=75 \
  --underperformed-session-count-threshold=3 \
  --ban-period=2

# Manual ban (emergency only)
selendra-node governance ban-from-committee \
  --validator="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
  --reason="Manual intervention required"
```

## 🔒 Staking Operations

### Setting Up a Validator

#### 1. Generate Keys

```bash
# Generate session keys
selendra-node key generate --scheme Sr25519
# Output: Secret seed and public key

# Generate node key for P2P networking
selendra-node key generate-node-key --base-path /var/lib/selendra
```

#### 2. Validator Node Configuration

```bash
#!/bin/bash
# validator-startup.sh

selendra-node \
  --validator \
  --name="YourValidatorName" \
  --chain=mainnet \
  --base-path=/var/lib/selendra \
  --port=30333 \
  --rpc-port=9944 \
  --ws-port=9944 \
  --prometheus-port=9615 \
  --public-validator-addresses="YOUR_IP:30333" \
  --node-key-file=/var/lib/selendra/node-key \
  --bootnodes="/ip4/BOOTNODE_IP/tcp/30333/p2p/BOOTNODE_PEER_ID" \
  --telemetry-url="wss://telemetry.selendra.org/submit/ 0" \
  --pruning=1000 \
  --db-cache=2048 \
  --pool-limit=8192
```

#### 3. Session Key Management

```bash
# Insert session keys into keystore
selendra-node key insert \
  --base-path /var/lib/selendra \
  --chain mainnet \
  --scheme Sr25519 \
  --suri "YOUR_SECRET_SEED" \
  --key-type aura

# Rotate session keys (hot swap)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944
```

### Staking Economics

#### Reward Distribution

- **Block Production**: Validators earn rewards for producing blocks
- **Finality**: Additional rewards for participating in AlephBFT finality
- **Commission**: Validators can set commission rates for nominators
- **Slash Protection**: Multi-tiered slashing for various offenses

#### Performance Metrics

```bash
# Monitor validator performance
curl -s http://localhost:9615/metrics | grep -E "(selendra_|aleph_)"

# Key metrics to track:
# - selendra_finality_lag
# - aleph_validator_uptime
# - selendra_block_production_rate
# - aleph_network_health
```

### Nominator Operations

#### 1. Staking as Nominator

```javascript
// Using PolkadotJS API
const api = await ApiPromise.create({ provider: wsProvider });

// Nominate validators
const tx = api.tx.staking.nominate([
  "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
]);

await tx.signAndSend(keyring.alice);
```

#### 2. Staking Pools

```javascript
// Join a nomination pool
const poolId = 1;
const amount = api.createType("Balance", "1000000000000"); // 1000 SEL

const joinTx = api.tx.nominationPools.join(amount, poolId);
await joinTx.signAndSend(nominator);
```

## 📊 Network Monitoring

### Key Performance Indicators

#### 1. Network Health

```bash
# Block production rate
curl -s "http://localhost:9944" \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getHeader"}' \
  | jq '.result.number'

# Finality lag
curl -s "http://localhost:9944" \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getFinalizedHead"}'
```

#### 2. Validator Performance

```sql
-- Example queries for monitoring database
SELECT
    validator_id,
    blocks_produced,
    uptime_percentage,
    reward_points
FROM validator_performance
WHERE era = current_era();
```

### Alert Configuration

```yaml
# prometheus-alerts.yml
groups:
  - name: selendra.rules
    rules:
      - alert: ValidatorDown
        expr: up{job="selendra-validator"} == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Validator {{ $labels.instance }} is down"

      - alert: HighFinalityLag
        expr: selendra_finality_lag > 10
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "High finality lag detected"
```

## 🛡️ Security Best Practices

### Validator Security

1. **Key Management**: Use hardware security modules (HSMs)
2. **Network Isolation**: Separate validator and RPC nodes
3. **Monitoring**: 24/7 alerting and monitoring
4. **Backup Strategy**: Regular backup of validator keys and data
5. **Update Management**: Automated security updates

### Sentry Node Architecture

```bash
# Sentry node configuration
selendra-node \
  --sentry \
  --name="Sentry-Node" \
  --reserved-nodes="/ip4/VALIDATOR_PRIVATE_IP/tcp/30333/p2p/VALIDATOR_PEER_ID" \
  --public-addr="/ip4/PUBLIC_IP/tcp/30333" \
  --rpc-external \
  --ws-external \
  --rpc-cors=all
```

This governance and staking system enables:

- **Decentralized Decision Making**: Community-driven network evolution
- **Economic Security**: Aligned incentives through staking
- **Performance Optimization**: Automatic underperformer removal
- **Scalable Consensus**: Efficient committee rotation
- **Enterprise Integration**: Professional validator operations
