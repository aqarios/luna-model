import re
from pathlib import Path
from collections import defaultdict
from gen_common import LIB_ROOT, PROJECT_ROOT, AUTOGEN_HEADER, collect_exports

_ANNOTATIONS = {
    "_core": "# type: ignore[reportAttributeAccessIssue,attr-defined]",
}


def extract_existing_docstring(init_path: Path) -> str:
    if not init_path.exists():
        return ""
    text = init_path.read_text(encoding="utf-8")
    match = re.match(r'^[\s\n]*(""".*?"""|\'\'\'.*?\'\'\')', text, re.DOTALL)
    return match.group(1).strip() if match else ""


def format_imports(import_entries):
    grouped = defaultdict(list)
    for imp in import_entries:
        if " import " not in imp:
            grouped[imp].append("")
        else:
            mod, expr = imp.split(" import ", 1)
            grouped[mod.strip()].append(expr.strip())

    print(grouped)

    formatted = []
    for mod in sorted(grouped, reverse=True):
        entries = [e for e in grouped[mod] if e]
        if not entries:
            formatted.append(mod)
        elif len(entries) == 1:
            formatted.append(f"{mod} import {entries[0]}")
        else:
            formatted.append(f"{mod} import (\n  " + ",\n  ".join(entries) + "\n)")
    return formatted


def generate_module_init(
    path: Path, symbols: list[dict], module: str, is_top: bool = False
):
    lines = [AUTOGEN_HEADER]

    docstring = extract_existing_docstring(path)
    if docstring:
        lines.append(docstring)
        lines.append("")

    raw_import_lines = []
    assign_lines = []
    all_exports = []
    public_modules = set()
    rebinding_modules = set()

    for sym in symbols:
        sym_name = sym["class_name"]
        mod_base = sym["module_path"].parts[0]
        is_submodule = Path(LIB_ROOT / mod_base).is_dir() and not mod_base.startswith(
            "_"
        )

        if is_top and is_submodule:
            public_modules.add(mod_base)
            rebinding_modules.add(mod_base)
            assign_lines.append(f"{sym_name} = {mod_base}.{sym_name}")
        else:
            if is_top:
                raw_import_lines.append(
                    f"from ._core import {sym_name} as __{sym_name}"
                )
                raw_import_lines.append(f"from .{mod_base} import {sym_name}")
                assign_lines.append(
                    f"{sym_name} = __{sym_name}  # type: ignore[misc,assignment]"
                )
            else:
                raw_import_lines.append(
                    f"from .._core import {mod_base} as __{mod_base}"
                )
                raw_import_lines.append(
                    f"from .{sym['module_path'].name} import {sym_name}"
                )
                assign_lines.append(f"{sym_name} = __{mod_base}.{sym_name}")

        all_exports.append(sym_name)

        if is_top and Path(LIB_ROOT / mod_base).is_dir():
            public_modules.add(mod_base)

    if is_top:
        for sub in sorted(public_modules):
            raw_import_lines.append(f"from . import {sub}")

    lines.extend(format_imports(raw_import_lines))
    lines.append("")
    lines.extend(assign_lines)
    lines.append("")
    lines.append("__all__ = [")
    for name in sorted(all_exports + list(public_modules)):
        lines.append(f'    "{name}",')
    lines.append("]\n")

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines), encoding="utf-8")
    print(f"✅ Wrote init: {path.relative_to(PROJECT_ROOT)}")


def main():
    exports = collect_exports(LIB_ROOT)
    top_level = [e for e in exports if "__init__" in e["targets"]]
    generate_module_init(
        LIB_ROOT / "__init__.py", top_level, module="__init__", is_top=True
    )

    sub_exports = defaultdict(list)
    for e in exports:
        for t in e["targets"]:
            if t != "__init__":
                sub_exports[e["module_path"].parts[0]].append(e)

    for sub, entries in sub_exports.items():
        generate_module_init(LIB_ROOT / sub / "__init__.py", entries, module=sub)


if __name__ == "__main__":
    main()
