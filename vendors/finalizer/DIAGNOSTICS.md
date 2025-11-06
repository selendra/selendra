# Finalizer Diagnostics Guide

## Problem: Blocks Stop After Force-Finalization

After using `try-finalize` to force-finalize blocks, you may find that the chain stops producing new blocks after ~20 blocks. This is usually **not** a finalizer bug—the emergency finalizer only finalizes existing blocks; it doesn't author new ones.

## Root Causes

Block production requires:
1. **Active validators** in the current session
2. **Aura authorities** (block authors) configured and running
3. **Authoring-enabled nodes** with correct session keys
4. **Peer connectivity** so nodes can gossip blocks

If any of these is missing, blocks won't be produced.

## Using the Doctor Command

Run diagnostics to check your chain's readiness:

```bash
cd vendors/finalizer
cargo run -- doctor --primary-endpoint ws://127.0.0.1:9944
```

### What It Checks

The `doctor` command inspects:

- **Chain status**: best/finalized blocks across primary and secondary nodes
- **Session index**: current session number
- **Validators count**: how many validators are active (must be > 0)
- **Aura authorities (current)**: how many block authors are configured now
- **Aura authorities (next session)**: upcoming authoring set
- **Emergency finalizer key**: whether the on-chain emergency finalizer is set
- **Finality versions**: current and next-session finality protocol versions
- **Pending extrinsics**: how many transactions are queued in the RPC node

### Sample Output

```
primary                          best: [132, 0xabcd..], finalized: [130, 0x1234..]

Session index: 5
Validators (current) count: 4
Aura authorities (current) count: 4
Next session Aura authorities count: 4
Emergency finalizer key (on-chain): a1b2c3d4e5f6...
Finality version: current=1, next_session=1
RPC node pending extrinsics: 0

Hints:
- Ensure at least one validator node is running with authoring enabled and correct session keys.
- If you just changed validators, wait for the next session for keys/authorities to take effect.
- Emergency finalize only finalizes existing blocks; it doesn't author new ones.
```

## Troubleshooting Steps

### No Validators or Aura Authorities

**Symptom**: Validators count = 0 or Aura authorities count = 0

**Cause**: Validator set not configured or session keys not set.

**Fix**:
1. Check your validator nodes are running with `--validator` flag.
2. Ensure session keys have been set via `author_rotateKeys` RPC and submitted via `session.setKeys` extrinsic.
3. Wait for the next session to begin (session changes happen every N blocks, configured in your runtime).

### Emergency Finalizer Not Activated

**Symptom**: "Emergency finalizer key (on-chain): NONE"

**Cause**: The emergency finalizer is set via `set_emergency_finalizer`, but it only activates on the **next session change**. See `pallets/aleph/src/lib.rs::on_new_session`.

**Fix**:
1. Submit `pallet_aleph::set_emergency_finalizer` (requires Root or AdminOrigin).
2. **Wait for the next session**—the key will be activated in `on_new_session`.
3. Re-run `doctor` to confirm activation.

### Nodes Not Authoring

**Symptom**: Validators and Aura authorities present, but no blocks produced.

**Possible causes**:
- Nodes started with `--rpc-external` but **without** `--validator` or authoring flags.
- Authoring disabled explicitly via `--no-authoring`.
- Session keys on the node don't match the on-chain keys.
- Nodes can't reach each other (no peers; check `--bootnodes` or libp2p connectivity).

**Fix**:
1. Start validator nodes with:
   ```bash
   ./target/release/selendra-node \
     --validator \
     --chain=local \
     --alice \
     --base-path /tmp/alice \
     --bootnodes /ip4/...
   ```
2. Verify session keys match via RPC:
   ```bash
   curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
     http://localhost:9933
   ```
3. Check node logs for errors like "not in authority set" or "no Aura slot claim."

### Session Not Progressing

**Symptom**: Session index stuck; next session authorities never become active.

**Cause**: Block production stalled before session transition.

**Fix**:
1. Ensure block production resumes (see "Nodes Not Authoring" above).
2. Force-finalize up to a block number **before** the session boundary if needed, then let authoring resume naturally.
3. If the emergency finalizer was queued but never activated, you may need to trigger a session manually by producing enough blocks to reach the session boundary.

## Workflow After Force-Finalization

1. **Force-finalize** stalled blocks:
   ```bash
   cargo run -- try-finalize --how-many 20 --seed-path seed.txt
   ```
2. **Run diagnostics**:
   ```bash
   cargo run -- doctor
   ```
3. **Check validators and Aura authorities**—if zero, wait for next session or configure session keys.
4. **Restart/check validator nodes**—ensure authoring is enabled and keys are correct.
5. **Monitor block production**—watch logs or poll `status` command:
   ```bash
   cargo run -- status
   ```

## Additional Notes

- **Emergency finalization is a one-time rescue tool**. Once blocks are finalized, normal consensus (Aura + AlephBFT) must resume.
- **Session boundaries**: New validators/authorities only take effect at session changes. If you force-finalize mid-session, you won't see new authorities until the next session starts.
- **Network partitions**: If your validator nodes can't communicate, they won't produce blocks even if configured correctly. Check libp2p connectivity and bootnodes.

## References

- `pallets/aleph/src/lib.rs`: Emergency finalizer activation in `on_new_session`
- `bin/node/src/rpc/selendra_node_rpc.rs`: RPC handler for emergency finalization
- `crate/finality-aleph/src/justification/`: Justification verification logic
- `vendors/selendra-client/src/pallets/`: Client APIs for session, Aura, and aleph storage
