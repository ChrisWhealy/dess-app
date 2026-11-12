#!/usr/bin/env bash
# Moves the application to a newly published svg-dom-graph, and runs everything that CI runs, against the registry's copy and with no local patch.
#
#   scripts/adopt.sh 0.2.51
#
# This is the last step of a release of the library: the published crate is the one that was qualified when its package built this application,
# and this shows that the copy that crates.io serves is the same thing. Commit the changes that it makes, and CI then has to pass as well.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
version="${1:?usage: scripts/adopt.sh <svg-dom-graph version>}"

# No local patch may be in force, since the point is the registry's copy.
! grep -rn 'patch.crates-io' Cargo.toml .cargo 2>/dev/null || { echo "a [patch.crates-io] is in force: remove it first" >&2; exit 1; }

for manifest in app/Cargo.toml crates/service-sim/Cargo.toml; do
  sed -i.bak -E "s/^(svg-dom-graph = )\"[^\"]*\"/\1\"$version\"/" "$manifest" && rm -f "$manifest.bak"
done
cargo update -p svg-dom-graph --precise "$version"
grep -A3 '^name = "svg-dom-graph"' Cargo.lock | grep -q 'source = "registry+https://github.com/rust-lang/crates.io-index"' \
  || { echo "svg-dom-graph is not from the registry in Cargo.lock" >&2; exit 1; }

cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --exclude dess-app --locked
scripts/build.sh
wasm-pack test --headless --chrome app
wasm-pack test --headless --chrome crates/service-sim
cargo deny check
echo "dess-app passes against svg-dom-graph $version from the registry"
