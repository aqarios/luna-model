#!/usr/bin/env python3
"""Check that the C ABI only changed compatibly since the last release.

Compares the committed C headers (crates/capi-host/include, generated from the Rust
definitions and kept current by `just cdrift`) with the same headers at the last
release tag. Alpha pre-releases give no ABI guarantee, so they are never compared
against; betas, release candidates, and releases are. Offsets, sizes, and enum values come from compiling the headers with the
system C compiler, not from parsing, so they are exactly what consumers see.

While `LUNA_MODEL_CAPI_ABI_VERSION` is unchanged, only these changes are allowed:
  - `LunaModelCApi` (the dispatch table) only grows: every released field keeps its
    name, offset, size, and declaration; new fields only use bytes no released field
    used (the end of the table, or padding);
  - new structs, enum constants, typedefs, and `lm_*` functions;
every released struct other than the table, enum constant value, enum size, typedef,
and `lm_*` prototype stays exactly as released. Anything else needs an ABI version
bump (a higher version allows every change; a lower one is always an error).

Usage:
  check-abi.py [--base-ref REF]   compare the working tree with REF (default: the
                                  newest non-alpha release tag `v*` merged into
                                  HEAD, other than one on HEAD itself)
  check-abi.py --self-test        verify the rules against mutated copies of the
                                  current headers
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
INCLUDE = "crates/capi-host/include"
TABLE = "LunaModelCApi"
ABI_DEFINE = "LUNA_MODEL_CAPI_ABI_VERSION"


@dataclass
class Field:
    name: str
    decl: str
    offset: int = 0
    size: int = 0


@dataclass
class Abi:
    version: int
    structs: dict[str, list[Field]] = field(default_factory=dict)
    struct_sizes: dict[str, tuple[int, int]] = field(default_factory=dict)  # size, align
    enums: dict[str, list[str]] = field(default_factory=dict)
    enum_sizes: dict[str, int] = field(default_factory=dict)
    constants: dict[str, int] = field(default_factory=dict)
    typedefs: dict[str, str] = field(default_factory=dict)
    functions: dict[str, str] = field(default_factory=dict)


def strip_comments(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    return re.sub(r"//[^\n]*", " ", text)


def normalize(decl: str) -> str:
    decl = " ".join(decl.split())
    return re.sub(r"\s*([(),*\[\]])\s*", r"\1", decl)


def declared_name(decl: str) -> str:
    if m := re.search(r"\(\s*\*\s*(\w+)\s*\)", decl):  # function pointer
        return m.group(1)
    if m := re.search(r"(\w+)\s*\[[^\]]*\]\s*$", decl):  # array
        return m.group(1)
    if m := re.search(r"(\w+)\s*$", decl):
        return m.group(1)
    raise ValueError(f"cannot find the declared name in {decl!r}")


def parse(include: Path) -> Abi:
    types = strip_comments((include / "luna_model/types.h").read_text())
    funcs = strip_comments((include / "luna_model/funcs.h").read_text())

    version = re.search(rf"#define\s+{ABI_DEFINE}\s+(\d+)", types)
    if version is None:
        raise ValueError(f"{ABI_DEFINE} not found")
    abi = Abi(int(version.group(1)))

    body = re.sub(r"^\s*#[^\n]*", " ", types, flags=re.M)
    for m in re.finditer(r"typedef\s+enum\s*\w*\s*\{(.*?)\}\s*(\w+)\s*;", body, flags=re.S):
        names = [c.split("=")[0].strip() for c in m.group(1).split(",") if c.strip()]
        abi.enums[m.group(2)] = names
    for m in re.finditer(r"typedef\s+struct\s*\w*\s*\{(.*?)\}\s*(\w+)\s*;", body, flags=re.S):
        if "{" in m.group(1):
            raise ValueError(f"nested definition in struct {m.group(2)}")
        decls = [d for d in (" ".join(d.split()) for d in m.group(1).split(";")) if d]
        abi.structs[m.group(2)] = [Field(declared_name(d), normalize(d)) for d in decls]
    rest = re.sub(r"typedef\s+(enum|struct)\s*\w*\s*\{.*?\}\s*\w+\s*;", " ", body, flags=re.S)
    for m in re.finditer(r"typedef\b[^;]*;", rest):
        abi.typedefs[declared_name(m.group(0)[:-1])] = normalize(m.group(0))
    if leftover := re.sub(r"typedef\b[^;]*;", " ", rest).strip():
        raise ValueError(f"unrecognized declarations in types.h: {leftover[:200]!r}")

    funcs = re.sub(r"^\s*#[^\n]*", " ", funcs, flags=re.M)
    funcs = re.sub(r'extern\s+"C"\s*\{|\}', " ", funcs)
    for decl in (d.strip() for d in funcs.split(";")):
        if not decl:
            continue
        name = re.search(r"(\w+)\s*\(", decl)
        if name is None:
            raise ValueError(f"unrecognized declaration in funcs.h: {decl[:200]!r}")
        abi.functions[name.group(1)] = normalize(decl)

    measure(include, abi)
    return abi


def measure(include: Path, abi: Abi) -> None:
    """Fills offsets, sizes, and enum values by compiling the headers."""
    lines = ["#include <stddef.h>", "#include <stdio.h>", '#include "luna_model/types.h"']
    lines.append("int main(void) {")
    for name, fields in abi.structs.items():
        lines.append(f'printf("S {name} %zu %zu\\n", sizeof({name}), _Alignof({name}));')
        for f in fields:
            lines.append(
                f'printf("F {name} {f.name} %zu %zu\\n", offsetof({name}, {f.name}), '
                f"sizeof((({name} *)0)->{f.name}));"
            )
    for name, constants in abi.enums.items():
        lines.append(f'printf("Z {name} %zu\\n", sizeof({name}));')
        for c in constants:
            lines.append(f'printf("E {c} %lld\\n", (long long){c});')
    lines.append("return 0;\n}")

    with tempfile.TemporaryDirectory() as tmp:
        source, binary = Path(tmp, "abi.c"), Path(tmp, "abi")
        source.write_text("\n".join(lines))
        cc = shutil.which("cc") or shutil.which("gcc") or shutil.which("clang")
        if cc is None:
            raise RuntimeError("no C compiler (cc/gcc/clang) found")
        subprocess.run(
            [cc, "-std=gnu11", f"-I{include}", "-o", str(binary), str(source)], check=True
        )
        output = subprocess.run([str(binary)], check=True, capture_output=True, text=True)

    fields = {(s, f.name): f for s, fs in abi.structs.items() for f in fs}
    for line in output.stdout.splitlines():
        kind, *values = line.split()
        if kind == "S":
            abi.struct_sizes[values[0]] = (int(values[1]), int(values[2]))
        elif kind == "F":
            f = fields[(values[0], values[1])]
            f.offset, f.size = int(values[2]), int(values[3])
        elif kind == "Z":
            abi.enum_sizes[values[0]] = int(values[1])
        elif kind == "E":
            abi.constants[values[0]] = int(values[1])


def compare(base: Abi, current: Abi) -> list[str]:
    """Returns the incompatible changes from `base` to `current`."""
    if current.version < base.version:
        return [f"ABI version decreased from {base.version} to {current.version}"]
    if current.version > base.version:
        return []

    errors = []
    for name, base_fields in base.structs.items():
        if name not in current.structs:
            errors.append(f"struct {name} was removed")
            continue
        fields = current.structs[name]
        if name == TABLE:
            errors += compare_table(base_fields, fields, base.struct_sizes[name], current.struct_sizes[name])
        elif (
            [(f.name, f.decl, f.offset, f.size) for f in base_fields]
            != [(f.name, f.decl, f.offset, f.size) for f in fields]
            or base.struct_sizes[name] != current.struct_sizes[name]
        ):
            errors.append(f"struct {name} changed (only {TABLE} may grow)")

    for name, constants in base.enums.items():
        if name not in current.enums:
            errors.append(f"enum {name} was removed")
        elif base.enum_sizes[name] != current.enum_sizes[name]:
            errors.append(f"enum {name} changed size")
        for c in constants:
            if c not in current.constants:
                errors.append(f"enum constant {c} was removed")
            elif current.constants[c] != base.constants[c]:
                errors.append(f"enum constant {c} changed from {base.constants[c]} to {current.constants[c]}")

    for kind, base_decls, decls in (
        ("typedef", base.typedefs, current.typedefs),
        ("function", base.functions, current.functions),
    ):
        for name, decl in base_decls.items():
            if name not in decls:
                errors.append(f"{kind} {name} was removed")
            elif decls[name] != decl:
                errors.append(f"{kind} {name} changed: {decl} -> {decls[name]}")
    return errors


def compare_table(
    base: list[Field], current: list[Field], base_size: tuple[int, int], size: tuple[int, int]
) -> list[str]:
    errors = []
    by_name = {f.name: f for f in current}
    for f in base:
        now = by_name.get(f.name)
        if now is None:
            errors.append(f"{TABLE}.{f.name} was removed or renamed")
        elif (now.offset, now.size, now.decl) != (f.offset, f.size, f.decl):
            errors.append(
                f"{TABLE}.{f.name} changed: offset {f.offset}, {f.size} bytes, `{f.decl}` -> "
                f"offset {now.offset}, {now.size} bytes, `{now.decl}` "
                "(insert new entry points only at the end)"
            )
    released = {f.name for f in base}
    for f in current:
        if f.name in released:
            continue
        overlaps = [b.name for b in base if f.offset < b.offset + b.size and b.offset < f.offset + f.size]
        if overlaps:
            errors.append(f"new field {TABLE}.{f.name} overlaps released field(s) {', '.join(overlaps)}")
    if size[0] < base_size[0]:
        errors.append(f"{TABLE} shrank from {base_size[0]} to {size[0]} bytes")
    if size[1] != base_size[1]:
        errors.append(f"{TABLE} alignment changed from {base_size[1]} to {size[1]}")
    return errors


def base_ref() -> str | None:
    """The newest non-alpha release tag merged into HEAD, other than one on HEAD itself."""

    def git(*args: str) -> str:
        return subprocess.run(
            ["git", *args], cwd=REPO_ROOT, check=True, capture_output=True, text=True
        ).stdout

    head = git("rev-parse", "HEAD").strip()
    # versionsort.suffix=-: a pre-release (v1.0.0-rc.1) sorts before its release (v1.0.0).
    tags = git(
        "-c", "versionsort.suffix=-", "tag", "--merged", "HEAD", "--list", "v[0-9]*",
        "--sort=-v:refname",
    ).split()
    for tag in tags:
        if "-alpha" in tag:
            continue
        if git("rev-list", "-n", "1", tag).strip() != head:
            return tag
    return None


def headers_at(ref: str, dest: Path) -> bool:
    """Extracts the headers at `ref` into `dest`; False if `ref` has none."""
    listed = subprocess.run(
        ["git", "ls-tree", "-r", "--name-only", ref, f"{INCLUDE}/luna_model/types.h"],
        cwd=REPO_ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()
    if not listed:
        return False
    archive = subprocess.run(
        ["git", "archive", ref, INCLUDE], cwd=REPO_ROOT, check=True, capture_output=True
    ).stdout
    subprocess.run(["tar", "-x", "-C", str(dest)], input=archive, check=True)
    return True


def check(ref: str | None) -> int:
    ref = ref or base_ref()
    if ref is None:
        print("✅ no earlier release to compare the C ABI with")
        return 0
    with tempfile.TemporaryDirectory() as tmp:
        if not headers_at(ref, Path(tmp)):
            print(f"✅ {ref} has no C API headers; nothing to compare")
            return 0
        base = parse(Path(tmp, INCLUDE))
    current = parse(REPO_ROOT / INCLUDE)
    errors = compare(base, current)
    if errors:
        print(f"❌ incompatible C ABI changes since {ref} (ABI version {base.version}):", file=sys.stderr)
        for e in errors:
            print(f"   - {e}", file=sys.stderr)
        print(
            f"   Append instead, or bump {ABI_DEFINE} (crates/capi*/src/lib.rs) for a breaking change.",
            file=sys.stderr,
        )
        return 1
    if current.version > base.version:
        print(f"✅ ABI version bumped {base.version} -> {current.version} since {ref}; no compatibility required")
    else:
        added = len(current.structs[TABLE]) - len(base.structs[TABLE])
        print(f"✅ C ABI {current.version} compatible with {ref} ({TABLE}: {added:+d} fields)")
    return 0


# --- self-test --------------------------------------------------------------------------------

FIRST = "    const char *(*version)(void);\n"
LAST = "    LmStatus (*control_flow_plan_add_meta_analysis)(LmControlFlowPlan *plan,\n"
APPENDED = "    LmStatus (*zz_appended)(void);\n"


def table_span(text: str) -> tuple[int, int]:
    end = text.index(f"}} {TABLE};")
    return text.rindex("typedef struct {", 0, end), end


def append_to_table(text: str, line: str = APPENDED) -> str:
    _, end = table_span(text)
    return text[:end] + line + text[end:]


def mutations() -> list[tuple[str, str, bool, object]]:
    """(description, file, compatible?, transform of that file's text)."""
    sub = str.replace

    def bump(t):
        return re.sub(rf"({ABI_DEFINE}) (\d+)", lambda m: f"{m.group(1)} {int(m.group(2)) + 1}", t)

    def lower(t):
        return re.sub(rf"({ABI_DEFINE}) (\d+)", lambda m: f"{m.group(1)} {int(m.group(2)) - 1}", t)

    def remove_last(t):
        start = t.index(LAST)
        return t[:start] + t[t.index(";", start) + 2 :]

    def swap(t):
        a, b = "    void (*buffer_free)(LmBuffer);\n", "    void (*str_buffer_free)(LmStrBuffer);\n"
        return sub(sub(sub(t, a, "@A@"), b, a), "@A@", b)

    return [
        ("unchanged", "types.h", True, lambda t: t),
        ("append an entry point", "types.h", True, append_to_table),
        ("append two entry points", "types.h", True, lambda t: append_to_table(append_to_table(t), "    void (*zz_second)(void);\n")),
        ("insert an entry point at the start", "types.h", False, lambda t: sub(t, FIRST, APPENDED + FIRST)),
        ("insert an entry point in the middle", "types.h", False, lambda t: sub(t, LAST, APPENDED + LAST)),
        ("swap two entry points", "types.h", False, swap),
        ("remove the last entry point", "types.h", False, remove_last),
        ("rename an entry point", "types.h", False, lambda t: sub(t, "(*version)(void)", "(*host_version)(void)")),
        ("change an entry point's signature", "types.h", False, lambda t: sub(t, "(*buffer_free)(LmBuffer)", "(*buffer_free)(LmBuffer, int)")),
        ("change an entry point's return type", "types.h", False, lambda t: sub(t, FIRST, "    char *(*version)(void);\n")),
        ("insert an entry point, ABI version bumped", "types.h", True, lambda t: bump(sub(t, FIRST, APPENDED + FIRST))),
        ("ABI version lowered", "types.h", False, lower),
        ("append to another struct", "types.h", False, lambda t: sub(t, "} LmBuffer;", "    int zz;\n} LmBuffer;")),
        ("change an enum value", "types.h", False, lambda t: sub(t, "LmStatus_Absent = 7", "LmStatus_Absent = 8")),
        ("add an enum constant", "types.h", True, lambda t: sub(t, "LmStatus_Absent = 7,", "LmStatus_Absent = 7,\n    LmStatus_ZzNew = 8,")),
        ("change a typedef", "types.h", False, lambda t: sub(t, "(*LmStrSink)(void *ctx, const char *s)", "(*LmStrSink)(const char *s)")),
        ("change a function prototype", "funcs.h", False, lambda t: sub(t, "LmBackwardFn func,", "LmBackwardFn func, int extra,")),
        ("add a function", "funcs.h", True, lambda t: sub(t, "LmStatus lm_register_plugin_backward(", "void lm_zz_new(void);\n\nLmStatus lm_register_plugin_backward(")),
    ]


def self_test() -> int:
    base_dir = REPO_ROOT / INCLUDE
    base = parse(base_dir)
    failures = 0

    # A table without `struct_size` (luna-model 0.8.0a1) -> with it: the field fills padding.
    with tempfile.TemporaryDirectory() as tmp:
        shutil.copytree(base_dir, tmp, dirs_exist_ok=True)
        types = Path(tmp, "luna_model/types.h")
        text = types.read_text()
        old = re.sub(r"(uint32_t abi_version;)\s*(?:/\*\*.*?\*/\s*)?uint32_t struct_size;", r"\1", text, flags=re.S)
        if old != text:
            types.write_text(old)
            ok = not compare(parse(Path(tmp)), base)
            failures += not ok
            print(f"{'ok  ' if ok else 'FAIL'} add struct_size in the padding after abi_version (compatible)")

    for description, file, compatible, transform in mutations():
        with tempfile.TemporaryDirectory() as tmp:
            shutil.copytree(base_dir, tmp, dirs_exist_ok=True)
            path = Path(tmp, "luna_model", file)
            text = path.read_text()
            changed = transform(text)
            if changed == text and description != "unchanged":
                print(f"FAIL {description}: the mutation did not apply")
                failures += 1
                continue
            path.write_text(changed)
            errors = compare(base, parse(Path(tmp)))
            ok = (not errors) == compatible
            failures += not ok
            verdict = "compatible" if compatible else "incompatible"
            print(f"{'ok  ' if ok else 'FAIL'} {description} ({verdict}){'' if ok else f': {errors}'}")
    print(f"{'✅' if not failures else '❌'} self-test: {failures} failure(s)")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--base-ref", help="release to compare with (default: the last release tag)")
    parser.add_argument("--self-test", action="store_true", help="verify the rules on mutated headers")
    args = parser.parse_args()
    return self_test() if args.self_test else check(args.base_ref)


if __name__ == "__main__":
    sys.exit(main())
