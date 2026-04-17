# Selendra Unified Accounts CLI

A command-line tool for binding Substrate (native) accounts to EVM (Ethereum-compatible) accounts on the Selendra network.

## Overview

The Unified Accounts system allows users to create a permanent mapping between their Substrate account and an EVM address. This enables a seamless experience across different virtual machines on the Selendra network.

## Features

- **Claim EVM Address**: Bind your Substrate account to a specific EVM address you control
- **Claim Default EVM Address**: Bind your Substrate account to its deterministically derived default EVM address
- **Query Mappings**: Check existing account mappings on-chain
- **Generate Signatures**: Create EIP-712 signatures for offline signing
- **Account Info**: Display account details without connecting to the network

## Installation

### From Source

```bash
cd crate/unified-accounts-cli
cargo build --release
```

The binary will be available at `target/release/unified-accounts`.

## Usage

### Default Configuration

- **Endpoint**: `wss://rpc-testnet.selendra.org:443`
- **Chain ID**: `1961` (automatically verified against the runtime)

### Claim a Specific EVM Address

To bind your Substrate account to an EVM address you control:

```bash
unified-accounts claim-evm \
    --seed "your twelve word seed phrase here" \
    --evm-key "0xYourEvmPrivateKey"
```

With custom endpoint:

```bash
unified-accounts claim-evm \
    --seed "your twelve word seed phrase here" \
    --evm-key "0xYourEvmPrivateKey" \
    --endpoint wss://rpc-testnet.selendra.org:443
```

### Claim Default EVM Address

To bind your Substrate account to its default derived EVM address:

```bash
unified-accounts claim-default \
    --seed "your twelve word seed phrase here"
```

### Query Account Mappings

Check the EVM address for a Substrate account:

```bash
unified-accounts query \
    --account "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

Check the Substrate account for an EVM address:

```bash
unified-accounts query \
    --evm-address "0x1234567890abcdef1234567890abcdef12345678"
```

### Generate Signature (Offline)

Generate an EIP-712 signature without submitting a transaction:

```bash
unified-accounts sign \
    --seed "your twelve word seed phrase here" \
    --evm-key "0xYourEvmPrivateKey" \
    --genesis-hash "0xed672f56a9cddb5bb20387025f2cf89020c251a8fd06b4b37bd9a44179c9eb87"
```

### Show Account Information

Display information about your accounts:

```bash
unified-accounts info \
    --seed "your twelve word seed phrase here" \
    --evm-key "0xYourEvmPrivateKey"
```

## Environment Variables

You can set credentials via environment variables instead of command-line arguments:

- `SUBSTRATE_SEED`: Your Substrate seed phrase
- `EVM_PRIVATE_KEY`: Your EVM private key

Example:

```bash
export SUBSTRATE_SEED="your twelve word seed phrase here"
export EVM_PRIVATE_KEY="0xYourEvmPrivateKey"
unified-accounts claim-evm
```

## Important Warnings

⚠️ **Read Before Using**

1. **Permanent Binding**: Once a mapping is created, it **CANNOT** be changed or reversed.

2. **Asset Transfer**: When claiming a specific EVM address:
   - Native balance from the default account for that EVM address will be automatically transferred to your account
   - **Other assets** (XC20 tokens, unclaimed staking rewards, etc.) must be transferred **manually BEFORE** claiming
   - Failure to transfer other assets will result in **PERMANENT LOSS**

3. **Storage Fee**: A small storage fee (0.01 SEL) is charged when creating mappings.

4. **Key Security**: Never share your seed phrases or private keys. Use environment variables or secure key management when possible.

## How It Works

### EIP-712 Signature

The unified accounts pallet uses [EIP-712](https://eips.ethereum.org/EIPS/eip-712) typed data signing to verify ownership of an EVM address. The signature contains:

- **Domain**: `Selendra EVM Claim` (version 1)
- **Chain ID**: The EVM chain ID (fetched from runtime, 1961 for testnet)
- **Salt**: The genesis block hash
- **Message**: The SCALE-encoded Substrate account ID

### Default Address Derivation

- **Substrate → EVM**: `blake2_256(account_id)[0..20]`
- **EVM → Substrate**: `blake2_256("evm:" || evm_address || padding)`

## Network Configuration

| Network | Endpoint | Chain ID |
|---------|----------|----------|
| Local | ws://127.0.0.1:9944 | varies |
| Testnet | wss://rpc-testnet.selendra.org:443 | 1961 |
| Mainnet | wss://rpc.selendra.org:443 | 1961 |

> **Note**: The CLI automatically fetches the correct Chain ID from the runtime. If your provided `--chain-id` differs from the runtime value, you'll see a warning and the runtime value will be used.

## Troubleshooting

### "AlreadyMapped" Error

This error occurs when:
- Your Substrate account already has an EVM address mapping
- The EVM address is already mapped to another Substrate account

### "FundsUnavailable" Error

Ensure your account has enough balance to cover:
- The storage fee (0.01 SEL)
- Transaction fees

### "InvalidSignature" Error

This can happen if:
- The EVM private key doesn't match the EVM address
- The chain ID is incorrect (CLI now auto-fetches from runtime)
- The genesis hash doesn't match the network

## Example Output

```
$ unified-accounts claim-evm --seed "..." --evm-key "0x..."

[INFO] Connecting to wss://rpc-testnet.selendra.org:443
[INFO] Substrate account: 5FzwxUhYamfgZRypCcKYSa1bMfaMEVsfs5NGFBrX83NQ5ck2
[INFO] EVM address: 0x2be029ed2e54a661d36f9dbbd34ae4e744de6e78
[INFO] Genesis hash: 0xed672f56a9cddb5bb20387025f2cf89020c251a8fd06b4b37bd9a44179c9eb87
[INFO] Submitting claim_evm_address transaction...
[INFO] Transaction submitted, waiting for confirmation...

✅ Successfully claimed EVM address!
   Substrate account: 5FzwxUhYamfgZRypCcKYSa1bMfaMEVsfs5NGFBrX83NQ5ck2
   EVM address: 0x2be029ed2e54a661d36f9dbbd34ae4e744de6e78
   Block hash: 0xf2b7874912babfd2a4176cd439b7b4e0e725a788652abcad06a6b0992dc56c77
   Tx hash: 0x54f43099348d71a3257fdc7e6c9a4fd42017f8eed075e890004b7fc1ea76749a
```

## License

Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
