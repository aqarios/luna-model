#!/bin/sh
# Bump the workspace version and sync every derived version.
#
# The `[workspace.package] version` in the root Cargo.toml is the single
# source of truth:
#   - every crate (pylm, js-lunamodel, pyo3-lunamodel, crates/*) inherits it
#     via `version.workspace = true`, so the Python wheel version (read by
#     maturin from pylm) and the napi crate version can never drift
#   - js-lunamodel/package.json is synced to it here
#   - Cargo.lock is refreshed here (uv.lock records no project version —
#     py-lunamodel declares its version as dynamic)
#
# `ci/tools/check-versions.sh` enforces the invariant in CI.
#
# Inputs (environment):
#   RELEASE_TYPE  optional  major | minor | patch | release | rc | beta | alpha
#                           (passed to `cargo set-version --bump`; default patch)

set -eu

: "${RELEASE_TYPE:=patch}"

REPO_ROOT=$(realpath "$(dirname "$0")/../..")
cd "$REPO_ROOT"

cargo install cargo-edit --locked --version 0.13.10

# All members inherit the workspace version, so this edits exactly one line:
# the [workspace.package] version in the root Cargo.toml.
cargo set-version --workspace --bump "$RELEASE_TYPE"

VERSION=$(sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)
test -n "$VERSION"

# Sync the npm package manifest (its Cargo.toml already inherits the bump).
(cd js-lunamodel && bun pm pkg set "version=$VERSION")

# Refresh Cargo.lock so it records the new version.
cargo update --workspace

echo "Workspace version set to $VERSION"
