#!/bin/bash

N_VALIDATORS=4
N_NON_VALIDATORS=0
N_LISTENERES=0
BUILD_SELENDRA_NODE='true'
BASE_PATH="$PWD/local"

shift $((OPTIND-1))

set -e

clear


if $BUILD_SELENDRA_NODE ; then
  cargo build --release
fi

## declare an array variable
declare -a mnemonics=(
  "auction praise leg remember critic amazing nurse couple motor seed spoon blame"
  "collect text boring shy misery refuse pipe junk two extend donate beef"
  "label cloud current air festival better anxiety witness judge dragon athlete barrel"
  "doll clump glue trap napkin double oval orbit kitchen inside syrup saddle"
)

declare -a account_ids
for i in $(seq 0 "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 ))"); do
  account_ids+=($(./target/release/selendra key inspect "${mnemonics[$i]}" | grep "SS58 Address:" | awk '{print $3;}')) 
done
validator_ids=("${account_ids[@]::N_VALIDATORS}")
# space separated ids
validator_ids_string="${validator_ids[*]}"
# comma separated ids
validator_ids_string="${validator_ids_string//${IFS:0:1}/,}"

echo "Bootstrapping chain for nodes 0..$((N_VALIDATORS - 1))"
./target/release/selendra bootstrap-chain --raw --base-path "$BASE_PATH" --account-ids "$validator_ids_string" --chain-type local > "$BASE_PATH/chainspec.json"