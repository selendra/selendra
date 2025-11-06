# Emergency Finalizer Key Generation Guide

## Overview

The emergency finalizer key is an **Ed25519** key used to force-finalize blocks when the normal consensus is stalled. The key must be:
1. Generated (Ed25519 keypair)
2. Set on-chain via `pallet_aleph::set_emergency_finalizer`
3. Stored locally in a seed file for the finalizer tool

## Method 1: Using Subkey (Recommended)

### Install Subkey

```bash
# If you don't have subkey installed
cargo install --force --git https://github.com/paritytech/substrate subkey
```

### Generate a New Key

```bash
# Generate a new Ed25519 key
subkey generate --scheme ed25519

# Example output:
# Secret phrase:       bottom drive obey lake curtain smoke basket hold race lonely fit walk
# Secret seed:         0x1234567890abcdef...
# Public key (hex):    0xabcdef1234567890...
# Account ID:          0xabcdef1234567890...
# Public key (SS58):   5Abc...xyz
# SS58 Address:        5Abc...xyz
```

### Save the Secret Phrase

Create or update `seed.txt` with the **secret phrase** (NOT the seed or public key):

```bash
cd /home/naths/project/selendra/vendors/finalizer

# Save the secret phrase to seed.txt
echo "bottom drive obey lake curtain smoke basket hold race lonely fit walk" > seed.txt
```

⚠️ **IMPORTANT**: 
- Use the **secret phrase** (12 or 24 words), NOT the hex seed
- Keep this file secure—it controls emergency finalization
- Use `//` prefix for dev keys (see Method 2)

## Method 2: Using Development Keys

For testing or development chains, you can use well-known dev keys:

### Alice's Key
```bash
echo "//Alice" > seed.txt
```

### Bob's Key
```bash
echo "//Bob" > seed.txt
```

### Charlie's Key
```bash
echo "//Charlie" > seed.txt
```

### Custom Dev Key
```bash
echo "//MyEmergencyKey" > seed.txt
```

The `//` prefix derives a key deterministically from the path—useful for testing.

## Method 3: Generate with Polkadot.js

1. Go to https://polkadot.js.org/apps
2. Navigate to **Developer → RPC calls**
3. Select `author` → `rotateKeys()`
4. Click **Submit RPC call**
5. Extract the Ed25519 portion of the session keys

**Or use the Accounts page:**
1. **Accounts → Add account**
2. Save the **mnemonic seed phrase** (12 or 24 words)
3. Choose **Ed25519** as the key type (important!)
4. Copy the mnemonic to `seed.txt`

## Method 4: Programmatically with Rust

Create a small Rust program:

```rust
use sp_core::{ed25519::Pair, Pair as TraitPair};

fn main() {
    // Generate a new keypair
    let (pair, phrase, _) = Pair::generate_with_phrase(None);
    
    println!("Secret phrase: {}", phrase);
    println!("Public key: 0x{}", hex::encode(pair.public().as_ref()));
    
    // Or derive from a known phrase
    let pair = Pair::from_string("//Alice", None).unwrap();
    println!("Alice public: 0x{}", hex::encode(pair.public().as_ref()));
}
```

## Setting the Key On-Chain

Once you have the key, you must register it on-chain before it can be used.

### Option A: Using Polkadot.js UI

1. Go to **Developer → Extrinsics**
2. Select the **sudo** or **council** account (requires Root or AdminOrigin)
3. Choose extrinsic: `aleph` → `setEmergencyFinalizer(emergencyFinalizer)`
4. For `emergencyFinalizer`, paste the **public key** (SS58 or hex format)
5. Submit the transaction

### Option B: Using Sudo (Command Line)

```bash
# First, get the public key from your seed phrase
subkey inspect "//Alice" --scheme ed25519

# Then submit via polkadot-js-api or your node's RPC
curl -H "Content-Type: application/json" \
  -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method": "author_submitExtrinsic",
    "params": ["0x..."]
  }' \
  http://localhost:9933
```

### Option C: Using selendra-client (Programmatic)

```rust
use selendra_client::{AlephSudoApi, TxStatus, RootConnection, keypair_from_string};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sudo_key = keypair_from_string("//Alice");
    let root = RootConnection::new("ws://127.0.0.1:9944", sudo_key).await?;
    
    // Get the emergency finalizer public key
    let emergency_key_phrase = "//EmergencyFinalizer";
    let emergency_pair = aleph_keypair_from_string(emergency_key_phrase);
    let emergency_account = AccountId::from(emergency_pair.public());
    
    // Set it on-chain
    root.set_emergency_finalizer(emergency_account, TxStatus::Finalized).await?;
    println!("Emergency finalizer set!");
    
    Ok(())
}
```

## Activation Timeline

⚠️ **The key does NOT activate immediately!**

From `pallets/aleph/src/lib.rs`:
> Sets the emergency finalization key. If called in session `N` the key can be used to finalize blocks from session `N+2` onwards, until it gets overridden.

**Timeline**:
- Session N: Call `set_emergency_finalizer`
- Session N+1: Key is queued in `NextEmergencyFinalizer` storage
- Session N+2: Key becomes active in `EmergencyFinalizer` storage (via `on_new_session` hook)

**To check activation**:
```bash
cd vendors/finalizer
cargo run -- doctor

# Look for:
# Emergency finalizer key (on-chain): abc123... (active)
# or
# Emergency finalizer key (on-chain): NONE (not yet active)
```

## Using the Key to Force-Finalize

Once the key is active:

```bash
cd vendors/finalizer

# Finalize the next 10 blocks
cargo run -- try-finalize --how-many 10 --seed-path seed.txt

# The tool will:
# 1. Read the key from seed.txt
# 2. Verify it matches the on-chain emergency finalizer
# 3. Sign each block hash with the key
# 4. Submit via RPC to force-finalize
```

## Verify Key Match

Before attempting finalization, verify your local key matches the on-chain key:

```bash
# Get public key from your seed file
subkey inspect "$(cat seed.txt)" --scheme ed25519

# Example output:
# Public key (hex):    0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

Then compare with on-chain:
```bash
cargo run -- doctor

# Look for:
# Emergency finalizer key (on-chain): d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

If they match, you're ready to finalize!

## Security Considerations

### Production Networks

1. **Generate offline**: Create the key on an air-gapped machine
2. **Use hardware wallet**: Store the seed phrase in a hardware wallet
3. **Multi-sig**: Consider requiring multiple signatures for setting the emergency key
4. **Rotation**: Periodically rotate the emergency finalizer key
5. **Access control**: Limit who can call `set_emergency_finalizer` (AdminOrigin)

### Key Storage

```bash
# Secure the seed file
chmod 600 seed.txt
chown validator:validator seed.txt

# Consider encrypted storage
gpg --encrypt seed.txt
# Decrypt when needed:
gpg --decrypt seed.txt.gpg > seed.txt
```

### Audit Trail

Every emergency finalization is recorded:
- The signed justification is submitted via RPC
- The node imports it and broadcasts to peers
- The signature can be verified against the on-chain emergency finalizer key
- Check node logs and chain events for audit

## Troubleshooting

### "On chain key does not match the key from file"

**Cause**: Your local `seed.txt` doesn't match the on-chain emergency finalizer.

**Fix**:
1. Check which key is set on-chain: `cargo run -- doctor`
2. Verify your local key: `subkey inspect "$(cat seed.txt)" --scheme ed25519`
3. Either:
   - Update `seed.txt` with the correct phrase, OR
   - Set a new emergency finalizer on-chain matching your local key

### "Failed to get the finalizer PK from chain"

**Cause**: No emergency finalizer is set on-chain.

**Fix**:
1. Call `pallet_aleph::set_emergency_finalizer` (requires Root or AdminOrigin)
2. Wait 2 sessions for activation
3. Verify with `cargo run -- doctor`

### "Can't create pair from seed value"

**Cause**: Invalid seed phrase format in `seed.txt`.

**Fix**:
- Use the **mnemonic phrase** (words), not hex seed
- For dev keys, use `//Alice` format with `//` prefix
- Remove any trailing whitespace or newlines

## Examples

### Complete Setup for Development

```bash
# 1. Generate a dev key
echo "//EmergencyFinalizer" > seed.txt

# 2. Get the public key
subkey inspect "//EmergencyFinalizer" --scheme ed25519
# Note the "Account ID" or "Public key (hex)"

# 3. Set it on-chain (using polkadot.js UI or sudo)
# Developer → Extrinsics → aleph.setEmergencyFinalizer(...)

# 4. Wait 2 sessions (check session with: cargo run -- doctor)

# 5. Force-finalize when needed
cargo run -- try-finalize --how-many 5
```

### Complete Setup for Production

```bash
# 1. Generate on air-gapped machine
subkey generate --scheme ed25519 > emergency_key.txt

# 2. Extract secret phrase (first line after "Secret phrase:")
cat emergency_key.txt | grep "Secret phrase" | cut -d: -f2 | xargs > seed.txt

# 3. Securely transfer seed.txt to finalizer machine

# 4. Get public key for on-chain registration
PUBLIC_KEY=$(cat emergency_key.txt | grep "Public key (hex)" | cut -d: -f2 | xargs)

# 5. Submit governance proposal to set emergency finalizer
# (Using council or Root via UI)

# 6. Wait for proposal execution + 2 sessions

# 7. Verify
cargo run -- doctor

# 8. Test (optional, on testnet first)
cargo run -- try-finalize --how-many 1
```

## Summary

**Quick Start (Development)**:
```bash
echo "//Alice" > seed.txt
# Set Alice's public key on-chain via aleph.setEmergencyFinalizer
# Wait 2 sessions
cargo run -- try-finalize --how-many 10
```

**Key Requirements**:
- ✅ Ed25519 keypair (12/24-word phrase or `//DevKey`)
- ✅ Registered on-chain via `set_emergency_finalizer`
- ✅ Active (2 sessions after registration)
- ✅ Local seed file matches on-chain public key

**Verify Before Use**:
```bash
cargo run -- doctor
# Should show: "Emergency finalizer key (on-chain): [your-public-key]"
```
