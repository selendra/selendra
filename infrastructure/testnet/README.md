# Selendra Testnet Node

A Docker-based setup for running a local Selendra blockchain testnet with multiple validators and RPC nodes.

## Quick Start

### Run with Docker Compose

```bash
docker-compose up -d
```

### Run with Docker Swarm

```bash
docker stack deploy -c docker-stack.yml selendra
```

## Setup Nginx Reverse Proxy

Create nginx config to proxy `127.0.0.1:9944` to `rpc.example.org`:

```nginx
server {
    listen 80;
    server_name rpc.example.org;
    
    location / {
        proxy_pass http://127.0.0.1:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Connect to Portal

1. Go to any Substrate portal (e.g., https://portal.selendra.org)
2. Connect to your RPC endpoint: `ws://rpc.example.org` or `ws://localhost:9944`

## Get Testnet Funds

1. **Get account secret**:
```bash
docker exec -it <container-id> /usr/local/bin/selendra-node key inspect //1
```

2. **Copy the secret seed** (e.g., `0x4433c3ada0cf37d3050d5435321872f4f84ef53f8b5f121560689d500b882245`)

3. **In the portal**:
   - Enable local storage when prompted
   - Create account with the secret seed
   - You'll have SEL tokens for testing

## Available Test Accounts

Use seeds `//0`, `//1`, `//2`, `//3`, `//4` to access pre-funded accounts.

## Ports

- **RPC**: 9944-9954
- **P2P**: 30333-30353  
- **Metrics**: 9615-9625

⚠️ **Development only** - Don't use these seeds with real funds!


