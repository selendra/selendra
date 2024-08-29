## Fork-Off tool

The tool will perform the following actions, in this order:
1. Download the whole state (key-value pairs) of the chain via the provided rpc endpoint `ws-rpc-endpoint`. More specifically it will first query the best block and then download the state at this block.
2. Dump the state to a json file. You can provide a path via `--snapshot-path`.
3. Read the state from the snapshot json file. This is because steps 1. and 2. can be omitted by running with `--use-snapshot-file` -- see example below.
4. Read the chainspec provided via `--initial-spec-path` you should pass here the one generated via `the bootstrap-chain` command, so `--initial-spec-path=chainspec.json` if it is in the same directory.
5. Replace the genesis state in the chainspec by the one from the snapshot WITH THE EXCEPTION of states of paths provided via a comma separated list using `--storage-keep-state`. The default setting is `--storage-keep-state=Aura,Aleph,Sudo,Staking,Session,Elections` and it's likely you don't want to change it.
6. If you have passed `--accounts-path` pointing to a file with a configuration for some accounts, then it will be written into chainspec (`System.Account` map). For an example see `AccountInfo-Template.json` file.
7. Alternatively to `--accounts-path` you can just pass `--balances` flag with which you can specify initial free balances for some accounts. Similarly, it will be saved to `System.Account` map.
8. The final, new chainspec is saved to the path provided via `--combined-spec-path`.

Note: `fork-off` expects address as an SS58 public key. 
For Alice it would be `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`.

So for instance to generate a new spec keeping the storage of testnet (note that in that case you should use the same binary as running on testnet to `bootstrap-chain`) we would run:


```bash
./target/release/fork-off \
  --ws-rpc-endpoint=wss://rpc.selendra.org \
  --initial-spec-path=chainspec.json \
  --combined-spec-path=combined.json \
  --storage-keep-state=Aura,Aleph,Sudo,Staking,Session,Elections,Balances,Ethereum,Evm,Identity,Multisig,Recovery
```
This will also create a `snapshot.json` file containing the state downloaded from testnet. In case the state downloaded correctly (easy to see from logs) but something went wrong when combining the specs (e.g. you want to use a different set of paths) then you can rerun without the need of downloading the state again (it might be time consuming):

```bash
target/release/fork-off \
  --ws-rpc-endpoint=wss://rpc.selendra.org \
  --initial-spec-path=chainspec.json \
  --combined-spec-path=combined.json \
  --use-snapshot-file
```

Finally, there is also an optional parameter `--max-requests` with a default value of `1000` which you can tweak to allow more/less concurrent in-flight requests while the state is downloading. Note that this might influence the risk of being banned for too many RPC requests, so use with caution. The default value seems to be safe.
