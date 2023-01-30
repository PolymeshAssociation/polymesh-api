# Updating Polymesh metadata

1. Download the newest raw metadata (Can use the `dev` chain metadata for all chains):
    * The example `download_metadata` from the `crates/polymesh-api-client/examples/` folder can be used to download the metadata.
		    - `cd crates/polymesh-api-client/`
				-	`cargo run --example download_metadata -- ws://localhost:9944/` # run against a local dev-chain node.
				-	That will download the latest runtime metadata from the local node as file `polymesh_dev_spec_5002000.meta`.
2. Add the metadata file as `specs/polymesh_<chain name>_spec_<spec version>.meta`
3. Update the metadata file path in `src/lib.rs`.
4. Bump version of `polymesh-api` in `Cargo.toml`.
5. Run `cargo build` to confirm that it compiles.  (This also updates `Cargo.lock`)
6. Commit changed/added files.
7. Publish: `cargo publish`
