#!/usr/bin/env bash

# Stop an extra validator node started by scripts/run_extra_validator.sh
# It prefers stopping via the PID file written at BASE_PATH/<name>.pid
# and falls back to searching by process arguments if needed.

set -euo pipefail

NAME=${NAME:-""}
INDEX=${INDEX:-""}
ACCOUNT_ID=${ACCOUNT_ID:-""}
BASE_PATH=${BASE_PATH:-"./run-nodes-local"}

script_path="${BASH_SOURCE[0]}"
script_dir=$(dirname "${script_path}")
selendra_node_root_dir=$(realpath "${script_dir}/..")
pushd "${selendra_node_root_dir}" > /dev/null
source ./scripts/common.sh

usage() {
  cat << EOF
Usage: $0 [options]
  --name NAME             Name used when starting the node (e.g., extra-validator-5)
  --index N               If --name not given, defaults to extra-validator-N
  --account-id SS58       Optional, used for fallback matching if no PID file
  --base-path PATH        Base path (default: ${BASE_PATH})
  -h|--help               Show this help

Examples:
  $0 --name extra-validator-5
  $0 --index 6
  $0 --account-id 5F... --base-path ./run-nodes-local
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --name) NAME="$2"; shift; shift ;;
    --index) INDEX="$2"; shift; shift ;;
    --account-id) ACCOUNT_ID="$2"; shift; shift ;;
    --base-path) BASE_PATH="$2"; shift; shift ;;
    -h|--help) usage; exit 0 ;;
    *) error "Unknown argument: $1" ;;
  esac
done

if [[ -z "${NAME}" && -n "${INDEX}" ]]; then
  NAME="extra-validator-${INDEX}"
fi

if [[ -z "${NAME}" ]]; then
  error "Provide --name or --index"
fi

pid_file="${BASE_PATH}/${NAME}.pid"
if [[ -f "${pid_file}" ]]; then
  pid=$(cat "${pid_file}")
  if kill -0 "${pid}" 2>/dev/null; then
    info "Stopping ${NAME} (PID ${pid})"
    kill "${pid}" || true
    # wait a little and escalate if still running
    for i in {1..10}; do
      if kill -0 "${pid}" 2>/dev/null; then
        sleep 0.5
      else
        break
      fi
    done
    if kill -0 "${pid}" 2>/dev/null; then
      warning "Process still alive, sending SIGKILL"
      kill -9 "${pid}" || true
    fi
    rm -f "${pid_file}"
    info "Stopped ${NAME}"
    popd > /dev/null
    exit 0
  else
    warning "PID in ${pid_file} is not running; removing stale pid file"
    rm -f "${pid_file}"
  fi
fi

# Fallback: try to find by process arguments
pids=$(pgrep -f "selendra-node .*--name ${NAME}" || true)
if [[ -z "${pids}" && -n "${ACCOUNT_ID}" ]]; then
  pids=$(pgrep -f "selendra-node .*(--base-path|--base\-path) ${BASE_PATH}/${ACCOUNT_ID}" || true)
fi

if [[ -z "${pids}" ]]; then
  error "Could not find running process for ${NAME}. If it was started with a different name, pass --account-id for fallback."
fi

info "Stopping ${NAME} (PID(s): ${pids})"
kill ${pids} || true
sleep 1
# Force kill if needed
still=$(echo ${pids} | xargs -n1 -I{} bash -c 'kill -0 {} 2>/dev/null && echo {}' | xargs || true)
if [[ -n "${still}" ]]; then
  warning "Some PIDs still alive, sending SIGKILL: ${still}"
  kill -9 ${still} || true
fi
info "Stopped ${NAME}"

popd > /dev/null
