#!/bin/sh
# Bump the workspace version and sync every derived version.
#
# The `[workspace.package] version` in the root Cargo.toml is the single
# source of truth:
#   - every crate (pylm, js-lunamodel, rs-lunamodel, crates/*) inherits it
#     via `version.workspace = true`, so the Python wheel version (read by
#     maturin from pylm) and the napi crate version can never drift
#   - js-lunamodel/package.json and the C headers (version.h) are synced to it here
#   - Cargo.lock is refreshed here (uv.lock records no project version —
#     py-lunamodel declares its version as dynamic)
#
# `ci/tools/check-versions.sh` enforces the invariant in CI.
#
# Inputs (environment):
#   RELEASE_TYPE  optional  major | minor | patch | release | rc | beta | alpha
#                           (passed to `cargo set-version --bump`; default patch),
#                           or <major|minor|patch>-<alpha|beta|rc> for the first
#                           pre-release of the next version, e.g. minor-alpha:
#                           0.7.4 -> 0.8.0-alpha.1

set -eu

: "${RELEASE_TYPE:=patch}"

REPO_ROOT=$(realpath "$(dirname "$0")/../..")
cd "$REPO_ROOT"

cargo install cargo-edit --locked --version 0.13.10

# Strip any `+<sha>` build metadata left by a nightly stamp (apply-version.sh):
sed -i.bak -E 's/^(version = "[^"+]+)\+[^"]+"/\1"/' Cargo.toml && rm -f Cargo.toml.bak

# All members inherit the workspace version, so this edits exactly one line:
# the [workspace.package] version in the root Cargo.toml.
read_version() { sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -n 1; }

case "$RELEASE_TYPE" in
  major-alpha | major-beta | major-rc | minor-alpha | minor-beta | minor-rc | patch-alpha | patch-beta | patch-rc)
    # A plain `--bump alpha` on a release version bumps the patch, so compute the
    # next major/minor/patch here and start its pre-release series at .1. Setting
    # it in one step keeps it an upgrade (a pre-release sorts below its release).
    base=$(read_version | sed 's/[-+].*//')
    major=${base%%.*}
    rest=${base#*.}
    minor=${rest%%.*}
    patch=${rest#*.}
    case "${RELEASE_TYPE%-*}" in
      major) next="$((major + 1)).0.0" ;;
      minor) next="$major.$((minor + 1)).0" ;;
      patch) next="$major.$minor.$((patch + 1))" ;;
    esac
    cargo set-version --workspace "$next-${RELEASE_TYPE#*-}.1"
    ;;
  *)
    cargo set-version --workspace --bump "$RELEASE_TYPE"
    ;;
esac

VERSION=$(read_version)
test -n "$VERSION"

if git ls-remote --exit-code --tags origin "refs/tags/v$VERSION" >/dev/null 2>&1; then
  echo "❌ v$VERSION is already released" >&2
  exit 1
fi

# Sync the npm package manifest (its Cargo.toml already inherits the bump).
(cd js-lunamodel && bun pm pkg set "version=$VERSION")

# Refresh Cargo.lock so it records the new version.
cargo update --workspace

# The committed C headers embed the version (luna_model/version.h): regenerate
# them and the copy bundled in the wheel, as `just cheader wheelheaders` does,
# so the drift checks pass on the release PR.
cargo build -p lunamodel-capi-host
rm -rf py-lunamodel/src/luna_model/include
mkdir -p py-lunamodel/src/luna_model/include
cp -R crates/capi-host/include/. py-lunamodel/src/luna_model/include/
cp -R cxx-lunamodel/src/. py-lunamodel/src/luna_model/include/

echo "Workspace version set to $VERSION"
