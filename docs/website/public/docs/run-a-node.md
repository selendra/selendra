---
title: Run a Node
section: Node Operations
order: 3
---

# Run a Node

Help secure the network. Run a node.

## System Requirements

**Minimum**
- CPU: 4 cores
- RAM: 8GB
- Storage: 200GB SSD
- Network: Stable connection

**Recommended**
- CPU: 8 cores
- RAM: 16GB
- Storage: 500GB NVMe SSD
- Network: 100 Mbps

## Quick Start

### Docker (Easiest)

```bash
docker pull selendrachain/selendra-node:latest

docker run -d \
  --name selendra-node \
  -p 9944:9944 \
  -p 9933:9933 \
  -p 30333:30333 \
  -v selendra-data:/data \
  selendrachain/selendra-node:latest \
  --chain mainnet \
  --name "My Node" \
  --rpc-external \
  --ws-external \
  --rpc-cors all
```

Node running. That's it.

### From Source

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# Clone
git clone https://github.com/selendra/selendra.git
cd selendra

# Build (takes 30-60 minutes)
cargo build --release

# Run
./target/release/selendra-node \
  --chain mainnet \
  --name "My Node" \
  --base-path /var/lib/selendra \
  --rpc-port 9933 \
  --ws-port 9944 \
  --port 30333
```

## Node Types

**Full Node**
Stores complete blockchain history. Serves data.

```bash
./selendra-node --chain mainnet --pruning archive
```

**Light Node**
Minimal storage. Good for development.

```bash
./selendra-node --chain mainnet --light
```

**Validator Node**
Produces blocks. Requires stake.

See [Become a Validator](/docs/become-validator)

## Monitoring

### Check Sync Status

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
  http://localhost:9933

# Via logs
docker logs -f selendra-node
```

### Metrics

```bash
# Enable Prometheus
./selendra-node \
  --chain mainnet \
  --prometheus-external \
  --prometheus-port 9615
```

Access metrics at `http://localhost:9615/metrics`

### Health Check

```bash
# Check if node is running
curl http://localhost:9933/health
```

## Backup

Important: Backup your node key.

```bash
# Node key location
/var/lib/selendra/chains/selendra/network/secret_ed25519

# Backup
cp /var/lib/selendra/chains/selendra/network/secret_ed25519 ~/selendra-key.backup
```

## Upgrade

### Docker

```bash
docker stop selendra-node
docker pull selendrachain/selendra-node:latest
docker start selendra-node
```

### Source

```bash
cd selendra
git pull
cargo build --release

# Restart node with new binary
systemctl restart selendra
```

## Troubleshooting

**Node not syncing**
```bash
# Check peers
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9933
```

**High disk usage**
```bash
# Prune old state
./selendra-node \
  --chain mainnet \
  --pruning 1000  # Keep last 1000 blocks
```

**Port conflicts**
Check ports 9933, 9944, 30333 are not in use.

```bash
sudo netstat -tulpn | grep -E '9933|9944|30333'
```

## Systemd Service

Run node as a service.

```bash
# Create service file
sudo nano /etc/systemd/system/selendra.service
```

```ini
[Unit]
Description=Selendra Node
After=network.target

[Service]
Type=simple
User=selendra
ExecStart=/usr/local/bin/selendra-node \
  --chain mainnet \
  --name "My Node" \
  --base-path /var/lib/selendra
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable selendra
sudo systemctl start selendra

# Check status
sudo systemctl status selendra

# View logs
journalctl -u selendra -f
```

## Security

**Firewall**
```bash
# Open required ports
sudo ufw allow 30333/tcp  # P2P
sudo ufw allow 9933/tcp   # RPC (if needed)
sudo ufw allow 9944/tcp   # WS (if needed)
sudo ufw enable
```

**RPC Security**
Don't expose RPC to internet unless necessary.

```bash
# Local only (safe)
--rpc-port 9933

# Public (use with caution)
--rpc-external --rpc-cors all
```

## Resources

RAM usage: ~4-8GB
Storage growth: ~10GB/month
Bandwidth: ~50GB/month

## Next Steps

Want to validate? See [Become a Validator](/docs/become-validator)

Need help? Ask in Discord.

https://discord.gg/selendra
