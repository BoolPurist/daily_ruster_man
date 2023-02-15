#!/bin/sh

set -e

export CI=true
export RUSTFLAGS='-D warnings'

cargo fmt --check
cargo clippy
cargo test

