#!/bin/sh
# Set the workspace version (and the synced package.json) to $1, in place.
#
# Used by CI to stamp nightly versions onto build artifacts — the change is
# never committed. Every crate inherits `[workspace.package] version`, so
# editing the root Cargo.toml is enough for cargo, maturin, and napi.

set -eu

VERSION="${1:?usage: apply-version.sh <version>}"

REPO_ROOT=$(realpath "$(dirname "$0")/../..")
cd "$REPO_ROOT"

VERSION="$VERSION" python3 - <<'PY'
import json
import os
import pathlib
import re

version = os.environ["VERSION"]

# [workspace.package] version — the first (and only) bare `version =` line.
cargo = pathlib.Path("Cargo.toml")
text, n = re.subn(
    r'^version = "[^"]*"', f'version = "{version}"', cargo.read_text(), count=1, flags=re.M
)
assert n == 1, "workspace version line not found in Cargo.toml"

# Internal path deps carry `version = "..."` requirements (for crates.io).
# Cargo refuses to build when a path dep's actual version does not satisfy
# the requirement, so nightly stamps must rewrite these too. Strip any `+build`
# metadata: cargo ignores (and warns about) metadata in a version requirement,
# and a metadata-less requirement still matches the stamped package version.
dep_version = version.split("+", 1)[0]
text, n = re.subn(
    r'(= \{ path = "[^"]+", version = ")[^"]*(")', rf'\g<1>{dep_version}\g<2>', text, flags=re.M
)
assert n > 0, "no internal path-dep version requirements found in Cargo.toml"
cargo.write_text(text)

pkg = pathlib.Path("js-lunamodel/package.json")
data = json.loads(pkg.read_text())
data["version"] = version
pkg.write_text(json.dumps(data, indent=2) + "\n")

print(f"version set to {version}")
PY
