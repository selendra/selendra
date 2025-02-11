# Running a Selendra RPC Node

## Introduction
This guide will help you set up and maintain a high-performance RPC node for the Selendra network.

## Hardware Requirements

### Minimum Specifications
```
CPU: 8 cores / 16 threads
RAM: 64GB DDR4
Storage: 2TB NVMe SSD
Bandwidth: 1 Gbps, 5TB monthly transfer
```

### Recommended Specifications
```
CPU: 16 cores / 32 threads
RAM: 128GB DDR4
Storage: 4TB NVMe SSD (RAID 1)
Bandwidth: 2.5 Gbps, 10TB monthly transfer
```

## Installation

### System Preparation
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Node Installation
```bash
# Clone Selendra
git clone https://github.com/selendra/selendra
cd selendra

# Build with specific features
cargo build --release --features fast-runtime
```

## Configuration

### Node Configuration
```bash
# Create service file
sudo tee /etc/systemd/system/selendra-rpc.service > /dev/null << EOF
[Unit]
Description=Selendra RPC Node
After=network-online.target

[Service]
User=$USER
ExecStart=/home/$USER/selendra/target/release/selendra \
    --chain mainnet \
    --name "YOUR_RPC_NODE" \
    --pruning archive \
    --rpc-cors all \
    --rpc-methods unsafe \
    --rpc-external \
    --ws-external \
    --rpc-port 9933 \
    --ws-port 9944 \
    --port 30333 \
    --max-runtime-instances 100 \
    --execution Native \
    --telemetry-url "wss://telemetry.selendra.org/submit 0"
Restart=always
RestartSec=3
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable selendra-rpc
sudo systemctl start selendra-rpc
```

### Nginx Reverse Proxy
```nginx
# /etc/nginx/sites-available/selendra-rpc
upstream selendra_rpc {
    server 127.0.0.1:9933;
    keepalive 32;
}

upstream selendra_ws {
    server 127.0.0.1:9944;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    server_name rpc.your-domain.com;

    ssl_certificate /etc/letsencrypt/live/rpc.your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rpc.your-domain.com/privkey.pem;

    # Security headers
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=rpc_limit:10m rate=100r/s;
    limit_req zone=rpc_limit burst=200 nodelay;

    # HTTP RPC
    location / {
        proxy_pass http://selendra_rpc;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    # WebSocket
    location /ws {
        proxy_pass http://selendra_ws;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }
}
```

## Monitoring

### Prometheus Configuration
```yaml
# /etc/prometheus/prometheus.yml
scrape_configs:
  - job_name: 'selendra_rpc'
    static_configs:
      - targets: ['localhost:9615']
    metrics_path: /metrics
```

### Grafana Dashboard
```json
{
  "dashboard": {
    "panels": [
      {
        "title": "RPC Requests",
        "targets": [
          {
            "expr": "rate(selendra_rpc_requests_total[5m])",
            "legendFormat": "{{method}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "targets": [
          {
            "expr": "rate(selendra_rpc_response_time_seconds_sum[5m])",
            "legendFormat": "{{method}}"
          }
        ]
      }
    ]
  }
}
```

## Security

### Firewall Configuration
```bash
# Configure UFW
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 30333/tcp  # P2P port
sudo ufw enable
```

### SSL Configuration
```bash
# Install certbot
sudo apt install -y certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d rpc.your-domain.com
```

## Load Balancing

### HAProxy Configuration
```haproxy
# /etc/haproxy/haproxy.cfg
frontend selendra_rpc
    bind *:443 ssl crt /etc/ssl/certs/selendra.pem
    mode http
    option forwardfor
    default_backend rpc_nodes

backend rpc_nodes
    mode http
    balance roundrobin
    option httpchk POST / HTTP/1.1\r\nContent-Type:\ application/json\r\n\r\n{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}
    http-check expect status 200
    server rpc1 127.0.0.1:9933 check
    server rpc2 127.0.0.1:9934 check
```

## Maintenance

### Database Management
```bash
# Compact database
./target/release/selendra purge-chain --chain mainnet

# Backup database
sudo systemctl stop selendra-rpc
tar -czf selendra-db-backup.tar.gz /home/$USER/.local/share/selendra/chains/mainnet/
sudo systemctl start selendra-rpc
```

### Update Procedure
```bash
# Stop service
sudo systemctl stop selendra-rpc

# Update code
cd selendra
git fetch
git checkout <latest-release>
cargo build --release --features fast-runtime

# Start service
sudo systemctl start selendra-rpc
```

## Troubleshooting

### Common Issues

1. High Memory Usage
```bash
# Check memory usage
free -h
# Adjust runtime instances
--max-runtime-instances 50
```

2. Slow Response Times
```bash
# Check system load
htop
# Monitor I/O
iostat -x 1
```

3. Connection Issues
```bash
# Check node status
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9933
```

## Performance Optimization

### System Tuning
```bash
# /etc/sysctl.conf
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 87380 67108864
net.core.netdev_max_backlog = 300000
net.ipv4.tcp_max_syn_backlog = 200000
net.ipv4.tcp_fastopen = 3
```

### Database Optimization
```bash
# Adjust database cache size
--database-cache-size 10240  # 10GB cache

# Use faster storage for database
mount -o noatime,data=writeback /dev/nvme0n1p1 /path/to/db
```

## Monitoring and Alerts

### Healthcheck Script
```bash
#!/bin/bash
# /usr/local/bin/check_rpc.sh

check_rpc() {
    response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://localhost:9933)
    
    if [[ $response == *"\"isSyncing\":false"* ]]; then
        echo "RPC node is healthy"
        exit 0
    else
        echo "RPC node is not responding correctly"
        exit 2
    fi
}

check_rpc
```

### Alert Configuration
```yaml
# /etc/alertmanager/alertmanager.yml
route:
  receiver: 'team-alerts'
  group_by: ['alertname']
  group_wait: 30s
  group_interval: 5m
  repeat_interval: 4h

receivers:
- name: 'team-alerts'
  slack_configs:
  - api_url: 'https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK'
    channel: '#node-alerts'
    send_resolved: true
```
