#!/bin/sh
# Build a py-lunamodel wheel for a single target/Python combination.
#
# Every wheel is cross-compiled from a Linux host:
#   - *-linux-gnu / *-linux-musl / *-apple-darwin via `maturin --zig`
#     (zig ships its own glibc/musl/macOS SDK shims; the `build` dependency
#     group provides zig through `maturin[zig]`)
#   - *-pc-windows-msvc via maturin's built-in xwin support (the MSVC SDK is
#     cached by CI under ~/.cache/cargo-xwin)
#
# Inputs (environment):
#   TARGET  required  Rust target triple to build for
#   PY      optional  Python interpreter version (default 3.13)
#
# Output: wheel in <repo-root>/build/

set -eux

: "${TARGET:?TARGET (rust target triple) must be set}"
: "${PY:=3.13}"

PROJECT_DIR=$(dirname "$(realpath "$(dirname "$0")")")

EXTRA_ARGS=""
case "$TARGET" in
  # zig links glibc 2.17; tag the wheel with the matching manylinux level.
  # musl targets are auto-tagged musllinux by maturin; macOS/Windows need no tag.
  *-unknown-linux-gnu) EXTRA_ARGS="--compatibility manylinux2014" ;;
esac

mkdir -p "$PROJECT_DIR/build"
cd "$PROJECT_DIR/py-lunamodel"
# shellcheck disable=SC2086 # EXTRA_ARGS is intentionally word-split
uv run --no-sync \
  --group=build \
  --no-group=dev \
  maturin build \
  --release \
  --zig \
  --strip \
  -i "${PY}" \
  --out "$PROJECT_DIR/build" \
  --target "$TARGET" \
  $EXTRA_ARGS
