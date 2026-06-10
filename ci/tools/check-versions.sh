#!/bin/sh
# Verify that every stated version matches the workspace version.
#
# The `[workspace.package] version` in the root Cargo.toml is the single
# source of truth. The only other place a version is *stated* is
# js-lunamodel/package.json (synced by set-release-version.sh) — everything
# else derives at build time:
#   - all crates inherit via `version.workspace = true`
#   - maturin reads the Python wheel version from the inherited pylm version
#     (py-lunamodel declares `dynamic = ["version"]`; uv.lock records none)
#
# Inputs (environment):
#   EXPECTED_VERSION  optional  additionally assert the workspace version
#                               equals this value (used on release tags)

set -eu

REPO_ROOT=$(realpath "$(dirname "$0")/../..")
cd "$REPO_ROOT"

WORKSPACE=$(sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)
test -n "$WORKSPACE"

NPM=$(python3 -c 'import json; print(json.load(open("js-lunamodel/package.json"))["version"])')

status=0

if [ "$NPM" != "$WORKSPACE" ]; then
  echo "❌ js-lunamodel/package.json is $NPM, workspace is $WORKSPACE" >&2
  echo "   (run ci/tools/apply-version.sh to re-sync)" >&2
  status=1
fi

if [ -n "${EXPECTED_VERSION:-}" ] && [ "$WORKSPACE" != "$EXPECTED_VERSION" ]; then
  echo "❌ workspace version is $WORKSPACE, expected $EXPECTED_VERSION" >&2
  status=1
fi

if [ "$status" -eq 0 ]; then
  echo "✅ all versions in sync: $WORKSPACE"
fi
exit "$status"
