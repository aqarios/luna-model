#!/bin/bash
# Check (default) or apply clang-format to hand-written C/C++ sources.
# Generated capi-host headers are excluded so formatting never diverges from the generator.
set -e

if [ -z "$1" ]; then
    args="--dry-run --Werror"          # check only
elif [ "$1" = "apply" ]; then
    args="-i"                          # rewrite in place
else
    echo "Invalid argument; use no arguments (check) or 'apply'." >&2
    exit 1
fi

our_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$our_dir/.." && pwd)"
style="${repo_root}/.clang-format"

files=$(git -C "$repo_root" ls-files \
    | grep -E '\.(c|h|cpp|hpp)$' \
    | grep -vE '^(crates/capi-host/include|py-lunamodel/src/luna_model/include)/luna_model/(types|funcs|funcs_py|version)\.h$')

status=0
for file in $files; do
    full="$repo_root/$file"
    [ -f "$full" ] || continue        # skip tracked-but-deleted files (mid-refactor)
    if ! clang-format --style="file:$style" $args "$full"; then
        status=1
    fi
done
exit $status
