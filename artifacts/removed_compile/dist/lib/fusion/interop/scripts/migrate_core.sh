#!/usr/bin/env bash
set -euo pipefail

log() {
  local level="$1"; shift
  printf "[%s] [%s] %s\n" "$(date +%s)" "$level" "$*"
}

ROOT_DIR="${1:-/opt/fusion}"
BACKUP_DIR="${ROOT_DIR}/backup"
BIN_DIR="${ROOT_DIR}/bin"
CONF_FILE="${ROOT_DIR}/Fusion.toml"
RUNTIME_BIN_NEW="${ROOT_DIR}/runtime_v3"
RUNTIME_BIN_ACTIVE="${BIN_DIR}/fusion-runtime"

log INFO "Starting Fusion core migration (v2 -> v3)"

if [[ ! -f "${RUNTIME_BIN_NEW}" ]]; then
  log ERROR "Missing new runtime binary at ${RUNTIME_BIN_NEW}"
  exit 1
fi

mkdir -p "${BACKUP_DIR}"
SNAPSHOT="${BACKUP_DIR}/runtime_backup_$(date +%Y%m%d_%H%M%S).tar.gz"

log INFO "Creating backup: ${SNAPSHOT}"
tar -czf "${SNAPSHOT}" -C "${ROOT_DIR}" bin Fusion.toml

log INFO "Stopping fusion service"
if command -v systemctl >/dev/null 2>&1; then
  systemctl stop fusion || true
fi

log INFO "Installing new runtime binary"
cp "${RUNTIME_BIN_NEW}" "${RUNTIME_BIN_ACTIVE}"
chmod +x "${RUNTIME_BIN_ACTIVE}"

log INFO "Patching Fusion.toml for Supernova"
if [[ -f "${CONF_FILE}" ]]; then
  sed -i 's/runtime_version\s*=.*/runtime_version = "v3-supernova"/' "${CONF_FILE}" || true
  sed -i 's/enable_io_uring\s*=.*/enable_io_uring = true/' "${CONF_FILE}" || true
fi

log INFO "Starting fusion service"
if command -v systemctl >/dev/null 2>&1; then
  systemctl start fusion || true
fi

log INFO "Running health check"
if [[ -x "${ROOT_DIR}/bin/fusion-health" ]]; then
  if ! "${ROOT_DIR}/bin/fusion-health"; then
    log ERROR "Health check failed, rolling back"
    tar -xzf "${SNAPSHOT}" -C "${ROOT_DIR}"
    if command -v systemctl >/dev/null 2>&1; then
      systemctl restart fusion || true
    fi
    exit 1
  fi
fi

log INFO "Migration complete"
