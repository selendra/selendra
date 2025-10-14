---
title: Run a Node
section: Node Operations
order: 3
---

# Run a Selendra Node

Learn how to run a Selendra node and participate in the network.

## System Requirements

- **CPU**: 4+ cores
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 200GB SSD
- **Network**: Stable internet connection

## Installation

### Using Docker

```bash
docker pull selendrachain/selendra-node:latest

docker run -d \
  --name selendra-node \
  -p 9944:9944 \
  -p 9933:9933 \
  -p 30333:30333 \
  selendrachain/selendra-node:latest \
  --chain mainnet \
  --rpc-external \
  --ws-external
```

### Build from Source

```bash
git clone https://github.com/selendra/selendra.git
cd selendra
cargo build --release

./target/release/selendra-node \
  --chain mainnet \
  --name "My Node"
```

## Validator Setup

Coming soon...
