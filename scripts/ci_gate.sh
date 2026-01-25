#!/usr/bin/env bash
set -euo pipefail
./scripts/security_gate.sh
./scripts/compiler_gates.sh
./scripts/runtime_gates.sh
./scripts/run_fuc_fixtures.sh
