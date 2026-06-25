#!/bin/sh
# Generate THIRD_PARTY_LICENSES.txt for every distributed artifact.
#
# One notice per shipped artifact, each built from *that* artifact's resolved
# dependency tree so the attribution is neither short (missing a bundled crate)
# nor padded (listing crates it does not ship):
#   - py-lunamodel/THIRD_PARTY_LICENSES.txt  <- pylm wheel
#       (maturin builds pylm with --no-default-features --features extension-module)
#   - js-lunamodel/THIRD_PARTY_LICENSES.txt  <- @aqarios/luna-model npm package
#
# cargo-about reproduces each dependency's LICENSE file verbatim (preserving its
# copyright notice), enforces the SPDX allow-list in ci/about/about.toml —
# failing the build on an unvetted or copyleft license — and, with network
# access, backfills license texts from clearlydefined.io for crates that ship
# none. No target filter is applied, so the union of every shipped platform
# (Linux/macOS/Windows, gnu/musl, x86_64/aarch64, plus wasm for js) is covered;
# only dev-dependencies are excluded, since they are not distributed.
#
# Requires network access. The cargo-about version is pinned for reproducible
# output; an already-installed matching binary is reused.

set -eu

ABOUT_VERSION=0.9.0

REPO_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$REPO_ROOT"

CARGO_HOME=${CARGO_HOME:-"$HOME/.cargo"}
ABOUT_BIN="$CARGO_HOME/bin/cargo-about"

installed=$("$ABOUT_BIN" --version 2>/dev/null | awk '{print $2}' || true)
if [ "$installed" != "$ABOUT_VERSION" ]; then
  cargo install cargo-about --locked --version "$ABOUT_VERSION" --root "$CARGO_HOME"
fi

if [ ! -x "$ABOUT_BIN" ]; then
  echo "cargo-about was installed, but $ABOUT_BIN is not executable" >&2
  exit 1
fi

gen() {
  manifest=$1
  output=$2
  shift 2
  echo "Generating $output from $manifest"
  "$ABOUT_BIN" generate \
    --config ci/about/about.toml \
    ci/about/about.hbs \
    --manifest-path "$manifest" \
    --output-file "$output" \
    "$@"
}

# Feature flags must mirror how each artifact is actually built (see the maturin
# config in py-lunamodel/pyproject.toml and the napi build for js-lunamodel), so
# the resolved dependency tree matches what ships.
gen py-lunamodel/pylm/Cargo.toml py-lunamodel/THIRD_PARTY_LICENSES.txt \
  --no-default-features --features extension-module

gen js-lunamodel/Cargo.toml js-lunamodel/THIRD_PARTY_LICENSES.txt

echo "THIRD_PARTY_LICENSES.txt regenerated for all artifacts."
