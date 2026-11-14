#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> fmt"
cargo fmt --all --check

echo "==> clippy (-D warnings, all features)"
cargo clippy --workspace --all-features --all-targets -- -D warnings

echo "==> check (all features)"
cargo check --workspace --all-features

echo "==> check (default features)"
cargo check --workspace

echo "==> test (all features)"
echo "    note: trybuild copies [workspace.dependencies] into its temporary workspace;"
echo "          unused_workspace_dependencies warnings from it are expected."
cargo test --workspace --all-features

echo "==> doc (-D warnings)"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

echo "==> gate passed"