#!/usr/bin/env bash
# Cargo for the engine workspace: `check` (fmt, clippy, test), `fmt`, `wasm` (wasm-pack build
# into packages/engine-wasm/pkg), `reference` (the WASM exports run natively, for the parity
# test) and `licenses` (cargo deny, ADR 0003).
set -euo pipefail
# Homebrew's rustup is keg-only and cargo-deny installs to ~/.cargo/bin; neither is on a
# non-login PATH (the Stop hook, pnpm scripts).
PATH="$HOME/.cargo/bin:/opt/homebrew/opt/rustup/bin:$PATH"
cd "$(dirname "${BASH_SOURCE[0]}")/../engine"
case "${1:-}" in
  check)
    cargo fmt --all --check
    cargo clippy --all-targets --quiet -- -D warnings
    cargo test --quiet
    ;;
  fmt) cargo fmt --all ;;
  wasm)
    wasm-pack --quiet build crates/wasm --release --target web --no-pack \
      --out-dir ../../../packages/engine-wasm/pkg
    ;;
  reference) cargo run --quiet --release -p fieldtest-wasm --example reference ;;
  licenses) cargo deny --log-level error check licenses ;;
  *) echo "usage: engine.sh check|fmt|wasm|reference|licenses" >&2; exit 64 ;;
esac
