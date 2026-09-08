#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cargo test --workspace --all-features --doc
cargo test --workspace --no-default-features --doc
