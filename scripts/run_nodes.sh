#!/bin/bash

function usage(){
  echo "Usage:
      ./run_nodes.sh [-v N_VALIDATORS] [-n N_NON_VALIDATORS] [-b false] [-p BASE_PATH] [-l N_LISTENERES] [SELENDRA_NODE_ARG]...
  where 2 <= N_VALIDATORS <= N_VALIDATORS + N_NON_VALIDATORS + N_LISTENERES <= 10
  (by default, N_VALIDATORS=4, N_NON_VALIDATORS=0, N_LISTENERES=0 and BASE_PATH=/tmp)"
}

N_VALIDATORS=4
N_NON_VALIDATORS=0
N_LISTENERES=0
BUILD_SELENDRA_NODE='true'
BASE_PATH='/tmp'

while getopts "v:n:b:p:l:" flag
do
  case "${flag}" in
    v) N_VALIDATORS=${OPTARG};;
    n) N_NON_VALIDATORS=${OPTARG};;
    b) BUILD_SELENDRA_NODE=${OPTARG};;
    p) BASE_PATH=${OPTARG};;
    l) N_LISTENERES=${OPTARG};;
    *)
      usage
      exit
      ;;
  esac
done

shift $((OPTIND-1))

killall -9 selendra-node

set -e

clear


if $BUILD_SELENDRA_NODE ; then
  cargo build --release
fi

declare -a account_ids
for i in $(seq 0 "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 ))"); do
  account_ids+=($(./target/release/selendra-node key inspect "//$i" | grep "SS58 Address:" | awk '{print $3;}'))
done
validator_ids=("${account_ids[@]::N_VALIDATORS}")
# space separated ids
validator_ids_string="${validator_ids[*]}"
# comma separated ids
validator_ids_string="${validator_ids_string//${IFS:0:1}/,}"

echo "Bootstrapping chain for nodes 0..$((N_VALIDATORS - 1))"
./target/release/selendra-node bootstrap-chain --raw --base-path "$BASE_PATH" --account-ids "$validator_ids_string" --chain-type local > "$BASE_PATH/chainspec.json"

for i in $(seq "$N_VALIDATORS" "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 ))"); do
  echo "Bootstrapping node $i"
  account_id=${account_ids[$i]}
  ./target/release/selendra-node bootstrap-node --base-path "$BASE_PATH/$account_id" --account-id "$account_id" --chain-type local
done

addresses=()
for i in $(seq 0 "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 ))"); do
    pk=$(./target/release/selendra-node key inspect-node-key --file $BASE_PATH/${account_ids[$i]}/p2p_secret)
    addresses+=("/dns4/localhost/tcp/$((30334+i))/p2p/$pk")
done

bootnodes=""
for i in 0 1; do
    bootnodes+=${addresses[i]}
done

run_node() {
  i=$1
  is_validator=$2
  auth=node-$i
  account_id=${account_ids[$i]}
  validator_port=$((30343 + i))

  [[ $is_validator = true ]] && validator=--validator || validator=""

  ./target/release/selendra-node purge-chain --base-path $BASE_PATH/$account_id --chain $BASE_PATH/chainspec.json -y
  ./target/release/selendra-node \
    $validator \
    --chain $BASE_PATH/chainspec.json \
    --base-path $BASE_PATH/$account_id \
    --name $auth \
    --rpc-port $((9933 + i)) \
    --ws-port $((9944 + i)) \
    --port $((30334 + i)) \
    --bootnodes $bootnodes \
    --node-key-file $BASE_PATH/$account_id/p2p_secret \
    --backup-path $BASE_PATH/$account_id/backup-stash \
    --unit-creation-delay 500 \
    --execution Native \
    --rpc-cors=all \
    --no-mdns \
    --public-validator-addresses 127.0.0.1:${validator_port} \
    --validator-port ${validator_port} \
    2> $auth.log > /dev/null & \
}

for i in $(seq 0 "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 ))"); do
  run_node "$i" true
done

for i in $(seq "$(( N_VALIDATORS + N_NON_VALIDATORS))" "$(( N_VALIDATORS + N_NON_VALIDATORS - 1 + N_LISTENERES))"); do
  run_node "$i" false
done