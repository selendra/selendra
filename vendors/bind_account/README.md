# Bind Account CLI

A command-line tool for binding Substrate and EVM addresses using Selendra's unified accounts pallet.

## Features

- **Claim Default EVM Address**: Map your Substrate account to a default-derived EVM address
- **Claim Specific EVM Address**: Map your Substrate account to a specific EVM address with cryptographic proof
- **Generate Mode**: Generate signatures and addresses without submitting transactions

## Installation

```bash
# Build the CLI
cargo build --release --bin bind_account

# Or install globally
cargo install --path vendors/bind_account
```

## Usage

### 1. Claim Default EVM Address

```bash
./target/release/bind_account claim-default \
  --mnemonic "your twelve word mnemonic phrase here" \
  --rpc "ws://127.0.0.1:9944"
```

This will:
- Connect to your Selendra node
- Generate a default EVM address from your Substrate account
- Submit the `claim_default_evm_address()` transaction
- Display the mapping result

### 2. Claim Specific EVM Address

```bash
./target/release/bind_account claim-evm \
  --mnemonic "your twelve word mnemonic phrase here" \
  --rpc "ws://127.0.0.1:9944"
```

This will:
- Generate an EVM address from the same seed
- Create EIP-712 signature proving ownership
- Submit the `claim_evm_address(address, signature)` transaction
- Display the mapping result

### 3. Generate Only (No Transaction)

```bash
./target/release/bind_account generate \
  --mnemonic "your twelve word mnemonic phrase here" \
  --rpc "ws://127.0.0.1:9944"
```

This will:
- Generate all addresses and signatures
- Display parameters for manual use in Polkadot.js Apps
- No blockchain transactions submitted

### 4. Comprehensive Test (NEW)

```bash
./target/release/bind_account test \
  --mnemonic "your twelve word mnemonic phrase here" \
  --rpc "ws://127.0.0.1:9944" \
  --target "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
  --amount "1000000000000"
```

This will:
- Check initial balances on both Substrate and EVM sides
- Claim unified account mapping if not already done
- Perform Substrate transfer to target account
- Perform EVM transfer to target's default EVM address
- Display comprehensive balance changes summary
- Test complete unified accounts functionality

## CLI Help

```bash
# Show all commands
./target/release/bind_account --help

# Show specific command help
./target/release/bind_account claim-default --help
./target/release/bind_account test --help
```

## Example Output

### Claim Default EVM Address

```
🚀 Claiming default EVM address...
📝 Account: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
🔗 Genesis: 0x4f7bd0b84b092d2bbb1a25eca2c23d53af4d79029b7e38d002619ea2ec70125c
🔄 Submitting claim_default_evm_address()...
✅ Transaction submitted successfully!
🎉 Transaction finalized!
🔗 Default EVM address claimed successfully!
   Substrate Account: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   Default EVM Address: 0x83c203c50a836a7384c304c60b846b7485c405c7
```

### Comprehensive Test Output

```
🧪 Starting comprehensive unified accounts test...

📋 Test Configuration:
===========================================
   Source Substrate: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   Source EVM: 0x1a2b3c4d5e6f7890abcdef1234567890abcdef12
   Source Default EVM: 0x83c203c50a836a7384c304c60b846b7485c405c7
   Target Substrate: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
   Target Default EVM: 0x9876543210fedcba0987654321fedcba09876543
   Transfer Amount: 1000000000000 units
   Genesis Hash: 0x4f7bd0b84b092d2bbb1a25eca2c23d53af4d79029b7e38d002619ea2ec70125c

📊 Step 1: Checking initial balances...
===========================================
   Source Substrate Balance: 999999998956980000000 units
   Source EVM Balance: 0 units
   Source Default EVM Balance: 0 units
   Target Substrate Balance: 0 units
   Target Default EVM Balance: 0 units

🔗 Step 2: Claiming unified account...
===========================================
✅ Claim transaction submitted!
🎉 Account claimed successfully!
   Substrate: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   EVM: 0x1a2b3c4d5e6f7890abcdef1234567890abcdef12

💸 Step 3: Substrate transfer...
===========================================
🔄 Submitting Substrate transfer...
✅ Substrate transfer submitted successfully!
🎉 Substrate transfer finalized!

📊 Step 4: Balances after Substrate transfer...
=============================================
   Source Substrate Balance: 999999997956970000000 units (Δ: -1000010000000)
   Target Substrate Balance: 1000000000000 units (Δ: 1000000000000)

💸 Step 5: EVM transfer...
===========================================
⚠️  Insufficient EVM balance for transfer. EVM balance: 0

📊 Step 6: Final balance summary...
====================================
   📈 Balance Changes:
   Source Substrate: 999999998956980000000 → 999999997956970000000 (Δ: -1000010000000)
   Source EVM: 0 → 0 (Δ: 0)
   Source Default EVM: 0 → 0 (Δ: 0)
   Target Substrate: 0 → 1000000000000 (Δ: 1000000000000)
   Target Default EVM: 0 → 0 (Δ: 0)

✅ Unified accounts test completed!
```

## Requirements

- Running Selendra node with unified-accounts pallet
- Account with sufficient balance for transaction fees
- Valid mnemonic phrase for account access

## Security Notes

- Never share your mnemonic phrase
- Use test mnemonics for development
- Verify node connection before submitting transactions
- Account mappings are permanent once created

## Error Handling

Common errors and solutions:

- **Account already mapped**: Each account can only be mapped once
- **Insufficient funds**: Ensure account has balance for fees and storage
- **Invalid signature**: EIP-712 signature verification failed
- **Connection failed**: Check if Selendra node is running and accessible