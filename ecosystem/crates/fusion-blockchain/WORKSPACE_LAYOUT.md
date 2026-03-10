# Fusion Tendermint Workspace Layout

## Target
Tendermint-style consensus first, with smart contracts included in MVP.

## Crate Tree

- fusion-blockchain (facade)
- fusion-chain-primitives
- fusion-chain-codec
- fusion-chain-crypto
- fusion-chain-ledger
- fusion-chain-state
- fusion-chain-tx
- fusion-chain-mempool
- fusion-chain-p2p
- fusion-chain-consensus-tendermint
- fusion-chain-finality
- fusion-chain-vm
- fusion-chain-contract-abi
- fusion-chain-contract-runtime
- fusion-chain-contract-sdk
- fusion-chain-rpc
- fusion-chain-node
- fusion-chain-node-local
- fusion-chain-audit
- fusion-chain-observability
- fusion-chain-testkit

## Dependency Layers

1. primitives -> codec -> crypto
2. state, tx, ledger
3. mempool, p2p, consensus-tendermint, finality
4. vm -> contract-abi -> contract-runtime -> contract-sdk
5. rpc + node
6. audit + observability
7. testkit

## Execution Order

1. Finalise primitive wire formats and chain IDs.
2. Implement validator key management and vote verification.
3. Wire mempool admission to account nonce and fee checks.
4. Enable block proposal, vote aggregation, and commit flow.
5. Deploy contract runtime with deterministic gas accounting.
6. Expose RPC and harden audit and telemetry gates.
7. Expand testkit to adversarial and Byzantine scenarios.
