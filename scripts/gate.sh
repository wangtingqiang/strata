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
cargo test --workspace --all-features
note=$'note: trybuild copies [workspace.dependencies] into its temporary workspace;\n      "unused workspace dependency" warnings from it are expected.'
if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
    printf '\033[1;33m%s\033[0m\n' "$note"
else
    printf '%s\n' "$note"
fi

echo "==> doc (-D warnings)"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

echo "==> gate passed"