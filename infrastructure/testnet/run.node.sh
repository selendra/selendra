#!/usr/bin/env bash

# Add signal handling for graceful shutdown
trap 'kill $(jobs -p); exit 0' SIGTERM SIGINT

# This script runs locally selendra-node consensus, ie set of selendra-node processes that are either
# validators (create blocks) or RPC nodes (read only nodes). By default, such consensus consist of
# one RPC node that is a bootnode, and 6 validator nodes.
#
# Before run, a chainspec is generated that is an initial testing Selendr chain configuration,
# that is a starting point for all selendra-nodes. Together with the chainspec, we generate a keystore
# that consist of two types of keys:
#   * two session keys for each validator (one for authoring blocks called AURA key and one for
#     participating in SelendraBFT consensus called SELENDRA key)
#   * one key for participating in P2P network called p2p key
# The keystore is stored in a filesystem altogether with database, and this is called base path.
# Optionally, script can build testing selendra-node binary for you (short session meaning 30 blocks
# instead of normal 900 blocks session).
#
# Each validator has also an associated account on chain called Stash Account which is used for
# staking. For simplicity, such accounts seeds are generated from cardinal numbers //0, //1, //2, ...
# We assume RPC nodes have ids //0, //1, //2, ... //N-1, where N is number of RPC nodes, and
# validators have ids //N, //N+1, ...
# Obviously, this is just for testing reasons and such seeds must be never used in production.
# Also, each SelendraNode chain has sudo account. Here we set it to //0 (the first RPC node account).
#
# Make sure a machine on which you're running your script has enough RAM memory. For testing nodes
# with empty db, a 1.5 GB per node would be enough.
#
# They are 3 set of ports you need to have opened and free on your machine in order to run consensus:
#  * RPC port - range [9944; 9954) - used for JSON RPC protocol
#  * P2p port - range [30333; 30343) - used for P2P peer network
#  * Validator port - range [30343; 30353) - used for consensus mechanism (SelendraBFT)
#
# You need to have installed following prerequisites in order to use that script:
#   * jq
#
# This script also accepts env variables instead of arguments, see --help for details. All arguments
# are optional.

set -euo pipefail

# ------------------------ constants --------------------------------------

SELENDRA_NODE="/usr/local/bin/selendra-node"
CHAINSPEC_GENERATOR="/usr/local/bin/chain-bootstrapper"
NODE_P2P_PORT_RANGE_START=30333
NODE_VALIDATOR_PORT_RANGE_START=30343
NODE_RPC_PORT_RANGE_START=9944
PROMETHEUS_PORT_RANGE_START=9615

# ------------------------ argument parsing and usage -----------------------

script_path="${BASH_SOURCE[0]}"
script_dir=$(dirname "${script_path}")
selendra_node_root_dir="/selendra"
pushd "${selendra_node_root_dir}" > /dev/null
source ./common.sh

function usage(){
  cat << EOF
Usage:
   $0
    [-v|--validators VALIDATORS]
      number of validators to bootstrap and start
    [-n|--rpc-nodes RPC_NODES]
      number of RPC nodes to bootstrap and start
    [-p|--base-path BASE_PATH]
        if specified, use given base path (keystore, db, AlephBFT backups)
        if not specified, base path is ./run-nodes-local
    [--finality-version]
      which finality version should be used, default = legacy
    [--dont-bootstrap]
      set if you don't want to bootstrap chain, ie generate keystore and chainspec
    [--dont-build]
       set if you do not want to build testing selendra-node binary
    [--dont-delete-db]
      set to not delete database
    [--dont-remove-abft-backups]
      set to not delete AlephBFT backups; by default they are removed since
      this script is intended to bootstrap chain by default, in which case you do not want to have
      them in 99% of scenarios
EOF
  exit 0
}

VALIDATORS=${VALIDATORS:-4}
RPC_NODES=${RPC_NODES:-1}
BASE_PATH=${BASE_PATH:-"./run-nodes-local"}
DONT_BOOTSTRAP=${DONT_BOOTSTRAP:-""}
DONT_BUILD_SELENDRA_NODE=${DONT_BUILD_SELENDRA_NODE:-""}
DONT_DELETE_DB=${DONT_DELETE_DB:-""}
DONT_REMOVE_ABFT_BACKUPS=${DONT_REMOVE_ABFT_BACKUPS:-""}
FINALITY_VERSION=${FINALITY_VERSION:-"current"}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -v|--validators)
      VALIDATORS="$2"
      shift;shift
      ;;
    -n|--rpc-nodes)
      RPC_NODES="$2"
      shift;shift
      ;;
    -p|--base-path)
      BASE_PATH="$2"
      shift;shift
      ;;
    --finality-version)
      FINALITY_VERSION="$2"
      shift;shift
      ;;
    --dont-bootstrap)
      DONT_BOOTSTRAP="true"
      shift
      ;;
    --dont-build)
      DONT_BUILD_SELENDRA_NODE="true"
      shift
      ;;
    --dont-delete-db)
      DONT_DELETE_DB="true"
      shift
      ;;
    --help)
      usage
      shift
      ;;
    --dont-remove-abft-backups)
      DONT_REMOVE_ABFT_BACKUPS="true"
      shift
      ;;
    *)
      error "Unrecognized argument $1!"
      ;;
  esac
done

# -----------------------  functions --------------------------------------
function get_backup_folders() {
  base_path="${1}"
  shift
  accounts_ids=("$@")

  declare -a backup_folders
  for account_id in ${accounts_ids[@]}; do
    maybe_backup_folder="${base_path}/${account_id}/backup-stash"
    if [[ -d "${maybe_backup_folder}" ]]; then
      backup_folders+=("${maybe_backup_folder}")
    fi
  done

  echo ${backup_folders[@]}
}

function get_ss58_address_from_seed() {
  local seed="$1"
  local selendra_node_path="$2"

  echo $(${selendra_node_path} key inspect --output-type json ${seed} | jq -r '.ss58Address')
}

function run_node() {
  local index="$1"
  local account_id="$2"
  local bootnode="$3"
  local is_rpc_node="$4"  # New parameter to distinguish RPC nodes from validators

  local node_name="node-${index}"
  local validator_port=$((NODE_VALIDATOR_PORT_RANGE_START + index))

  local node_args=(
    --validator
    --public-validator-addresses "0.0.0.0:${validator_port}"  # Listen on all interfaces
    --chain "${BASE_PATH}/chainspec.json"
    --bootnodes "${bootnode}"
    --base-path "${BASE_PATH}/${account_id}"
    --name "${node_name}"
    --rpc-port $((NODE_RPC_PORT_RANGE_START + index))
    --port $((NODE_P2P_PORT_RANGE_START + index))
    --prometheus-port $((PROMETHEUS_PORT_RANGE_START + index))
    --validator-port "${validator_port}"
    --node-key-file "${BASE_PATH}/${account_id}/p2p_secret"
    --backup-path "${BASE_PATH}/${account_id}/backup-stash"
    --no-mdns
    --pool-limit 1024
    --db-cache 1024
    --runtime-cache-size 2
    --max-runtime-instances 8
    --enable-log-reloading
    --detailed-log-output
    -laleph-party=debug
    -laleph-network=debug
    -lnetwork-clique=debug
    -laleph-finality=debug
    -laleph-justification=debug
    -laleph-data-store=debug
    -laleph-updater=debug
    -laleph-metrics=debug
    -laleph-abft=debug
  )

  # Only add RPC external access for RPC nodes, not validators
  if [[ "${is_rpc_node}" == "true" ]]; then
    node_args+=(--unsafe-rpc-external)
    node_args+=(--rpc-cors=all)
  fi

  info "Running node ${index}..."
  "${SELENDRA_NODE}" "${node_args[@]}"  2> "${BASE_PATH}/${node_name}.log" > /dev/null &
}

# ------------------------- input checks ----------------------------------

if [[ "${VALIDATORS}" -lt 1 ]]; then
  error "Number of validators should be at least 1!"
fi
if [[ "${VALIDATORS}" -lt 4 ]]; then
  warning "AlephBFT is only supported for more than 4 nodes."
fi
if [[ "${RPC_NODES}" -lt 1 ]]; then
  error "Number of RPC nodes should be at least 1!"
fi
if [[ $(( VALIDATORS + RPC_NODES )) -gt 10 ]]; then
  info "Current number of validators is ${VALIDATORS} and RPC nodes is ${RPC_NODES}"
  error "Total number of validators and rpc nodes should not be greater than 10!"
fi
if [[ -z "${DONT_BOOTSTRAP}" && "${DONT_DELETE_DB}" == "true" ]]; then
  error "--dont-delete-db is set and --dont-bootstrap is not set
        When bootstraping chain, db must be deleted!
        Or if you want just to remove database, pass --dont-bootstrap to this script."
fi
if [[ "${DONT_BOOTSTRAP}" == "true" && ! -f "${BASE_PATH}/chainspec.json" ]]; then
  error "Flag --dont-bootstrap is set and there is no ${BASE_PATH}/chainspec.json file, maybe you
        forget to bootstrap chain?"
fi
if ! command -v jq &> /dev/null; then
    error "jq could not be found on PATH!"
fi
if [[ "${FINALITY_VERSION}" != "current" && "${FINALITY_VERSION}" != "legacy" ]]; then
  error "Flag finality-version should be either current or legacy."
fi

# ------------------- main script starts here ------------------------------

info "Starting $0"
info "Creating base path ${BASE_PATH} if it does not exist"
mkdir -p "${BASE_PATH}"
info "Stopping all current node processes"
if ! killall -9 selendra-node 2> /dev/null; then
  info "No selendra-node processes found."
fi

if [[ -z "${DONT_BUILD_SELENDRA_NODE}" ]]; then
  info "Using pre-built selendra-node and chain-bootstrapper binaries."
  # Skip building since we're using pre-built binaries
elif [[ ! -x "${SELENDRA_NODE}" || ! -x "${CHAINSPEC_GENERATOR}" ]]; then
  error "${SELENDRA_NODE} or ${CHAINSPEC_GENERATOR} does not exist or it's not an executable file!"
fi

NUMBER_OF_NODES_TO_BOOTSTRAP=$(( VALIDATORS + RPC_NODES ))
info "Generating ${NUMBER_OF_NODES_TO_BOOTSTRAP} stash accounts identities."
declare -a rpc_node_account_ids
for i in $(seq 0 "$(( RPC_NODES - 1 ))"); do
  rpc_node_account_ids+=($(get_ss58_address_from_seed "//${i}" "${SELENDRA_NODE}"))
  # rpc_node_account_ids="5DM7PJEFPbcYViEzFXu5GjF96JgoSJ3rb6jfXLsmXqrPVG2o"
done

declare -a validator_account_ids
for i in $(seq "${RPC_NODES}" "$(( NUMBER_OF_NODES_TO_BOOTSTRAP - 1 ))"); do
  validator_account_ids+=($(get_ss58_address_from_seed "//${i}" "${SELENDRA_NODE}"))
done

# # Hardcoded list of addresses (must match or exceed VALIDATORS)
# validator_account_ids=(
#   "5G1MS9ewwQVJsQE8HRPeHLcxSRabQefq7eSAHXpPpq5noxZT"
#   "5CGVrJDrC1Ey2wRwmH55dWXzLYab8yqtTFNX3oNuDoyDPVrb"
#   "5CaSeVW9EEpZg6ktQMQAj1Jj6QnE7w8k3BgBTrgwcQQxBZWR"
#   "5EeqTNH1DCJYmwbhNnCr2jnGeqH5WZ4G9SXVxPzDPfGV9mVH"
# )

# Check if there are enough addresses
if [[ ${#validator_account_ids[@]} -lt ${VALIDATORS} ]]; then
  error "Not enough hardcoded validator account IDs for ${VALIDATORS} validators!"
fi
# Trim the array to the number of validators needed
validator_account_ids=("${validator_account_ids[@]:0:${VALIDATORS}}")

info "Following identities were generated:"
info "RPC nodes: ${rpc_node_account_ids[@]}"
info "Validator nodes: ${validator_account_ids[@]}"

# Log all test account details for easy access in cloud dashboard
info "=========================================="
info "TEST ACCOUNT DETAILS:"
info "=========================================="
for i in $(seq 0 "$(( NUMBER_OF_NODES_TO_BOOTSTRAP - 1 ))"); do
  info "Account //${i}:"
  "${SELENDRA_NODE}" key inspect //${i} | grep -E "(Secret seed|Public key|SS58 Address)"
  info "------------------------------------------"
done
info "=========================================="

if [[ -z "${DONT_BOOTSTRAP}" ]]; then
  info "Bootstrapping chain for ${NUMBER_OF_NODES_TO_BOOTSTRAP} nodes."

  all_account_ids=(${validator_account_ids[@]} ${rpc_node_account_ids[@]})
  # space separated ids
  all_account_ids_string="${all_account_ids[*]}"
  # comma separated ids
  all_account_ids_string="${all_account_ids_string//${IFS:0:1}/,}"

  # space separated ids
  validator_ids_string="${validator_account_ids[*]}"
  # comma separated ids
  validator_ids_string="${validator_ids_string//${IFS:0:1}/,}"

  info "Populating keystore for all accounts with session keys and libp2p key, and generating chainspec"
  # "${CHAINSPEC_GENERATOR}" bootstrap-chain \
    # --raw \
    # --base-path "${BASE_PATH}" \
    # --account-ids "${all_account_ids_string}" \
    # --authorities-account-ids "${validator_ids_string}" \
    # --token-symbol "SEL" \
    # --chain-id "selendra" \
    # --chain-name "Selendra Network" \
    # --chain-type live > "${BASE_PATH}/chainspec.json" \
    # --sudo-account-id 5G1MS9ewwQVJsQE8HRPeHLcxSRabQefq7eSAHXpPpq5noxZT \
    # --rich-account-ids "${all_account_ids_string}" \
    # --finality-version "${FINALITY_VERSION}"

  # Set sudo account to the first account (//0)
  sudo_account_id="${rpc_node_account_ids[0]}"
  info "Setting sudo account to: ${sudo_account_id}"

  # Log sudo account details for easy access in cloud dashboard
  info "=========================================="
  info "SUDO ACCOUNT DETAILS (//0):"
  info "=========================================="
  "${SELENDRA_NODE}" key inspect //0
  info "=========================================="

  "${CHAINSPEC_GENERATOR}" bootstrap-chain \
    --raw \
    --base-path "${BASE_PATH}" \
    --account-ids "${all_account_ids_string}" \
    --authorities-account-ids "${validator_ids_string}" \
    --chain-type live \
    --chain-id "selendra-testnet" \
    --chain-name "Selendra Testnet" \
    --token-symbol "SEL" \
    --sudo-account-id "${sudo_account_id}" \
    --rich-account-ids "${validator_ids_string}" \
    --finality-version "${FINALITY_VERSION}" > "${BASE_PATH}/chainspec.json"

  if [[ "${DONT_REMOVE_ABFT_BACKUPS}" == "true" ]]; then
    all_account_ids=(${validator_account_ids[@]} ${rpc_node_account_ids[@]})
    non_empty_backups=$(get_backup_folders "${BASE_PATH}" ${all_account_ids[@]})
    if [[ -n "${non_empty_backups}" ]]; then
      warning "Found following non-empty ABFT backups in base path:"
      warning "${non_empty_backups}"
      warning "In 99% you want them to be removed when bootstraping chain"
      warning "Re-run this script without flag --dont-remove-abtf-backups if you want them to be removed."
    fi
  fi
fi

info "Creating bootnode p2p multiaddress."
p2p_key_public=$("${SELENDRA_NODE}" key inspect-node-key --file "${BASE_PATH}/${rpc_node_account_ids[0]}/p2p_secret")
bootnode_multiaddress="/dns4/localhost/tcp/$((NODE_P2P_PORT_RANGE_START))/p2p/${p2p_key_public}"
info "Bootnode p2p multiaddress is ${bootnode_multiaddress}"

if [[ -z "${DONT_DELETE_DB}" ]] ; then
  info "Removing database for all nodes (aka purging chain)."
  for i in $(seq 0 "$(( RPC_NODES - 1 ))"); do
    rpc_node_account_id=${rpc_node_account_ids[$i]}
    "${SELENDRA_NODE}" purge-chain --base-path "${BASE_PATH}/${rpc_node_account_id}" --chain "${BASE_PATH}/chainspec.json" -y > /dev/null 2>&1
  done
  for i in $(seq 0 "$(( VALIDATORS - 1 ))"); do
    validator_account_id="${validator_account_ids[$i]}"
    "${SELENDRA_NODE}" purge-chain --base-path "${BASE_PATH}/${validator_account_id}" --chain "${BASE_PATH}/chainspec.json" -y > /dev/null 2>&1
  done
fi

if [[ -z "${DONT_REMOVE_ABFT_BACKUPS}" ]]; then
  all_account_ids=(${validator_account_ids[@]} ${rpc_node_account_ids[@]})
  backups=$(get_backup_folders "${BASE_PATH}" ${all_account_ids[@]})
  if [[ "${backups[@]}" ]]; then
    info "Removing AlephBFT backups."
    echo "${backups[@]}" | xargs rm -rf
  fi
fi

for i in $(seq 0 "$(( RPC_NODES - 1 ))"); do
  rpc_node_account_id=${rpc_node_account_ids[$i]}
  run_node "$i" "${rpc_node_account_id}" "${bootnode_multiaddress}" "true"
done

for i in $(seq 0 "$(( VALIDATORS - 1 ))"); do
  validator_account_id=${validator_account_ids[$i]}
  run_node $(( i + RPC_NODES )) "${validator_account_id}" "${bootnode_multiaddress}" "false"
done

popd > /dev/null
info "All nodes started. Waiting for processes..."

# Keep the script running and handle signals
while true; do
    sleep 1
    # Check if any selendra-node processes are still running
    if ! pgrep -f selendra-node > /dev/null; then
        error "All selendra-node processes died, exiting..."
        exit 1
    fi
done
