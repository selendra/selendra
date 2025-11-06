# Force-Finalization Block Production Fix

## Summary

Added a `doctor` diagnostics command to the finalizer tool to help identify why blocks stop being produced after force-finalization. The tool now provides visibility into session state, validators, Aura authorities, and emergency finalizer configuration.

## Changes Made

### 1. Enhanced `vendors/finalizer/src/commands.rs`

**Added imports**:
- `AlephApi`, `AuthorRpc`, `SessionApi` from `selendra_client::pallets`
- `AccountId` type for session queries
- `AuraPublic` type alias for Aura authority keys
- `rpc_params` for runtime API calls

**New helper functions**:
- `get_current_aura_authorities()`: Queries `AuraApi_authorities` runtime API to get current block authors
- `get_next_session_aura_authorities()`: Queries `AlephSessionApi_next_session_aura_authorities` to check upcoming authors

**New `doctor()` function**:
Performs comprehensive diagnostics:
1. Fetches chain status (best/finalized blocks) from primary and secondary nodes
2. Queries current session index and validator count
3. Checks current and next-session Aura authorities
4. Verifies emergency finalizer key is set on-chain
5. Reports finality protocol versions
6. Checks RPC node health (pending extrinsics)
7. Prints actionable hints for common issues

### 2. Updated `vendors/finalizer/src/main.rs`

**Changes**:
- Imported `doctor` function from `commands` module
- Added `Command::Doctor` variant to CLI enum
- Wired `Doctor` command to call `doctor(connections).await`

**New CLI usage**:
```bash
finalizer doctor [--primary-endpoint ws://...] [--secondary-endpoints ws://...,ws://...]
```

### 3. Created `vendors/finalizer/DIAGNOSTICS.md`

Comprehensive troubleshooting guide covering:
- Root causes of block production failures
- How to use the `doctor` command
- Sample output and interpretation
- Step-by-step fixes for common scenarios:
  - Missing validators or Aura authorities
  - Emergency finalizer not activated
  - Nodes not authoring despite correct config
  - Session transitions stuck
- Recommended workflow after force-finalization
- References to relevant code

## Why Blocks Stop After Force-Finalization

**Force-finalization is not block production**:
- The `try-finalize` command uses the emergency finalizer RPC to mark existing blocks as finalized.
- It does **not** trigger new block authoring.
- Block production depends on:
  1. **Validators** being configured in the session
  2. **Aura authorities** (session keys) being set and active
  3. **Validator nodes** running with authoring enabled
  4. **Network connectivity** for block gossip

If any of these prerequisites is missing, blocks won't be produced—regardless of finalization status.

## Common Scenarios and Solutions

### Scenario 1: Emergency Finalizer Not Yet Active

**Diagnosis via `doctor`**:
```
Emergency finalizer key (on-chain): NONE
```

**Cause**: `set_emergency_finalizer` was called, but the key activates only on the **next session** (see `pallets/aleph/src/lib.rs::on_new_session`).

**Solution**:
1. Wait for the next session to start.
2. Alternatively, trigger session rotation by producing enough blocks (if possible).
3. Re-run `doctor` to confirm activation.

### Scenario 2: No Validators or Aura Authorities

**Diagnosis via `doctor`**:
```
Validators (current) count: 0
Aura authorities (current) count: 0
WARNING: No validators in current session — Aura cannot author blocks.
```

**Cause**: Validator set or session keys not configured.

**Solution**:
1. Start validator nodes with `--validator` flag.
2. Rotate keys via `author_rotateKeys` RPC.
3. Submit session keys via `session.setKeys` extrinsic.
4. Wait for next session boundary for keys to become active.
5. Verify with `doctor` that validators and Aura authorities are now present.

### Scenario 3: Nodes Running but Not Authoring

**Diagnosis via `doctor`**:
```
Validators (current) count: 4
Aura authorities (current) count: 4
```
(But blocks still not being produced)

**Cause**: Nodes may be:
- Started without `--validator` or authoring flags
- Running with `--no-authoring`
- Have mismatched session keys (local vs. on-chain)
- Isolated (no network peers)

**Solution**:
1. Check node startup flags—ensure `--validator` is present.
2. Verify session keys match between node and chain.
3. Check node logs for "not in authority set" or Aura errors.
4. Ensure nodes can reach each other (check `--bootnodes` and libp2p connectivity).

### Scenario 4: Session Stuck

**Diagnosis via `doctor`**:
```
Session index: 5
Next session Aura authorities count: 4
```
(But session never increments)

**Cause**: Sessions advance based on block number. If block production is stalled, the session can't progress.

**Solution**:
1. Fix block production first (see Scenario 2 or 3).
2. Once blocks resume, the session will advance naturally at the next session boundary.
3. If emergency finalizer was queued, it will activate when the new session starts.

## Workflow: From Force-Finalization to Resumed Production

1. **Force-finalize stalled blocks**:
   ```bash
   cd vendors/finalizer
   cargo run -- try-finalize --how-many 20 --seed-path seed.txt
   ```

2. **Run diagnostics**:
   ```bash
   cargo run -- doctor
   ```
   Review output for warnings about validators, authorities, or emergency finalizer.

3. **Fix any issues** identified:
   - No validators → configure session keys and wait for next session.
   - Emergency finalizer NONE → wait for session transition or trigger one.
   - Nodes not authoring → restart with correct flags and verify keys.

4. **Monitor block production**:
   ```bash
   cargo run -- status
   ```
   Repeat until you see `best` block incrementing.

5. **Verify finality** is advancing:
   Check that `finalized` block increases as AlephBFT consensus operates.

## Technical Details

### Emergency Finalizer Activation Path

`pallets/aleph/src/lib.rs`:
```rust
fn on_new_session(...) {
    Self::update_emergency_finalizer();
    // ...
}

fn update_emergency_finalizer() {
    if let Some(finalizer) = NextEmergencyFinalizer::<T>::take() {
        EmergencyFinalizer::<T>::put(finalizer);
    }
}
```
- `set_emergency_finalizer` sets `NextEmergencyFinalizer`.
- On session change, `on_new_session` moves it to `EmergencyFinalizer` (active).
- The finalizer RPC checks `EmergencyFinalizer` storage to verify signatures.

### Aura Block Authoring

Aura (Authority Round) assigns block authoring slots to validators based on:
- **Session keys** (`Aura` key in `pallet_session::NextKeys`)
- **Session validators** from `pallet_session::Validators`
- **Current slot** (timestamp-based)

If no Aura authorities are configured, no slots are assigned → no blocks produced.

### Runtime API Queries

The `doctor` command uses these runtime APIs:
- `AuraApi_authorities`: Current Aura key set
- `AlephSessionApi_next_session_aura_authorities`: Upcoming Aura keys
- `Session::current_index()`: Session index
- `Session::validators()`: Current validator set
- `Aleph::emergency_finalizer()`: Emergency signer public key

All available via `state_call` RPC or storage queries.

## Testing the Fix

1. **Set up a local devnet** with 4 validators (Alice, Bob, Charlie, Dave):
   ```bash
   ./scripts/run_nodes.sh
   ```

2. **Trigger a finalization stall** (for testing):
   - Stop all nodes momentarily to create a gap.
   - Restart nodes; they won't finalize the gap.

3. **Force-finalize**:
   ```bash
   cd vendors/finalizer
   cargo run -- try-finalize --how-many 10
   ```

4. **Run diagnostics**:
   ```bash
   cargo run -- doctor
   ```
   Verify all checks pass (validators, authorities, finalizer key).

5. **Observe blocks resume**:
   ```bash
   cargo run -- status
   # Watch best block number increase
   ```

## Conclusion

The finalizer tool now provides actionable diagnostics to pinpoint **why** blocks aren't being produced after force-finalization. The `doctor` command checks all prerequisites for block authoring and prints warnings with concrete next steps. This eliminates guesswork and accelerates recovery from consensus stalls.

## References

- **Finalizer code**: `vendors/finalizer/src/commands.rs`
- **Emergency finalization RPC**: `bin/node/src/rpc/selendra_node_rpc.rs`
- **Aleph pallet**: `pallets/aleph/src/lib.rs`
- **Session management**: `pallets/committee-management/src/lib.rs`
- **Client APIs**: `vendors/selendra-client/src/pallets/{aleph,session,author}.rs`
- **Diagnostics guide**: `vendors/finalizer/DIAGNOSTICS.md`
