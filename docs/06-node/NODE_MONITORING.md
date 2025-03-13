# Selendra Node Monitoring Guide

## Monitoring Stack Setup

### Prerequisites
```bash
# Install required packages
sudo apt update
sudo apt install -y prometheus grafana node-exporter alertmanager
```

### Prometheus Configuration
```yaml
# /etc/prometheus/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "alerts.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

scrape_configs:
  - job_name: 'selendra_node'
    static_configs:
      - targets: ['localhost:9615']
    metrics_path: /metrics

  - job_name: 'node_exporter'
    static_configs:
      - targets: ['localhost:9100']

  - job_name: 'validator_monitor'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: /metrics
```

### Alert Rules
```yaml
# /etc/prometheus/alerts.yml
groups:
- name: selendra_alerts
  rules:
  - alert: NodeDown
    expr: up == 0
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "Node {{ $labels.instance }} down"
      description: "Node has been down for more than 5 minutes"

  - alert: HighMemoryUsage
    expr: (node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100 > 90
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High memory usage on {{ $labels.instance }}"
      description: "Memory usage is above 90%"

  - alert: DiskSpaceLow
    expr: node_filesystem_avail_bytes{mountpoint="/"} / node_filesystem_size_bytes{mountpoint="/"} * 100 < 10
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "Low disk space on {{ $labels.instance }}"
      description: "Disk space is below 10%"

  - alert: BlockProductionSlow
    expr: selendra_block_height_change_rate < 0.1
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "Slow block production on {{ $labels.instance }}"
      description: "Block production rate is below threshold"

  - alert: ValidatorOffline
    expr: selendra_validator_active == 0
    for: 2m
    labels:
      severity: critical
    annotations:
      summary: "Validator {{ $labels.instance }} offline"
      description: "Validator node is not active"
```

### Grafana Dashboard
```json
{
  "dashboard": {
    "id": null,
    "title": "Selendra Node Dashboard",
    "tags": ["selendra", "blockchain"],
    "timezone": "browser",
    "panels": [
      {
        "title": "Block Height",
        "type": "graph",
        "datasource": "Prometheus",
        "targets": [
          {
            "expr": "selendra_block_height",
            "legendFormat": "Block Height"
          }
        ]
      },
      {
        "title": "Peer Count",
        "type": "graph",
        "datasource": "Prometheus",
        "targets": [
          {
            "expr": "selendra_sub_libp2p_peers_count",
            "legendFormat": "Peer Count"
          }
        ]
      },
      {
        "title": "System Resources",
        "type": "row",
        "panels": [
          {
            "title": "CPU Usage",
            "type": "graph",
            "datasource": "Prometheus",
            "targets": [
              {
                "expr": "100 - (avg by (instance) (irate(node_cpu_seconds_total{mode=\"idle\"}[5m])) * 100)",
                "legendFormat": "CPU %"
              }
            ]
          },
          {
            "title": "Memory Usage",
            "type": "graph",
            "datasource": "Prometheus",
            "targets": [
              {
                "expr": "(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100",
                "legendFormat": "Memory %"
              }
            ]
          }
        ]
      }
    ]
  }
}
```

## Custom Monitoring Scripts

### Block Production Monitor
```python
#!/usr/bin/env python3
# monitor_blocks.py

import time
import requests
import prometheus_client

# Prometheus metrics
BLOCK_HEIGHT = prometheus_client.Gauge('selendra_block_height', 'Current block height')
BLOCK_TIME = prometheus_client.Histogram('selendra_block_time', 'Time between blocks')

def get_block_height():
    try:
        response = requests.post('http://localhost:9933',
            json={
                "jsonrpc": "2.0",
                "method": "chain_getBlock",
                "params": [],
                "id": 1
            })
        return int(response.json()['result']['block']['header']['number'], 16)
    except Exception as e:
        print(f"Error getting block height: {e}")
        return None

def monitor_blocks():
    prometheus_client.start_http_server(9090)
    last_block = get_block_height()
    last_time = time.time()

    while True:
        current_block = get_block_height()
        current_time = time.time()

        if current_block and last_block:
            if current_block > last_block:
                block_time = (current_time - last_time) / (current_block - last_block)
                BLOCK_TIME.observe(block_time)
                BLOCK_HEIGHT.set(current_block)
                last_block = current_block
                last_time = current_time

        time.sleep(1)

if __name__ == '__main__':
    monitor_blocks()
```

### Validator Performance Monitor
```python
#!/usr/bin/env python3
# monitor_validator.py

import time
import requests
import prometheus_client

# Prometheus metrics
VALIDATOR_ACTIVE = prometheus_client.Gauge('selendra_validator_active', 'Validator active status')
ERA_POINTS = prometheus_client.Gauge('selendra_era_points', 'Era points earned')

def check_validator_status(address):
    try:
        response = requests.post('http://localhost:9933',
            json={
                "jsonrpc": "2.0",
                "method": "staking_activeEra",
                "params": [],
                "id": 1
            })
        return 1 if response.status_code == 200 else 0
    except:
        return 0

def get_era_points(address):
    try:
        response = requests.post('http://localhost:9933',
            json={
                "jsonrpc": "2.0",
                "method": "staking_erasRewardPoints",
                "params": [],
                "id": 1
            })
        points = response.json()['result']['individual'].get(address, 0)
        return points
    except:
        return 0

def monitor_validator(address):
    prometheus_client.start_http_server(9091)

    while True:
        active = check_validator_status(address)
        points = get_era_points(address)

        VALIDATOR_ACTIVE.set(active)
        ERA_POINTS.set(points)

        time.sleep(60)

if __name__ == '__main__':
    monitor_validator('YOUR_VALIDATOR_ADDRESS')
```

## Alert Notifications

### Telegram Bot Setup
```python
#!/usr/bin/env python3
# telegram_alerts.py

import telegram
import asyncio
from aiohttp import web
import json

BOT_TOKEN = 'YOUR_BOT_TOKEN'
CHAT_ID = 'YOUR_CHAT_ID'

async def handle_alert(request):
    try:
        alert = await request.json()
        bot = telegram.Bot(BOT_TOKEN)
        
        for alert in alert['alerts']:
            message = f"""
🚨 *Alert: {alert['labels']['alertname']}*
Severity: {alert['labels']['severity']}
Status: {alert['status']}

{alert['annotations']['description']}

Time: {alert['startsAt']}
            """
            
            await bot.send_message(
                chat_id=CHAT_ID,
                text=message,
                parse_mode=telegram.ParseMode.MARKDOWN
            )
        
        return web.Response(status=200)
    except Exception as e:
        return web.Response(status=500, text=str(e))

app = web.Application()
app.router.add_post('/alert', handle_alert)

if __name__ == '__main__':
    web.run_app(app, port=8080)
```

## Performance Benchmarking

### Network Performance Test
```bash
#!/bin/bash
# benchmark_network.sh

echo "Testing network performance..."

# Test download speed
wget -O /dev/null http://speedtest.tele2.net/1GB.zip 2>&1 | \
    grep -E '/dev/null' | \
    awk '{print $3 " " $4}'

# Test peer connectivity
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_networkState", "params":[]}' \
     http://localhost:9933 | jq .result.connectedPeers

# Test block sync speed
start_block=$(curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock", "params":[]}' \
     http://localhost:9933 | jq .result.block.header.number)

sleep 60

end_block=$(curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock", "params":[]}' \
     http://localhost:9933 | jq .result.block.header.number)

echo "Blocks synced in last minute: $((end_block - start_block))"
```

## Maintenance Scripts

### Automatic Backup
```bash
#!/bin/bash
# backup_node.sh

BACKUP_DIR="/backup/selendra"
DATE=$(date +%Y%m%d)
NODE_DATA="/data/selendra"

# Create backup directory
mkdir -p $BACKUP_DIR

# Stop node
systemctl stop selendra-node

# Create backup
tar -czf $BACKUP_DIR/node-data-$DATE.tar.gz $NODE_DATA

# Start node
systemctl start selendra-node

# Remove backups older than 7 days
find $BACKUP_DIR -name "node-data-*.tar.gz" -mtime +7 -delete

# Send notification
curl -X POST \
     -H "Content-Type: application/json" \
     -d "{\"text\":\"Backup completed: node-data-$DATE.tar.gz\"}" \
     https://your-notification-webhook.com
```

### Log Rotation
```
# /etc/logrotate.d/selendra
/var/log/selendra/*.log {
    daily
    rotate 7
    compress
    delaycompress
    notifempty
    create 640 selendra selendra
    sharedscripts
    postrotate
        systemctl reload selendra-node
    endscript
}
```

## Recovery Procedures

### Emergency Recovery Script
```bash
#!/bin/bash
# recover_node.sh

# Check if node is running
if ! systemctl is-active --quiet selendra-node; then
    echo "Node is down, attempting recovery..."
    
    # Check disk space
    DISK_USAGE=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
    if [ $DISK_USAGE -gt 90 ]; then
        echo "Disk space critical. Cleaning up..."
        journalctl --vacuum-time=2d
    fi
    
    # Check memory
    FREE_MEM=$(free -m | awk 'NR==2 {print $4}')
    if [ $FREE_MEM -lt 1000 ]; then
        echo "Low memory. Clearing caches..."
        sync && echo 3 > /proc/sys/vm/drop_caches
    fi
    
    # Restart node
    systemctl restart selendra-node
    
    # Wait for node to start
    sleep 30
    
    # Check if node is syncing
    SYNC_STATUS=$(curl -s -H "Content-Type: application/json" \
         -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
         http://localhost:9933 | jq -r '.result.isSyncing')
    
    if [ "$SYNC_STATUS" = "true" ]; then
        echo "Node is recovering and syncing..."
    else
        echo "Node recovery failed. Manual intervention required."
        # Send alert
        curl -X POST \
             -H "Content-Type: application/json" \
             -d "{\"text\":\"⚠️ Node recovery failed. Manual intervention required.\"}" \
             https://your-alert-webhook.com
    fi
fi
```
