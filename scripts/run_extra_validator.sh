#!/usr/bin/env bash

# Run a single additional validator node against an already running local network.
# Useful for testing "add validator" flows without re-bootstrapping the whole chain.
#
# Requirements:
# - The local network was started previously (e.g., with scripts/run_nodes.sh)
# - A chainspec exists at BASE_PATH/chainspec.json
# - selendra-node and chain-bootstrapper are built (target/release)
#
# This script can generate the keystore (session keys + p2p key) for the new validator
# and then launch selendra-node with non-conflicting ports.

set -euo pipefail

# ------------------------ defaults --------------------------------------
SELENDRA_NODE="target/release/selendra-node"
CHAINSPEC_GENERATOR="target/release/chain-bootstrapper"
BASE_PATH=${BASE_PATH:-"./run-nodes-local"}
INDEX=${INDEX:-5}                    # used to derive ports if not explicitly set
NAME=${NAME:-"extra-validator-${INDEX}"}
SEED=${SEED:-""}                     # e.g. //5 or //Extra
ACCOUNT_ID=${ACCOUNT_ID:-""}         # ss58 address; if empty and SEED provided, we'll derive
BOOTNODE_MULTIADDR=${BOOTNODE_MULTIADDR:-""}
GENERATE_KEYS=${GENERATE_KEYS:-"true"}
FINALITY_VERSION=${FINALITY_VERSION:-"current"}
PRINT_SESSION_KEY=${PRINT_SESSION_KEY:-"true"}

NODE_P2P_PORT_RANGE_START=${NODE_P2P_PORT_RANGE_START:-30333}
NODE_VALIDATOR_PORT_RANGE_START=${NODE_VALIDATOR_PORT_RANGE_START:-30343}
NODE_RPC_PORT_RANGE_START=${NODE_RPC_PORT_RANGE_START:-9944}
PROMETHEUS_PORT_RANGE_START=${PROMETHEUS_PORT_RANGE_START:-9615}

RPC_PORT=${RPC_PORT:-$((NODE_RPC_PORT_RANGE_START + INDEX))}
P2P_PORT=${P2P_PORT:-$((NODE_P2P_PORT_RANGE_START + INDEX))}
VALIDATOR_PORT=${VALIDATOR_PORT:-$((NODE_VALIDATOR_PORT_RANGE_START + INDEX))}
PROMETHEUS_PORT=${PROMETHEUS_PORT:-$((PROMETHEUS_PORT_RANGE_START + INDEX))}

script_path="${BASH_SOURCE[0]}"
script_dir=$(dirname "${script_path}")
selendra_node_root_dir=$(realpath "${script_dir}/..")
pushd "${selendra_node_root_dir}" > /dev/null
source ./scripts/common.sh

usage() {
  cat << EOF
Usage: $0 [options]
  --seed SEED                 Seed phrase for the validator (e.g., //5). If set, ACCOUNT_ID is derived.
  --account-id SS58           SS58 address of the validator. Overrides --seed derivation if provided.
                              If neither --seed nor --account-id is provided, a random account will be generated.
  --index N                   Port index to avoid conflicts (defaults to ${INDEX}).
  --base-path PATH            Base path with chainspec and node data (default: ${BASE_PATH}).
  --name NAME                 Node name (default: ${NAME}).
  --bootnode-multiaddr ADDR   Bootnode multiaddress. If empty, will try to infer from BASE_PATH.
  --no-generate-keys          Do not generate keystore/p2p key (assumes already present).
  --finality-version VER      Finality version for key generation (current|legacy). Default: ${FINALITY_VERSION}
  --no-print-session-key      Do not query and print session key after start (default prints).
  --rpc-port PORT             RPC port (default: ${RPC_PORT}).
  --p2p-port PORT             P2P port (default: ${P2P_PORT}).
  --validator-port PORT       Validator port (default: ${VALIDATOR_PORT}).
  --prometheus-port PORT      Prometheus port (default: ${PROMETHEUS_PORT}).
  -h|--help                   Show this help.

Environment overrides: BASE_PATH, INDEX, NAME, SEED, ACCOUNT_ID, BOOTNODE_MULTIADDR, GENERATE_KEYS,
                       NODE_*_RANGE_START, *_PORT, FINALITY_VERSION
EOF
}

# ------------------------ arg parsing -----------------------------------
while [[ $# -gt 0 ]]; do
  case "$1" in
    --seed) SEED="$2"; shift; shift ;;
    --account-id) ACCOUNT_ID="$2"; shift; shift ;;
    --index) INDEX="$2"; shift; shift ;;
    --base-path) BASE_PATH="$2"; shift; shift ;;
    --name) NAME="$2"; shift; shift ;;
    --bootnode-multiaddr) BOOTNODE_MULTIADDR="$2"; shift; shift ;;
    --no-generate-keys) GENERATE_KEYS=""; shift ;;
    --finality-version) FINALITY_VERSION="$2"; shift; shift ;;
    --no-print-session-key) PRINT_SESSION_KEY=""; shift ;;
    --rpc-port) RPC_PORT="$2"; shift; shift ;;
    --p2p-port) P2P_PORT="$2"; shift; shift ;;
    --validator-port) VALIDATOR_PORT="$2"; shift; shift ;;
    --prometheus-port) PROMETHEUS_PORT="$2"; shift; shift ;;
    -h|--help) usage; exit 0 ;;
    *) error "Unknown argument: $1" ;;
  esac
done

# ------------------------ validations -----------------------------------
if [[ ! -f "${SELENDRA_NODE}" ]]; then
  error "${SELENDRA_NODE} not found. Build the node first (cargo build --release -p selendra-node)."
fi
if [[ ! -f "${CHAINSPEC_GENERATOR}" ]]; then
  error "${CHAINSPEC_GENERATOR} not found. Build it (cargo build --release -p chain-bootstrapper)."
fi
if [[ ! -f "${BASE_PATH}/chainspec.json" ]]; then
  error "chainspec.json not found at ${BASE_PATH}. Start base network first or point --base-path correctly."
fi
if ! command -v jq &> /dev/null; then
  error "jq could not be found on PATH!"
fi
if [[ -n "${PRINT_SESSION_KEY}" ]] && ! command -v curl &> /dev/null; then
  error "curl could not be found on PATH!"
fi

mkdir -p "${BASE_PATH}"

# Derive account-id from seed if needed
if [[ -z "${ACCOUNT_ID}" && -n "${SEED}" ]]; then
  ACCOUNT_ID=$(${SELENDRA_NODE} key inspect --output-type json ${SEED} | jq -r '.ss58Address')
fi

# If neither provided, generate a random account and use it
if [[ -z "${ACCOUNT_ID}" && -z "${SEED}" ]]; then
  info "No --account-id or --seed provided; generating a random account."
  GEN_JSON=$(${SELENDRA_NODE} key generate --output-type json)
  ACCOUNT_ID=$(echo "${GEN_JSON}" | jq -r '.ss58Address')
  SECRET_PHRASE=$(echo "${GEN_JSON}" | jq -r '.secretPhrase // empty')
  info "Generated account SS58: ${ACCOUNT_ID}"
  if [[ -n "${SECRET_PHRASE}" ]]; then
    info "Secret phrase (keep safe): ${SECRET_PHRASE}"
  fi
fi

if [[ -z "${ACCOUNT_ID}" ]]; then
  error "Provide --account-id SS58 or --seed to derive it."
fi

# Optionally generate keystore and p2p key for this account under BASE_PATH/ACCOUNT_ID
if [[ -n "${GENERATE_KEYS}" ]]; then
  info "Generating keystore and p2p key for ${ACCOUNT_ID} at ${BASE_PATH}/${ACCOUNT_ID}"
  # We call bootstrapper only to write keystore and p2p key. We ignore the stdout chainspec.
  ${CHAINSPEC_GENERATOR} bootstrap-chain \
    --raw \
    --base-path "${BASE_PATH}" \
    --account-ids "${ACCOUNT_ID}" \
    --authorities-account-ids "${ACCOUNT_ID}" \
    --chain-type local \
    --finality-version "${FINALITY_VERSION}" \
    > /dev/null
fi

# Determine bootnode multiaddr if not provided
if [[ -z "${BOOTNODE_MULTIADDR}" ]]; then
  # Try to find any existing p2p_secret under BASE_PATH/*/p2p_secret and assume its port is 30333
  maybe_secret=$(find "${BASE_PATH}" -maxdepth 2 -type f -name p2p_secret | head -n 1 || true)
  if [[ -n "${maybe_secret}" ]]; then
    pubkey=$(${SELENDRA_NODE} key inspect-node-key --file "${maybe_secret}")
    # Default to localhost and default first p2p port; adjust if needed
    BOOTNODE_MULTIADDR="/dns4/localhost/tcp/${NODE_P2P_PORT_RANGE_START}/p2p/${pubkey}"
    info "Inferred bootnode multiaddress: ${BOOTNODE_MULTIADDR}"
  else
    error "Could not infer bootnode address. Provide --bootnode-multiaddr explicitly."
  fi
fi

# ------------------------ run node --------------------------------------
node_args=(
  --validator
  --public-validator-addresses "127.0.0.1:${VALIDATOR_PORT}"
  --chain "${BASE_PATH}/chainspec.json"
  --bootnodes "${BOOTNODE_MULTIADDR}"
  --base-path "${BASE_PATH}/${ACCOUNT_ID}"
  --name "${NAME}"
  --rpc-port "${RPC_PORT}"
  --port "${P2P_PORT}"
  --prometheus-port "${PROMETHEUS_PORT}"
  --validator-port "${VALIDATOR_PORT}"
  --node-key-file "${BASE_PATH}/${ACCOUNT_ID}/p2p_secret"
  --backup-path "${BASE_PATH}/${ACCOUNT_ID}/backup-stash"
  --rpc-cors=all
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

log_file="${BASE_PATH}/${NAME}.log"
info "Starting extra validator ${NAME} (account ${ACCOUNT_ID})"
"${SELENDRA_NODE}" "${node_args[@]}" 2> "${log_file}" > /dev/null &
pid=$!
echo "${pid}" > "${BASE_PATH}/${NAME}.pid"
info "Launched (PID: ${pid}). Logs: ${log_file}"
info "To stop: ./scripts/stop_extra_validator.sh --name '${NAME}' --base-path '${BASE_PATH}'"

# Optionally query and print session key via RPC (author_rotateKeys)
if [[ -n "${PRINT_SESSION_KEY}" ]]; then
  info "Waiting for RPC (port ${RPC_PORT}) to become ready..."
  ready=""
  for i in {1..30}; do
    if curl -s -H "Content-Type: application/json" \
         -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
         "http://127.0.0.1:${RPC_PORT}" | jq -e '.result' > /dev/null; then
      ready="yes"; break
    fi
    sleep 1
  done
  if [[ -z "${ready}" ]]; then
    warning "RPC not ready yet; skipping session key query."
  else
    info "Querying session key with author_rotateKeys (this rotates local keys)."
    resp=$(curl -s -H "Content-Type: application/json" \
                 -d '{"id":1,"jsonrpc":"2.0","method":"author_rotateKeys","params":[]}' \
                 "http://127.0.0.1:${RPC_PORT}")
    session_key=$(echo "$resp" | jq -r '.result // empty')
    if [[ -n "${session_key}" && "${session_key}" != "null" ]]; then
      echo
      echo "Session key (author_rotateKeys): ${session_key}"
      echo "Use session.setKeys(keys=${session_key}, proof=0x00) on-chain for this validator."
      echo
    else
      warning "author_rotateKeys did not return a key. Raw response: $resp"
    fi
  fi
fi

popd > /dev/null
