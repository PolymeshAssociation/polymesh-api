#!/bin/sh
NODE_URL="${1:-ws://localhost:9944/}"

cargo run -p polymesh-api-client --example download_metadata -- $NODE_URL
