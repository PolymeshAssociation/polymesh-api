# polymesh-tools

Helper CLI utilities for Polymesh chain operations.

## Commands

- `upgrade-chain <WASM_FILE> [--yes]`
- `committee-upgrade <WASM_FILE> <COMMITTEE_KEY>... [--yes]`
- `committee-proposal <PROPOSAL> <COMMITTEE_KEY>... [--url <URL>] [--description <DESCRIPTION>]`

The runtime version is read from the WASM file itself (no version argument needed). Both
upgrade commands display the runtime details (spec name, spec version, `setCode` hash) and
ask for confirmation before submitting; pass `--yes` / `-y` to skip the prompt.

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
cargo run -p polymesh-tools -- committee-upgrade /path/to/runtime.wasm "//Alice"
```

You can also pass multiple committee keys:

```bash
cargo run -p polymesh-tools -- committee-upgrade /path/to/runtime.wasm "//Alice" "//Bob" "<seed phrase or private key>"
```

The runtime spec version is extracted from the WASM and used to build the proposal URL and
description. Before anything is submitted you will be shown the runtime details and asked to
confirm:

```text
Runtime details:
  WASM file:    /path/to/runtime.wasm
  Spec name:    polymesh_testnet
  Spec version: 8002000 (v8.2.0)
  Code size:    2444409 bytes (compressed)
  setCode hash: 0xabf89e6167144fc2e6073c7b1f295fa422ea69da320e678f49db5db3df78c82d
Proceed with the upgrade? [y/N]:
```

Use `--yes` to skip the prompt (e.g. in scripts).

Committee-based proposal for an arbitrary runtime call:

```bash
cargo run -p polymesh-tools -- committee-proposal 0x<hex-encoded-call> "//Alice"
```

The `<PROPOSAL>` argument is the hex encoded SCALE call data of the runtime call to propose
(with or without a `0x` prefix). Optional `--url` and `--description` flags attach discussion
metadata to the PIP:

```bash
cargo run -p polymesh-tools -- committee-proposal 0x<hex-encoded-call> "//Alice" "//Bob" \
  --url "https://forum.polymesh.network/t/proposal-discussion" \
  --description "Increase active PIP limit"
```

## Local Dev-Chain Note

For a local development chain, one committee key is typically enough:

- `//Alice`

Example:

```bash
cd crates/polymesh-tools && ./reset_db.sh
cargo run -- committee-upgrade /tmp/polymesh_runtime.compact.compressed.wasm "//Alice"
```

## Committee Upgrade Flow

The `committee-upgrade` command performs these steps:

1. Loads the WASM, extracts the runtime spec version and `setCode` hash, and asks for confirmation.
2. Resolves each provided key to its on-chain DID.
3. Checks membership and vote thresholds for `UpgradeCommittee` and `PolymeshCommittee`.
4. Uses `UpgradeCommittee.vote_or_propose(Yes, Pips.propose(System.set_code(...), ...))`.
5. Extracts the created `PipId` from chain events.
6. Uses `PolymeshCommittee.vote_or_propose(Yes, Pips.approve_committee_proposal(pip_id))`.
7. Uses the `PolymeshCommittee.release_coordinator` DID key to call `Pips.reschedule_execution(pip_id, None)`.

The `committee-proposal` command runs the same flow, but instead of building
`System.set_code` from a WASM file it decodes the hex encoded runtime call passed on the
command line and proposes it directly via `Pips.propose(<call>, ...)`.

## Runtime Version Detection

The runtime version is read directly from the WASM file, so it no longer has to be passed as a
CLI argument. The WASM is loaded with the Polymesh fork of [`subwasm`]'s `wasm-loader` (which
also transparently decompresses compact/compressed runtimes). The `spec_version` is decoded from
the `runtime_version` custom section embedded in the runtime, and converted to a release version
(`8_002_000` -> `8.2.0`). The `setCode` hash is the blake2_256 hash of the `System.set_code` call
built from the (possibly compressed) code blob, i.e. the same hash shown on-chain for the proposal.

[`subwasm`]: https://github.com/PolymeshAssociation/subwasm

## Notes

- On non-dev networks, you may need multiple committee keys to satisfy vote thresholds.
- Each provided key must map to a unique DID.
- If a key is not linked to an identity, the command exits with an error.
