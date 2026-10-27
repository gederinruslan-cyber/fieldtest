#!/usr/bin/env bash
# Cargo for the engine workspace: `check` (fmt, clippy, test), `fmt`, `wasm` (wasm32 build
# until F0-03 adds wasm-pack) and `licenses` (cargo deny, ADR 0003).
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
  wasm) cargo build --quiet --release --target wasm32-unknown-unknown ;;
  licenses) cargo deny --log-level error check licenses ;;
  *) echo "usage: engine.sh check|fmt|wasm|licenses" >&2; exit 64 ;;
esac
