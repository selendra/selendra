# Setting Up Selendra Nodes

This guide covers the setup and operation of different types of Selendra Network nodes.

## Types of Nodes

Selendra Network supports three types of nodes:

- **Archive Node**: Stores the complete blockchain history and state
- **Full Node**: Maintains recent state but prunes older history
- **Light Node**: Minimal storage requirements, relies on full nodes for data
- **Validator Node**: Participates in block production and consensus (requires staking)

## Hardware Requirements

| Node Type | CPU | RAM | Storage | Network |
|-----------|-----|-----|---------|---------|
| Light Node | 4 cores | 8 GB | 50+ GB SSD | 50+ Mbps |
| Full Node | 8 cores | 16 GB | 500+ GB SSD | 100+ Mbps |
| Archive Node | 16+ cores | 64 GB | 2+ TB SSD | 200+ Mbps |
| Validator | 8+ cores | 32+ GB | 1+ TB SSD | 300+ Mbps, low latency |

## Setting Up a Selendra RPC Node

### Using Docker

The quickest way to get started is using our official Docker image:

Pull the latest Selendra node image
docker pull selendranetwork/node:latest

Run a full node

```bash
docker run -d --name selendra-node \
-p 9944:9944 -p 9933:9933 -p 30333:30333 \
-v /path/to/data:/data \
selendranetwork/node:latest \
--name "YOUR_NODE_NAME" \
--rpc-cors=all \
--pruning=1000 \
--chain selendra-mainnet
```

### Using Source Code

Install dependencies (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev
```

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```
Clone the Selendra repository

```bash
git clone https://github.com/selendra/selendra.git
cd selendra
```

Build the node

```bash
cargo build --release
```

Run the node

```bash
cargo run --release --bin selendra-node -- --name "YOUR_NODE_NAME" --rpc-cors=all --pruning=1000 --chain selendra-mainnet
```

## Running a Validator Node

### Prerequisites

- A full node running with the same chain
- 50,000 SEL tokens staked in the network

### Steps to Run a Validator

1. Initialize the validator state

```bash
cargo run --release --bin selendra-node -- --name "YOUR_NODE_NAME" --rpc-cors=all --pruning=1000 --chain selendra-mainnet
```

## Important Configuration Options

| Parameter | Description | Example |
|-----------|-------------|---------|
| `--chain` | Specifies the blockchain network | `--chain=selendra-mainnet` |
| `--pruning` | Sets pruning strategy | `--pruning=archive` (no pruning), `--pruning=1000` (default) |
| `--port` | P2P port | `--port=30333` |
| `--rpc-port` | HTTP RPC port | `--rpc-port=9933` |
| `--ws-port` | WebSocket port | `--ws-port=9944` |
| `--rpc-external` | Allow external RPC | Use with caution! |
| `--ws-external` | Allow external WebSocket | Use with caution! |
| `--validator` | Run as validator | Required for validator nodes |

## Security Considerations

When running nodes, especially validators or nodes with external RPC access:

- Use a firewall to restrict access
- Run behind a reverse proxy for additional security
- Use strong credentials for RPC access
- Regularly update node software
- Monitor system resources and performance

## Running a Validator Node

To set up a validator node, follow these steps after your node is synced:

1. **Generate session keys**:
   ```bash
   curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933
   ```

2. **Stake at least 50,000 SEL tokens** through the Selendra Portal
3. **Bond tokens and set session keys** (see stake-reward.md for details)
4. **Start validation** by running your node with the `--validator` flag

## Monitoring Your Node

To monitor your node's performance:

```bash
# Enable Prometheus metrics
./target/release/selendra-node \
  --prometheus-external \
  --prometheus-port=9615 \
  --chain selendra-mainnet
```

These metrics can be visualized using Grafana with our [pre-configured dashboards](https://github.com/selendra/node-dashboard).

## Upgrading Your Node

When new versions are released:

1. Back up your keys and data
2. Stop your running node
3. Update to the latest version
4. Restart your node

For Docker:
```bash
docker pull selendranetwork/node:latest
docker stop selendra-node
docker rm selendra-node
# Re-run with your original parameters
```

For manual installations:
```bash
cd selendra-node
git pull
cargo build --release
# Restart with your original parameters
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Node won't sync | Check network connectivity and peer count |
| High resource usage | Adjust pruning settings or upgrade hardware |
| Connection refused | Check that ports are open and services running |
| Missing rewards | Ensure validator is properly set up and active |

For more help, join our [Discord community](https://discord.gg/selendra) or check our [troubleshooting forum](https://forum.selendra.org/troubleshooting).


