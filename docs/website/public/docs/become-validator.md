---
title: Become a Validator
section: Node Operations
order: 6
---

# Become a Validator

Secure the network. Earn rewards.

## Requirements

**Hardware**
- CPU: 8+ cores
- RAM: 16GB minimum, 32GB recommended
- Storage: 500GB NVMe SSD
- Network: 1Gbps, <50ms latency

**Stake**
- Minimum: TBD SEL
- Recommended: Higher stake = Higher chance of selection

**Uptime**
- 99.9% uptime required
- Slash risk if offline

## How It Works

**AlephBFT + Aura Consensus**
- Committee of validators produces blocks
- Rotating selection based on stake
- 1000ms blocks, <2s finality

**Rewards**
- Block production rewards
- Transaction fees
- Nomination pool rewards

**Penalties**
- Offline: Small slash
- Double-sign: Large slash
- Lose validator status

## Setup

### 1. Run a Full Node

First, sync a full node.

```bash
./selendra-node \
  --chain mainnet \
  --name "My Validator" \
  --validator \
  --base-path /var/lib/selendra
```

Wait for sync. Can take 1-2 days.

### 2. Generate Session Keys

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9933
```

Save the keys. You need them.

### 3. Bond Tokens

Go to https://explorer.selendra.org

**Steps:**
1. Connect wallet
2. Go to Staking → Account Actions
3. Click "Bond Funds"
4. Enter amount (minimum stake)
5. Select "Staked" as destination
6. Submit transaction

### 4. Set Session Keys

Still on explorer:

1. Go to Staking → Account Actions
2. Click "Set Session Keys"
3. Paste keys from step 2
4. Submit transaction

### 5. Validate

1. Go to Staking → Account Actions
2. Click "Validate"
3. Set commission (0-100%)
4. Submit transaction

Done. Now wait for election.

## Monitoring

**Check validator status**
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9933
```

**Monitor metrics**
```bash
# Enable Prometheus
./selendra-node \
  --validator \
  --prometheus-external \
  --prometheus-port 9615
```

Set up Grafana. Monitor:
- Block production
- Peer count
- Sync status
- System resources

**Alerts**
Set up alerts for:
- Node offline
- Not producing blocks
- High resource usage

## Commission

Your cut of rewards.

- 0%: All rewards to nominators
- 5%: You keep 5%, give 95% to nominators
- 100%: You keep all rewards

Higher commission = Fewer nominators.

## Nominators

Other users can nominate your validator.

**Benefits:**
- More stake = Higher election chance
- Share rewards

**How it works:**
1. Users bond tokens
2. Nominate your validator
3. If you're elected, they earn too
4. You get commission

## Nomination Pools

Alternative to direct nomination.

**Advantages:**
- Lower minimum stake
- Easier for users
- More stable rewards

**Setup:**
Create a nomination pool for your validator.

```bash
# Coming soon: CLI command
selendra pools create --validator <address>
```

## Economics

**Example Calculation**

Stake: 10,000 SEL
Block rewards: 1 SEL per block
Blocks per day: 86,400 (1s blocks)
Your share: 1% (if 100 validators)

Daily: 864 SEL × 1% = 8.64 SEL
Annual: 3,153 SEL (31.5% APY)

Minus commission for nominators.

**Reality Check**
- Actual APY: 10-30%
- Depends on total stake
- Depends on election rate
- Slashing reduces rewards

## Slashing

**Offline Slash**
- Miss too many blocks
- Lose 0.1% of stake

**Equivocation Slash**
- Double sign (run two nodes)
- Lose 100% of stake
- Get kicked

**How to Avoid**
- Run one node only
- High uptime
- Monitor constantly
- Have backup plan

## Best Practices

**Security**
- Firewall properly
- No SSH password auth
- Separate keys for signing
- Keep session keys secret

**Redundancy**
- Have backup server ready
- Monitor uptime 24/7
- Alert system configured

**Updates**
- Subscribe to announcements
- Upgrade during low-activity
- Test on testnet first

**Community**
- Join validator Discord
- Share uptime reports
- Help new validators

## Troubleshooting

**Not getting elected**
- Increase stake
- Lower commission
- Build reputation
- Get nominators

**Missing blocks**
- Check node sync
- Check session keys
- Restart node
- Check system resources

**High resource usage**
- Upgrade hardware
- Optimize node config
- Prune old state

## Economics

**Costs**
- Server: $50-200/month
- Stake: Lock up capital
- Time: Monitoring & maintenance

**Revenue**
- Block rewards
- Transaction fees
- Nomination commission

**Break-even**
Depends on stake size and commission.

## Resources

**Tools**
- Explorer: https://explorer.selendra.org
- Telemetry: https://telemetry.selendra.org
- Validator Stats: https://stats.selendra.org

**Community**
- Validator Discord: https://discord.gg/selendra
- Weekly calls: Thursdays 4pm UTC

**Guides**
- Security: https://docs.selendra.org/security
- Monitoring: https://docs.selendra.org/monitoring
- Troubleshooting: https://docs.selendra.org/troubleshooting

## Next Steps

1. Sync a full node
2. Get minimum stake
3. Generate session keys
4. Bond and validate
5. Monitor and earn

Questions? Ask in Discord.

https://discord.gg/selendra
