# Selendra Validator Guide

## Introduction to Validation

### What is a Validator?
Validators are crucial participants in the Selendra network who:
- Secure the network by validating transactions
- Produce new blocks
- Participate in consensus
- Earn rewards for their service

### Requirements
- Minimum 50,000 SEL tokens for staking
- Dedicated server meeting minimum specifications
- 24/7 uptime
- Technical knowledge of Linux systems

## Hardware Requirements

### Recommended Specifications
```
CPU: 4 cores / 8 threads @ 3.4GHz
RAM: 32GB DDR4
Storage: 1TB NVMe SSD
Bandwidth: 1 Gbps, 2TB monthly transfer
```

### Server Setup
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Running a Node

### Installation
```bash
# Clone Selendra
git clone https://github.com/selendra/selendra
cd selendra

# Build the node
cargo build --release

# Start the node
./target/release/selendra \
    --name "YOUR_NODE_NAME" \
    --chain mainnet \
    --pruning archive \
    --telemetry-url "wss://telemetry.selendra.org/submit 0" \
    --validator
```

### Node Management
```bash
# Create service file
sudo tee /etc/systemd/system/selendra.service > /dev/null << EOF
[Unit]
Description=Selendra Validator
After=network-online.target

[Service]
User=$USER
ExecStart=/home/$USER/selendra/target/release/selendra \
    --validator \
    --name "YOUR_NODE_NAME" \
    --chain mainnet \
    --pruning archive \
    --telemetry-url "wss://telemetry.selendra.org/submit 0"
Restart=always
RestartSec=3
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable selendra
sudo systemctl start selendra
```

## Validator Setup

### Generate Session Keys
```bash
# Generate keys
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933
```

### Bond Tokens
1. Go to Network → Staking → Account actions
2. Click "Validator" button
3. Enter the amount to stake
4. Set your session keys
5. Set commission rate

### Monitoring

#### Prometheus Setup
```yaml
# /etc/prometheus/prometheus.yml
scrape_configs:
  - job_name: 'selendra_node'
    static_configs:
      - targets: ['localhost:9615']
```

#### Grafana Dashboard
- Import Selendra dashboard template
- Monitor key metrics:
  - Block production
  - Validator performance
  - System resources
  - Network connectivity

## Security Best Practices

### Server Security
```bash
# Update firewall rules
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 30333/tcp  # P2P port
sudo ufw enable

# Secure SSH
sudo nano /etc/ssh/sshd_config
# Set:
PermitRootLogin no
PasswordAuthentication no
```

### Key Management
- Use separate accounts for:
  - Stash (holds funds)
  - Controller (manages validation)
  - Session (runs validator node)
- Keep keys in cold storage
- Use hardware wallets when possible

## Troubleshooting

### Common Issues
1. Node not synchronizing
```bash
# Check logs
sudo journalctl -u selendra -f
```

2. Missing blocks
```bash
# Check node status
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' http://localhost:9933
```

3. Performance issues
```bash
# Monitor system resources
htop
iostat -x 1
```

## Maintenance

### Regular Tasks
1. Monitor system resources
2. Update software regularly
3. Backup session keys
4. Review performance metrics
5. Participate in governance

### Updates
```bash
# Update Selendra node
cd selendra
git fetch
git checkout <latest-release>
cargo build --release
sudo systemctl restart selendra
```

## Community and Support
- Join our [Validator Chat](https://t.me/selendra_validators)
- Follow [Validator Updates](https://twitter.com/selendra_val)
- Report issues on [GitHub](https://github.com/selendra/selendra)
- Participate in [governance](https://selendra.org/governance)
