#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  installer="$(mktemp)"
  curl -fsSLo "$installer" https://sh.rustup.rs
  sh "$installer" -y --profile minimal --default-toolchain stable --no-modify-path
  . "$HOME/.cargo/env"
  rm "$installer"
fi

cargo run --release --locked --manifest-path shredder/Cargo.toml --
