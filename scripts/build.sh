#!/usr/bin/env bash
# Builds the page: the wasm module and its glue go to web/pkg, and web/ is then the whole site, ready to be served from any static host.
#
#   scripts/build.sh            a release build
#   scripts/build.sh --dev      a debug build, which is quicker
#
# Needs the toolchain in rust-toolchain.toml (or any stable that is 1.85 or newer) with the wasm32-unknown-unknown target, and wasm-pack
# 0.13 or newer. `--locked` makes the build use Cargo.lock as it is, so that two builds of one commit give the same dependencies.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
profile=--release
[ "${1:-}" = "--dev" ] && profile=--dev
rm -rf web/pkg
wasm-pack build app --target web "$profile" --out-dir ../web/pkg --out-name dess_app -- --locked
rm -f web/pkg/.gitignore web/pkg/package.json web/pkg/README.md
echo "built web/: $(du -sh web/pkg | cut -f1) of wasm and glue"
