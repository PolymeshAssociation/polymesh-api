# polymesh-tools

Helper CLI utilities for Polymesh chain operations.

## Commands

- `upgrade-chain <WASM_FILE>`
- `committee-upgrade <WASM_FILE> <VERSION> <COMMITTEE_KEY>...`

## Prerequisites

- A running Polymesh node (default URL: `ws://localhost:9944`).
- A Rust toolchain compatible with this workspace.
- Access to signing keys linked to on-chain identities when using committee flows.

Optional environment variables:

- `POLYMESH_URL` (default: `ws://localhost:9944`)
- `DATABASE_URL` (default: `accounts.db`)

## Build

From the repository root:

```bash
cargo build -p polymesh-tools
```

## Reset Local Signer DB

When a local dev-chain is recreated, reset the local signer database before running committee commands.

From the `crates/polymesh-tools` directory:

```bash
./reset_db.sh
```

What this does:

- Removes existing `accounts.db*` files.
- Recreates `accounts.db` from `empty-accounts.db`.

This is important when chain state is reset, so cached signer/nonces in the local DB do not conflict.

## Usage

From the repository root:

```bash
cargo run -p polymesh-tools -- upgrade-chain /path/to/runtime.wasm
```

Committee-based upgrade:

```bash
cargo run -p polymesh-tools -- committee-upgrade /path/to/runtime.wasm 7.3.0 "//Alice"
```

You can also pass multiple committee keys:

```bash
cargo run -p polymesh-tools -- committee-upgrade /path/to/runtime.wasm 7.3.0 "//Alice" "//Bob" "<seed phrase or private key>"
```

## Local Dev-Chain Note

For a local development chain, one committee key is typically enough:

- `//Alice`

Example:

```bash
cd crates/polymesh-tools && ./reset_db.sh
cargo run -- committee-upgrade /tmp/polymesh_runtime.compact.compressed.wasm 7.3.0 "//Alice"
```

## Committee Upgrade Flow

The `committee-upgrade` command performs these steps:

1. Resolves each provided key to its on-chain DID.
2. Checks membership and vote thresholds for `UpgradeCommittee` and `PolymeshCommittee`.
3. Uses `UpgradeCommittee.vote_or_propose(Yes, Pips.propose(System.set_code(...), ...))`.
4. Extracts the created `PipId` from chain events.
5. Uses `PolymeshCommittee.vote_or_propose(Yes, Pips.approve_committee_proposal(pip_id))`.
6. Uses the `PolymeshCommittee.release_coordinator` DID key to call `Pips.reschedule_execution(pip_id, None)`.

## Notes

- On non-dev networks, you may need multiple committee keys to satisfy vote thresholds.
- Each provided key must map to a unique DID.
- If a key is not linked to an identity, the command exits with an error.
