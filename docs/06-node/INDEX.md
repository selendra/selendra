# Selendra Node Operation Guides

Welcome to the Selendra node operation documentation. This section contains comprehensive guides for running and maintaining Selendra network nodes.

## Available Guides

### [Validator Setup Guide](VALIDATOR_SETUP.md)
- Complete validator node setup
- Security best practices
- Monitoring and maintenance
- Backup and recovery procedures

### [RPC Node Guide](RPC_NODE_GUIDE.md)
- RPC node setup and configuration
- Load balancing and scaling
- Performance optimization
- Security hardening

### [Node Monitoring Guide](NODE_MONITORING.md)
- Prometheus and Grafana setup
- Alert configuration
- Performance metrics
- Health checks

## Quick Start

### For Validators
```bash
# Quick validator node setup
git clone https://github.com/selendra/selendra
cd selendra
cargo build --release
./target/release/selendra --validator --chain mainnet
```

### For RPC Providers
```bash
# Quick RPC node setup
./target/release/selendra \
    --chain mainnet \
    --rpc-cors all \
    --rpc-methods unsafe \
    --rpc-external
```

## Hardware Requirements

### Validator Node
- CPU: 8 cores / 16 threads
- RAM: 64GB DDR4
- Storage: 1TB NVMe SSD
- Bandwidth: 1 Gbps

### RPC Node
- CPU: 16 cores / 32 threads
- RAM: 128GB DDR4
- Storage: 4TB NVMe SSD
- Bandwidth: 2.5 Gbps

## Support Resources
- [Technical Documentation](https://docs.selendra.org)
- [GitHub Repository](https://github.com/selendra/selendra)
- [Validator Chat](https://t.me/selendra_validators)
- [Node Operators Forum](https://forum.selendra.org/node-operators)
