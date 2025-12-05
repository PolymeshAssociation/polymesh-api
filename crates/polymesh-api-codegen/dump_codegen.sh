#!/bin/sh
#

cargo run --example dump_codegen -- ws://127.0.0.1:9944 >./examples/test_api/polymesh.rs
