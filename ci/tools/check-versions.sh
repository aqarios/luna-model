#!/bin/sh
# Verify that every stated version matches the workspace version.
#
# The `[workspace.package] version` in the root Cargo.toml is the single
# source of truth. The only other places a version is *stated* are
# js-lunamodel/package.json and the rs-lunamodel SDK workspace (synced by
# set-release-version.sh) — everything else derives at build time:
#   - all crates inherit via `version.workspace = true`
#   - maturin reads the Python wheel version from the inherited pylm version
#     (py-lunamodel declares `dynamic = ["version"]`; uv.lock records none)
#
# Every tracked Cargo.lock (also the nested rs-lunamodel workspaces') records
# the workspace-versioned crates; it must record them at the workspace version
# (refreshed by set-release-version.sh).
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

SDK=$(sed -n 's/^version = "\([^"]*\)".*/\1/p' rs-lunamodel/Cargo.toml | head -n 1)
if [ "$SDK" != "$WORKSPACE" ]; then
  echo "❌ rs-lunamodel/Cargo.toml is $SDK, workspace is $WORKSPACE" >&2
  echo "   (run ci/tools/apply-version.sh to re-sync)" >&2
  status=1
fi

if [ "$NPM" != "$WORKSPACE" ]; then
  echo "❌ js-lunamodel/package.json is $NPM, workspace is $WORKSPACE" >&2
  echo "   (run ci/tools/apply-version.sh to re-sync)" >&2
  status=1
fi

# Every tracked Cargo.lock (the root one and the nested workspaces') must record
# the workspace-versioned crates at the workspace version.
if ! WORKSPACE="$WORKSPACE" python3 - <<'PY'
import os
import subprocess
import sys
import tomllib

workspace = os.environ["WORKSPACE"]
tracked = subprocess.run(
    ["git", "ls-files", "*Cargo.toml", "*Cargo.lock"], capture_output=True, text=True, check=True
).stdout.split()

inherit = set()
for path in (p for p in tracked if p.endswith("Cargo.toml")):
    with open(path, "rb") as f:
        package = tomllib.load(f).get("package", {})
    if package.get("version") == {"workspace": True}:
        inherit.add(package["name"])

ok = True
for path in (p for p in tracked if p.endswith("Cargo.lock")):
    with open(path, "rb") as f:
        packages = tomllib.load(f).get("package", [])
    for package in packages:
        if "source" not in package and package["name"] in inherit and package["version"] != workspace:
            print(f"❌ {path} records {package['name']} {package['version']}, workspace is {workspace}", file=sys.stderr)
            ok = False
if not ok:
    print("   (run `cargo update --workspace` in that lockfile's directory)", file=sys.stderr)
sys.exit(0 if ok else 1)
PY
then
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
