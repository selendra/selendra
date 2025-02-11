# Comprehensive Validator Setup Guide

## Pre-requisites

### Hardware Requirements
```
CPU: 8 cores / 16 threads
RAM: 64GB DDR4
Storage: 1TB NVMe SSD
Bandwidth: 1 Gbps, 3TB monthly transfer
```

### Security Requirements
- Dedicated server (no shared hosting)
- DDoS protection
- Hardware security module (optional but recommended)
- Regular security audits
- Backup system

## Initial Setup

### System Preparation
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install security packages
sudo apt install -y ufw fail2ban unattended-upgrades

# Configure automatic security updates
sudo dpkg-reconfigure -plow unattended-upgrades

# Basic security hardening
sudo tee /etc/sysctl.d/90-security.conf > /dev/null << EOF
kernel.sysrq = 0
kernel.core_uses_pid = 1
kernel.kptr_restrict = 2
kernel.panic = 60
kernel.panic_on_oops = 60
kernel.perf_event_paranoid = 3
kernel.yama.ptrace_scope = 2
kernel.unprivileged_bpf_disabled = 1
net.core.bpf_jit_harden = 2
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.all.log_martians = 1
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.all.send_redirects = 0
net.ipv4.conf.default.accept_redirects = 0
net.ipv4.conf.default.log_martians = 1
net.ipv6.conf.all.accept_redirects = 0
net.ipv6.conf.default.accept_redirects = 0
EOF

sudo sysctl -p /etc/sysctl.d/90-security.conf
```

### Validator Account Setup

1. Create Accounts
```bash
# Install Selendra CLI
cargo install subkey --force

# Generate Stash Account
subkey generate --scheme sr25519
# Save the output securely

# Generate Controller Account
subkey generate --scheme sr25519
# Save the output securely

# Generate Session Keys
subkey generate --scheme ed25519
# Save the output securely
```

2. Secure Key Storage
```bash
# Create encrypted storage
sudo apt install -y cryptsetup
sudo cryptsetup luksFormat /dev/nvme0n2
sudo cryptsetup luksOpen /dev/nvme0n2 validator_keys
sudo mkfs.ext4 /dev/mapper/validator_keys
sudo mount /dev/mapper/validator_keys /mnt/validator_keys

# Store keys securely
echo "export VALIDATOR_KEYS=/mnt/validator_keys" >> ~/.bashrc
source ~/.bashrc
```

## Validator Node Setup

### Installation
```bash
# Install dependencies
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown --toolchain nightly

# Clone and build Selendra
git clone https://github.com/selendra/selendra
cd selendra
cargo build --release
```

### Node Configuration
```bash
# Create service file
sudo tee /etc/systemd/system/selendra-validator.service > /dev/null << EOF
[Unit]
Description=Selendra Validator
After=network-online.target
StartLimitIntervalSec=0

[Service]
User=$USER
ExecStart=/home/$USER/selendra/target/release/selendra \
    --validator \
    --chain mainnet \
    --name "YOUR_VALIDATOR_NAME" \
    --pruning archive \
    --telemetry-url "wss://telemetry.selendra.org/submit 0" \
    --port 30333 \
    --rpc-port 9933 \
    --ws-port 9944 \
    --execution Native \
    --base-path /data/selendra \
    --keystore-path /mnt/validator_keys \
    --enable-offchain-indexing true

Restart=always
RestartSec=120
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable selendra-validator
sudo systemctl start selendra-validator
```

## Monitoring Setup

### Prometheus Configuration
```yaml
# /etc/prometheus/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'selendra_validator'
    static_configs:
      - targets: ['localhost:9615']
    metrics_path: /metrics

  - job_name: 'node_exporter'
    static_configs:
      - targets: ['localhost:9100']
```

### Grafana Dashboard
```bash
# Install Grafana
sudo apt-get install -y apt-transport-https software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install grafana

# Start Grafana
sudo systemctl enable grafana-server
sudo systemctl start grafana-server
```

### Alert Configuration
```yaml
# /etc/alertmanager/config.yml
global:
  resolve_timeout: 5m

route:
  group_by: ['alertname']
  group_wait: 30s
  group_interval: 5m
  repeat_interval: 12h
  receiver: 'team-alerts'

receivers:
- name: 'team-alerts'
  telegram_configs:
  - bot_token: 'YOUR_BOT_TOKEN'
    chat_id: YOUR_CHAT_ID
    parse_mode: 'HTML'
```

## Security Measures

### Firewall Setup
```bash
# Configure UFW
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 30333/tcp  # P2P port
sudo ufw enable

# Additional security for SSH
sudo tee -a /etc/ssh/sshd_config > /dev/null << EOF
PermitRootLogin no
PasswordAuthentication no
MaxAuthTries 3
PubkeyAuthentication yes
Protocol 2
EOF

sudo systemctl restart ssh
```

### Fail2ban Configuration
```ini
# /etc/fail2ban/jail.local
[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
findtime = 600
```

## Validator Operations

### Session Key Management
```bash
# Generate session keys
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
     http://localhost:9933

# Set session keys on-chain through the Polkadot.js UI
```

### Staking Operations
```bash
# Check validator status
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9933

# Monitor staking rewards
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "staking_erasRewardPoints", "params":[]}' \
     http://localhost:9933
```

## Backup and Recovery

### Database Backup
```bash
#!/bin/bash
# /usr/local/bin/backup-validator.sh

BACKUP_DIR="/backup/validator"
DATE=$(date +%Y%m%d)

# Stop validator
sudo systemctl stop selendra-validator

# Create backup
tar -czf $BACKUP_DIR/validator-db-$DATE.tar.gz /data/selendra/chains/mainnet/db

# Start validator
sudo systemctl start selendra-validator

# Cleanup old backups (keep last 7 days)
find $BACKUP_DIR -name "validator-db-*.tar.gz" -mtime +7 -delete
```

### Key Backup
```bash
#!/bin/bash
# /usr/local/bin/backup-keys.sh

BACKUP_DIR="/secure-backup/keys"
DATE=$(date +%Y%m%d)

# Backup keys
tar -czf $BACKUP_DIR/validator-keys-$DATE.tar.gz /mnt/validator_keys

# Encrypt backup
gpg --encrypt --recipient your@email.com $BACKUP_DIR/validator-keys-$DATE.tar.gz
```

## Performance Tuning

### System Optimization
```bash
# /etc/sysctl.d/91-validator.conf
vm.swappiness = 1
vm.vfs_cache_pressure = 50
vm.dirty_background_ratio = 5
vm.dirty_ratio = 10
fs.file-max = 2097152
net.core.rmem_max = 67108864
net.core.wmem_max = 67108864
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 87380 67108864
```

### Database Optimization
```bash
# Add to validator service file
--database-cache-size 10240
--max-runtime-instances 8
```

## Maintenance Procedures

### Regular Updates
```bash
#!/bin/bash
# /usr/local/bin/update-validator.sh

cd /home/$USER/selendra

# Fetch updates
git fetch
LATEST_TAG=$(git tag -l | sort -V | tail -n1)

# Stop validator
sudo systemctl stop selendra-validator

# Backup
./backup-validator.sh

# Update
git checkout $LATEST_TAG
cargo build --release

# Start validator
sudo systemctl start selendra-validator
```

### Health Checks
```bash
#!/bin/bash
# /usr/local/bin/check-validator.sh

check_sync() {
    local sync_status=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
        http://localhost:9933 | jq -r '.result.isSyncing')
    
    if [ "$sync_status" = "false" ]; then
        echo "Node is synced"
        return 0
    else
        echo "Node is still syncing"
        return 1
    fi
}

check_validator_status() {
    local active_status=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "staking_activeEra", "params":[]}' \
        http://localhost:9933)
    
    if [ $? -eq 0 ]; then
        echo "Validator is active"
        return 0
    else
        echo "Validator status check failed"
        return 1
    fi
}

main() {
    check_sync || exit 1
    check_validator_status || exit 1
    echo "All checks passed"
}

main
```

## Troubleshooting

### Common Issues

1. Node Not Syncing
```bash
# Check logs
sudo journalctl -u selendra-validator -f

# Check network connectivity
sudo netstat -tulpn | grep selendra
```

2. Missing Blocks
```bash
# Check resource usage
htop
iostat -x 1

# Check validator logs
sudo journalctl -u selendra-validator -n 100 --no-pager
```

3. Performance Issues
```bash
# Monitor system metrics
vmstat 1
iostat -x 1
sar -n DEV 1

# Check for resource constraints
free -h
df -h
```

## Community and Support

### Resources
- [Validator Chat](https://t.me/selendra_validators)
- [Technical Documentation](https://docs.selendra.org)
- [GitHub Repository](https://github.com/selendra/selendra)
- [Forum](https://forum.selendra.org)

### Emergency Contacts
```
Technical Support: support@selendra.org
Security Team: security@selendra.org
Emergency Hotline: +1-XXX-XXX-XXXX
```
