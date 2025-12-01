# Pallets Removed from polkadot-sdk Workspace

## Summary
- **Total directories on disk**: 81 frame pallets
- **Pallets in workspace**: 42 pallets
- **Removed from compilation**: ~39 pallets

## What This Means
The removed pallets still exist on disk in `vendors/substrate/frame/` but they are **NOT included** in the workspace's `Cargo.toml`. This means:
- ✅ They won't be compiled when you run `cargo build`
- ✅ They won't be checked when you run `cargo check`
- ✅ Faster build times
- ✅ Less disk space used during builds
- ✅ Cleaner dependency tree

## Pallets KEPT (Used by Selendra)
```
✓ aura
✓ authorship
✓ balances
✓ benchmarking
✓ collective
✓ contracts (+ proc-macro + uapi)
✓ democracy
✓ election-provider-support (+ solution-type)
✓ elections-phragmen
✓ executive
✓ identity
✓ multisig
✓ nomination-pools (+ runtime-api)
✓ preimage
✓ proxy
✓ safe-mode
✓ scheduler
✓ session
✓ staking (+ reward-curve + reward-fn + runtime-api)
✓ sudo
✓ support (+ procedural + tools)
✓ system (+ rpc/runtime-api)
✓ timestamp
✓ transaction-payment (+ rpc + rpc/runtime-api)
✓ treasury
✓ try-runtime
✓ tx-pause
✓ utility
✓ vesting
```

## Pallets REMOVED (Not used by Selendra)

### Consensus & Block Production
```
✗ alliance
✗ authority-discovery
✗ babe
✗ beefy
✗ beefy-mmr
✗ grandpa
✗ im-online
```

### Asset Management
```
✗ asset-conversion
✗ asset-rate
✗ assets
✗ nft-fractionalization
✗ nfts (+ runtime-api)
✗ uniques
```

### Governance & Treasury
```
✗ bounties
✗ child-bounties
✗ conviction-voting
✗ core-fellowship
✗ ranked-collective
✗ referenda
✗ salary
✗ tips
✗ whitelist
```

### Staking & Elections
```
✗ bags-list (+ fuzzer + remote-tests)
✗ election-provider-multi-phase (+ test-staking-e2e)
✗ election-provider-support/benchmarking
✗ election-provider-support/solution-type/fuzzer
✗ fast-unstake
✗ nomination-pools/benchmarking
✗ nomination-pools/fuzzer
✗ nomination-pools/test-staking
✗ offences (+ benchmarking)
```

### Miscellaneous
```
✗ atomic-swap
✗ broker
✗ glutton
✗ indices
✗ insecure-randomness-collective-flip
✗ lottery
✗ membership
✗ merkle-mountain-range
✗ message-queue
✗ mixnet
✗ nicks
✗ nis
✗ node-authorization
✗ paged-list (+ fuzzer)
✗ recovery
✗ remark
✗ root-offences
✗ root-testing
✗ scored-pool
✗ session/benchmarking
✗ society
✗ state-trie-migration
✗ statement
✗ system/benchmarking
✗ transaction-payment/asset-conversion-tx-payment
✗ transaction-payment/asset-tx-payment
✗ transaction-payment/skip-feeless-payment
✗ transaction-storage
```

### Examples & Tests
```
✗ benchmarking/pov
✗ contracts/fixtures
✗ examples (all subdirectories):
  - examples/basic
  - examples/default-config
  - examples/dev-mode
  - examples/frame-crate
  - examples/kitchensink
  - examples/offchain-worker
  - examples/split
  - examples/tasks
✗ support/test
✗ support/test/compile_pass
✗ support/test/pallet
✗ support/test/stg_frame_crate
```

## Primitives Removed

### Test & Fuzzer Primitives
```
✗ primitives/api/test
✗ primitives/application-crypto/test
✗ primitives/arithmetic/fuzzer
✗ primitives/core/fuzz
✗ primitives/npos-elections/fuzzer
✗ primitives/runtime-interface/test
✗ primitives/runtime-interface/test-wasm
✗ primitives/runtime-interface/test-wasm-deprecated
```

### Unused Primitives
```
✗ primitives/merkle-mountain-range
✗ primitives/mixnet
✗ primitives/statement-store
✗ primitives/test-primitives
✗ primitives/transaction-storage-proof
```

## Utils & Infrastructure Removed

### Test Utilities
```
✗ substrate/test-utils (all)
✗ substrate/test-utils/cli
✗ substrate/test-utils/client
✗ substrate/test-utils/runtime
✗ substrate/test-utils/runtime/client
✗ substrate/test-utils/runtime/transaction-pool
```

### Build & Development Tools
```
✗ substrate/scripts/ci/node-template-release
✗ substrate/utils/binary-merkle-tree
✗ substrate/utils/frame/frame-utilities-cli
✗ substrate/utils/frame/generate-bags
✗ substrate/utils/frame/generate-bags/node-runtime
✗ substrate/utils/frame/remote-externalities
✗ substrate/utils/frame/rpc/client
✗ substrate/utils/frame/rpc/state-trie-migration-rpc
✗ substrate/utils/frame/try-runtime/cli
```

## Build Performance Impact

Before cleanup:
- Total workspace members: ~270+
- Frame pallets compiled: 122

After cleanup:
- Total workspace members: 156
- Frame pallets compiled: 42
- **Reduction: ~40-45% fewer crates to compile**

## Verification

To verify a pallet was removed:
```bash
cd vendors/polkadot-sdk
grep '"substrate/frame/PALLET_NAME"' Cargo.toml
# If not found, it's removed from the workspace
```

To see all included pallets:
```bash
cd vendors/polkadot-sdk
grep '"substrate/frame/' Cargo.toml
```
