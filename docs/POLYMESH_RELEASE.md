# Updating Polymesh metadata

1. Download the newest raw metadata (Can use the `dev` chain metadata for all chains):
	-	`./download_metadata.sh ws://localhost:9944/` # run against a local dev-chain node.
	-	That will download the latest runtime metadata from the local node as file `polymesh_dev_spec_5002000.meta`.
2. Add the metadata file as `specs/polymesh_<chain name>_spec_<spec version>.meta`
3. Update the metadata file path in `src/lib.rs`.
4. Bump version of `polymesh-api` in `Cargo.toml`.
5. Run `cargo build` to confirm that it compiles.  (This also updates `Cargo.lock`)
6. Commit changed/added files.
7. Publish: `cargo publish`

# Publishing order for all crates

1. `crates/polymesh-api-client`
2. `crates/polymesh-api-ink`
3. `crates/polymesh-api-codegen`
4. `crates/polymesh-api-codegen-macro`
5. `polymesh-api`
6. `crates/polymesh-api-client-extras`
7. `crates/polymesh-api-tester`
8. `crates/polymesh-offline-signer`
