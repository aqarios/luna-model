#!/bin/sh
# Compute (and print) the nightly version derived from the workspace version.
#
#   stable      X.Y.Z        ->  X.Y.(Z+1)-dev.<commit-count>+<short-sha>
#   prerelease  X.Y.Z-pre.N  ->  X.Y.Z-pre.<commit-count>+<short-sha>
#
# Cargo treats this as a semver prerelease; maturin normalizes it to a PEP 440
# dev/local version for the wheel (e.g. 0.6.4.dev1875+abc1234).
#
# Requires full git history (checkout with fetch-depth: 0) for a meaningful
# commit count. Prints the version to stdout; does not modify anything —
# pair with ci/tools/apply-version.sh.

set -eu

REPO_ROOT=$(realpath "$(dirname "$0")/../..")
cd "$REPO_ROOT"

CURRENT=$(sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)
test -n "$CURRENT"

COMMIT_COUNT=$(git rev-list --count HEAD)
COMMIT_HASH=$(git rev-parse --short HEAD)

CURRENT="$CURRENT" COMMIT_COUNT="$COMMIT_COUNT" COMMIT_HASH="$COMMIT_HASH" \
python3 - <<'PY'
import os
import re
import sys

version = os.environ["CURRENT"]
count = os.environ["COMMIT_COUNT"]
sha = os.environ["COMMIT_HASH"]

match = re.match(r"^(\d+\.\d+\.\d+)(?:-([a-zA-Z]+)[.]?\d*)?(?:\+.+)?$", version)
if not match:
    sys.exit(f"unable to parse workspace version: {version}")

base, prerelease = match.groups()
if prerelease:
    print(f"{base}-{prerelease}.{count}+{sha}")
else:
    major, minor, patch = base.split(".")
    print(f"{major}.{minor}.{int(patch) + 1}-dev.{count}+{sha}")
PY
