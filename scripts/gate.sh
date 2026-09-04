#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> fmt"
cargo fmt --all --check

echo "==> clippy (-D warnings, all features)"
cargo clippy --workspace --all-features -- -D warnings

echo "==> check (all features)"
cargo check --workspace --all-features

echo "==> check (default features)"
cargo check --workspace

echo "==> test (all features)"
cargo test --workspace --all-features

echo "==> doc (-D warnings)"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

echo "==> gate passed"