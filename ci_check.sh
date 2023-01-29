#!/bin/sh

set -e

export RUSTFLAGS='-D warnings'

cargo fmt --check
cargo check
cargo clippy
cargo test

